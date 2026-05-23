mod handler;
mod host;
pub mod option;
mod output;
mod state;

use bytes::Bytes;
use futures::future::FutureExt;
use http::Uri;
use http_body_util::combinators::UnsyncBoxBody;
use http_body_util::BodyExt as _;
use std::path::PathBuf;

use std::time::Duration;
use tokio::sync::oneshot;

use wasmtime::{Result, bail, error::Context as _};
use wasmtime_wasi_http::WasiHttpView;
use wasmtime_wasi_http::handler::p2::bindings as p2;
use wasmtime_wasi_http::handler::{HandlerState, Proxy, ProxyPre};

use crate::wasmtime_cli::common::RunCommon;

pub type ResponseBody = UnsyncBoxBody<Bytes, wasmtime::Error>;
pub type Request = hyper::Request<hyper::body::Incoming>;

// ============================================================
// 辅助函数：Bytes -> UnsyncBoxBody<Bytes, hyper::Error>
// 通过 boxed_unsync() 擦除私有 MapErr 类型
// 满足 new_incoming_request 的 B::Error: Into<ErrorCode> 约束
// ============================================================
fn full_bytes_body(bytes: Bytes) -> UnsyncBoxBody<Bytes, hyper::Error> {
    http_body_util::Full::new(bytes)
        .map_err(|_: std::convert::Infallible| -> hyper::Error {
            unreachable!("Full<Bytes> never errors")
        })
        .boxed_unsync()
}

// ============================================================
// 泛型 handle_request
// ============================================================
pub async fn handle_request<B>(
    base_url: &str,
    run_option: option::RunOption,
    req: hyper::Request<B>,
) -> Result<hyper::Response<ResponseBody>>
where
    B: http_body::Body<Data = Bytes> + Send + Unpin + 'static,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    let mut run_common = crate_run_common(run_option.clone())?;

    let mut wasm_file = run_option.wasm_file;
    let wasm_file_md5 = check_file(&wasm_file)?;
    let wasm_cache_file = format!("{}.cwasm", wasm_file_md5);

    if let Some(dir) = run_option.wasm_cache_dir {
        if let Ok(cache_file) =
            crate::compiler::process(&mut run_common, &wasm_file, &dir, &wasm_cache_file)
        {
            run_common.allow_precompiled = true;
            wasm_file = cache_file.to_string_lossy().to_string();
        }
    }

    let component_path = PathBuf::from(wasm_file);

    let handler = handler::crate_proxy_handler(&mut run_common, component_path, None, None)?;

    let req_id = handler.next_req_id();

    // println!(
    //     "Request {req_id} handling {} to {}",
    //     req.method(),
    //     req.uri()
    // );

    let mut req = req;
    
    *req.uri_mut() = strip_uri_prefix(&req.uri(), base_url)?;

    // println!(
    //     "Request {req_id} handling {} to {}",
    //     req.method(),
    //     req.uri()
    // );

    type P2Response = Result<
        hyper::Response<wasmtime_wasi_http::body::HyperOutgoingBody>,
        p2::http::types::ErrorCode,
    >;
    type P3Response = hyper::Response<ResponseBody>;

    enum Sender {
        P2(oneshot::Sender<P2Response>),
        P3(oneshot::Sender<P3Response>),
    }

    enum Receiver {
        P2(oneshot::Receiver<P2Response>),
        P3(oneshot::Receiver<P3Response>),
    }

    let (tx, rx) = match handler.instance_pre() {
        ProxyPre::P2(_) => {
            let (tx, rx) = oneshot::channel();
            (Sender::P2(tx), Receiver::P2(rx))
        }
        ProxyPre::P3(_) => {
            let (tx, rx) = oneshot::channel();
            (Sender::P3(tx), Receiver::P3(rx))
        }
    };

    // 统一收集 body 为 Bytes
    let (parts, body) = req.into_parts();
    let body_bytes: Bytes = body
        .collect()
        .await
        .map_err(|e| wasmtime::Error::msg(e.to_string()))?
        .to_bytes();

    handler.spawn(
        if handler.state().max_instance_reuse_count() == 1 {
            Some(req_id)
        } else {
            None
        },
        Box::new(move |store, proxy| {
            Box::pin(
                async move {
                    match proxy {
                        // ----------------------------------------------------
                        // P2 分支
                        // UnsyncBoxBody<Bytes, hyper::Error>
                        //   -> hyper::Error: Into<ErrorCode> ✅
                        // ----------------------------------------------------
                        Proxy::P2(proxy) => {
                            let Sender::P2(tx) = tx else { unreachable!() };

                            let rebuilt_req = hyper::Request::from_parts(
                                parts,
                                // ✅ 类型已擦除，Error = hyper::Error
                                full_bytes_body(body_bytes),
                            );

                            let (req_resource, out) = store.with(move |mut store| {
                                let req = store.data_mut().new_incoming_request(
                                    p2::http::types::Scheme::Http,
                                    rebuilt_req,
                                )?;
                                let out = store.data_mut().new_response_outparam(tx)?;
                                wasmtime::error::Ok((req, out))
                            })?;

                            proxy
                                .wasi_http_incoming_handler()
                                .call_handle(store, req_resource, out)
                                .await
                        }

                        // ----------------------------------------------------
                        // P3 分支
                        // ----------------------------------------------------
                        Proxy::P3(proxy) => {
                            use wasmtime_wasi_http::p3::bindings::http::types::{
                                ErrorCode, Request,
                            };

                            let Sender::P3(tx) = tx else { unreachable!() };

                            // P3 map_err 转为 ErrorCode
                            let mapped_body = http_body_util::Full::new(body_bytes).map_err(
                                |_: std::convert::Infallible| ErrorCode::InternalError(None),
                            );

                            let rebuilt_req = http::Request::from_parts(parts, mapped_body);

                            let (request, request_io_result) = Request::from_http(rebuilt_req);

                            let (res, task) = proxy.handle(store, request).await??;

                            let res = store
                                .with(|mut store| res.into_http(&mut store, request_io_result))?;

                            _ = tx.send(res.map(|body| body.map_err(|e| e.into()).boxed_unsync()));

                            task.block(store).await;
                            Ok(())
                        }
                    }
                }
                .map(move |result| {
                    if let Err(error) = result {
                        eprintln!("[{req_id}] :: {error:?}");
                    }
                }),
            )
        }),
    );

    Ok(match rx {
        Receiver::P2(rx) => rx
            .await
            .context("guest never invoked `response-outparam::set` method")?
            .map_err(wasmtime::Error::from)?
            .map(|body| body.map_err(|e| e.into()).boxed_unsync()),

        Receiver::P3(rx) => rx.await?,
    })
}

fn crate_run_common(run_option: option::RunOption) -> Result<RunCommon> {
    let mut dirs: Vec<(String, String)> = Vec::new();
    if let Some(dir) = run_option.work_dir {
        dirs.push((dir, "/".to_string()));
    }
    if let Some(map_dirs) = run_option.map_dirs {
        for (key, value) in map_dirs {
            dirs.push((key, value));
        }
    }

    let mut vars: Vec<(String, Option<String>)> = Vec::new();
    if let Some(env_vars) = run_option.env_vars {
        for (key, value) in env_vars {
            vars.push((key, value));
        }
    }

    let mut run_common = RunCommon {
        common: Default::default(),
        allow_precompiled: false,
        profile: None,
        dirs,
        vars,
        allowed_outbound_hosts: run_option.allowed_outbound_hosts.unwrap_or(Vec::new()),
        block_url: run_option.block_url.clone(),
        block_networks: run_option.block_networks.unwrap_or(Vec::new()),
    };
    run_common.common.wasm.component_model = Some(true);

    // C++ exception handle: wasm.exceptions
    run_common.common.wasm.exceptions = Some(true);

    run_common.common.wasi.cli = Some(true);
    run_common.common.wasi.http = Some(true);
    run_common.common.wasi.inherit_network = Some(true);
    run_common.common.wasi.allow_ip_name_lookup = Some(true);
    run_common.common.wasi.tcp = Some(true);
    run_common.common.wasi.udp = Some(true);
    run_common.common.wasi.config = Some(true);
    run_common.common.wasi.keyvalue = Some(true);

    let mut vars: Vec<wasmtime_cli_flags::KeyValuePair> = Vec::new();
    if let Some(config_vars) = run_option.config_vars {
        for (k, v) in config_vars {
            vars.push(wasmtime_cli_flags::KeyValuePair { key: k, value: v });
        }
    }

    run_common.common.wasi.config_var = vars;

    let mut vars: Vec<wasmtime_cli_flags::KeyValuePair> = Vec::new();
    if let Some(keyvalue_vars) = run_option.keyvalue_vars {
        for (k, v) in keyvalue_vars {
            vars.push(wasmtime_cli_flags::KeyValuePair { key: k, value: v });
        }
    }
    run_common.common.wasi.keyvalue_in_memory_data = vars;

    let timeout = if let Some(timeout) = run_option.wasm_timeout {
        Some(Duration::from_secs(timeout as u64))
    } else {
        None
    };

    run_common.common.wasm.timeout = timeout;
    run_common.common.wasm.max_memory_size = run_option.wasm_max_memory_size.map(|v| v as usize);
    run_common.common.wasm.max_wasm_stack = run_option.wasm_max_wasm_stack.map(|v| v as usize);
    run_common.common.wasm.fuel = run_option.wasm_fuel.map(|v| v as u64);

    Ok(run_common)
}

fn check_file(path: &str) -> Result<String> {
    use std::io::Read;

    let file = std::fs::File::open(path);

    if let Err(e) = file {
        bail!("Error opening wasm file: {e}");
    }

    let mut file = file?;

    let mut context = md5::Context::new();
    let mut buffer = [0u8; 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        context.consume(&buffer[..n]);
    }

    let digest = context.finalize();

    Ok(format!("{:x}", digest))
}

/// 去除 URI 路径前缀
/// 例："/sites/site1/api/hello" -> "/api/hello"
/// 例："/sites/site1"           -> "/"
fn strip_uri_prefix(uri: &Uri, prefix: &str) -> Result<Uri> {
    let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");

    // 去除前缀
    let stripped = if path_and_query.starts_with(prefix) {
        let rest = &path_and_query[prefix.len()..];
        // 确保以 "/" 开头，若去除后为空则返回 "/"
        if rest.is_empty() {
            "/"
        } else if rest.starts_with('/') {
            rest
        } else {
            // 前缀不是完整路径段，拼回 "/"
            return Err(wasmtime::Error::msg(format!(
                "prefix '{prefix}' is not a complete path segment"
            )));
        }
    } else {
        // 无匹配前缀，保持原样
        path_and_query
    };

    // 重建 URI，保留 scheme 和 authority
    let mut uri_builder = Uri::builder();

    if let Some(authority) = uri.authority() {
        uri_builder = uri_builder
            .scheme(uri.scheme().cloned().unwrap_or(http::uri::Scheme::HTTP))
            .authority(authority.as_str());
    }

    let new_uri = uri_builder
        .path_and_query(stripped)
        .build()
        .map_err(|e| wasmtime::Error::msg(e.to_string()))?;

    Ok(new_uri)
}
