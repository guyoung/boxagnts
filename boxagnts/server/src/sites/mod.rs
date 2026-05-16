use std::convert::Infallible;

use axum::body::Body;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use tower::service_fn;

pub fn make_dynamic_service() -> impl tower::Service<
    Request<Body>,
    Response = Response<boxagnts_wasm_sandbox::wasmtime_http::ResponseBody>,
    Error = Infallible,
    Future = impl Send,
> + Clone {
    service_fn(move |req: Request<Body>| async move {
        let route = match parse_route_path(req.uri().path()) {
            Some(v) => v,
            None => {
                return Ok::<_, Infallible>(text_response(StatusCode::BAD_REQUEST, "Bad Request"));
            }
        };

        let site = route.site.clone();
        let _path = route.path.clone();

        let app_extensions_dir = boxagnts_workspace::path::get_app_extensions_dir().await;
        let app_cache_dir = boxagnts_workspace::path::get_app_cache_dir().await;
        let workspace_dir = boxagnts_workspace::path::get_workspace_dir().await;

        let site = boxagnts_gateway::site::store::find_site_name(&site).await;

        if let Err(e) = site {
            return Ok::<_, Infallible>(text_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }

        let site = site.unwrap();

        if site.is_none() {
            return Ok::<_, Infallible>(text_response(StatusCode::NOT_FOUND, "Not found"));
        }

        let site = site.unwrap();

        if !site.enabled {
            return Ok::<_, Infallible>(text_response(StatusCode::NOT_FOUND, "Site is disabled"));
        }

        let wasm_file = format!(
            "{}",
            app_extensions_dir
                .join("services/")
                .join(site.component)
                .display()
        );

        let work_dir = workspace_dir.join("root").join(site.path.clone());

        if !work_dir.exists() {
            return Ok::<_, Infallible>(text_response(StatusCode::NOT_FOUND, "Not found"));
        }

        let mut run_option = boxagnts_wasm_sandbox::wasmtime_http::option::RunOption::default();
        run_option.work_dir = Some(format!("{}", work_dir.display()));
        run_option.wasm_cache_dir = Some(format!("{}", app_cache_dir.display()));
        run_option.wasm_file = wasm_file.clone();

        let base_url = format!("/sites/{}", site.name);

        let resp = boxagnts_wasm_sandbox::wasmtime_http::handle_request(&base_url, run_option, req)
            .await
            .unwrap_or_else(|err| {
                text_response(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            });

        Ok::<_, Infallible>(resp)
    })
}

fn text_response(
    status: StatusCode,
    text: impl Into<Bytes>,
) -> Response<boxagnts_wasm_sandbox::wasmtime_http::ResponseBody> {
    Response::builder()
        .status(status)
        .body(boxed_body(text))
        .unwrap()
}

fn boxed_body(data: impl Into<Bytes>) -> boxagnts_wasm_sandbox::wasmtime_http::ResponseBody {
    Full::new(data.into())
        .map_err(|never| match never {})
        .boxed_unsync()
}

#[derive(Debug, Clone)]
struct RouteMatch {
    site: String,
    path: String,
}

fn parse_route_path(uri_path: &str) -> Option<RouteMatch> {
    let trimmed = uri_path.trim_start_matches('/');
    let mut parts = trimmed.splitn(3, '/');

    let _ = parts.next()?.to_string();
    let site = parts.next()?.to_string();
    let path = parts.next().unwrap_or("").to_string();

    if site.is_empty() {
        return None;
    }

    Some(RouteMatch { site, path })
}
