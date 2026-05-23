use wasmtime::{ StoreLimits, format_err };
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{
    DEFAULT_OUTGOING_BODY_BUFFER_CHUNKS, DEFAULT_OUTGOING_BODY_CHUNK_SIZE, WasiHttpCtx,
    WasiHttpView,
};

#[cfg(feature = "wasi-config")]
use wasmtime_wasi_config::WasiConfigVariables;
#[cfg(feature = "wasi-keyvalue")]
use wasmtime_wasi_keyvalue::WasiKeyValueCtx;

pub struct Host {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub http: WasiHttpCtx,
    pub http_outgoing_body_buffer_chunks: Option<usize>,
    pub http_outgoing_body_chunk_size: Option<usize>,

    #[cfg(feature = "component-model-async")]
    pub p3_http: crate::wasmtime_cli::common::DefaultP3Ctx,

    pub limits: StoreLimits,



    #[cfg(feature = "wasi-config")]
    pub wasi_config: Option<WasiConfigVariables>,

    #[cfg(feature = "wasi-keyvalue")]
    pub wasi_keyvalue: Option<WasiKeyValueCtx>,

    /*** ***/
    pub allowed_outbound_hosts:
        Option<crate::extension::outbound_networking_config::allowed_hosts::OutboundAllowedHosts>,
    pub block_url: Option<String>,
    pub block_networks:
        Option<crate::extension::outbound_networking_config::blocked_networks::BlockedNetworks>,
    /*** ***/
}

impl WasiView for Host {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

impl WasiHttpView for Host {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn outgoing_body_buffer_chunks(&mut self) -> usize {
        self.http_outgoing_body_buffer_chunks
            .unwrap_or_else(|| DEFAULT_OUTGOING_BODY_BUFFER_CHUNKS)
    }

    fn outgoing_body_chunk_size(&mut self) -> usize {
        self.http_outgoing_body_chunk_size
            .unwrap_or_else(|| DEFAULT_OUTGOING_BODY_CHUNK_SIZE)
    }

    /*** ***/
    fn send_request(
        &mut self,
        request: hyper::Request<wasmtime_wasi_http::body::HyperOutgoingBody>,
        config: wasmtime_wasi_http::types::OutgoingRequestConfig,
    ) -> wasmtime_wasi_http::HttpResult<wasmtime_wasi_http::types::HostFutureIncomingResponse> {
        let uri = request.uri();

        let allowed_outbound_hosts = self.allowed_outbound_hosts.clone().unwrap();
        let is_allowed = futures::executor::block_on(async {
            allowed_outbound_hosts
                .check_url(
                    uri.to_string().as_str(),
                    uri.scheme_str().unwrap_or("https"),
                )
                .await
                .unwrap_or(false)
        });
        if !is_allowed {
            return Err(wasmtime_wasi_http::HttpError::trap(format_err!("destination not allowed")));
        }

        Ok(wasmtime_wasi_http::types::default_send_request(
            request, config,
        ))
        /*** ***/
    }
}

#[cfg(feature = "component-model-async")]
impl wasmtime_wasi_http::p3::WasiHttpView for Host {
    fn http(&mut self) -> wasmtime_wasi_http::p3::WasiHttpCtxView<'_> {
        wasmtime_wasi_http::p3::WasiHttpCtxView {
            table: &mut self.table,
            ctx: &mut self.p3_http,
        }
    }
}