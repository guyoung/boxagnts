use axum::{
    Json,
    body::Body,
    extract::{Multipart, Query},
    http::{
        HeaderValue, StatusCode,
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    },
    response::{IntoResponse, Response},
};
use serde_json::Value;

fn error_into_response(err: boxagnts_gateway::api::file::FileError) -> Response {
    let status =
        StatusCode::from_u16(err.http_status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    (status, Json(err.to_json())).into_response()
}

pub async fn list_files(
    Query(query): Query<boxagnts_gateway::api::file::ListQuery>,
) -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::list_files(query.path.as_deref())
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn mkdir(
    Json(payload): Json<boxagnts_gateway::api::file::MkdirRequest>,
) -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::mkdir(payload)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn upload(
    Query(query): Query<boxagnts_gateway::api::file::ListQuery>,
    mut multipart: Multipart,
) -> Result<Json<Value>, Response> {
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error_into_response(boxagnts_gateway::api::file::FileError::bad_request(
            format!("Invalid multipart data: {}", e),
        ))
    })? {
        let file_name = field
            .file_name()
            .ok_or_else(|| {
                error_into_response(boxagnts_gateway::api::file::FileError::bad_request(
                    "Missing file name",
                ))
            })?
            .to_string();

        let data = field
            .bytes()
            .await
            .map_err(|e| {
                error_into_response(boxagnts_gateway::api::file::FileError::internal(format!(
                    "Failed to read uploaded file: {}",
                    e
                )))
            })?
            .to_vec();

        files.push(boxagnts_gateway::api::file::UploadFile { file_name, data });
    }

    let result = boxagnts_gateway::api::file::upload(query.path.as_deref(), files)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn delete(
    Json(payload): Json<boxagnts_gateway::api::file::DeleteRequest>,
) -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::delete(payload)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn rename(
    Json(payload): Json<boxagnts_gateway::api::file::RenameRequest>,
) -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::rename(payload)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn copy(
    Json(payload): Json<boxagnts_gateway::api::file::CopyRequest>,
) -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::copy(payload)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn move_item(
    Json(payload): Json<boxagnts_gateway::api::file::MoveRequest>,
) -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::move_item(payload)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}

pub async fn download(
    Query(query): Query<boxagnts_gateway::api::file::DownloadQuery>,
) -> Result<Response<Body>, Response> {
    let file = boxagnts_gateway::api::file::download(&query.path)
        .await
        .map_err(|e| error_into_response(e))?;

    let disposition = format!("attachment; filename=\"{}\"", file.file_name);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_str(&file.content_type).map_err(|_| {
                error_into_response(boxagnts_gateway::api::file::FileError::internal(
                    "Invalid content type",
                ))
            })?,
        )
        .header(
            CONTENT_DISPOSITION,
            HeaderValue::from_str(&disposition).map_err(|_| {
                error_into_response(boxagnts_gateway::api::file::FileError::internal(
                    "Invalid content disposition",
                ))
            })?,
        )
        .body(Body::from(file.data))
        .map_err(|e| {
            error_into_response(boxagnts_gateway::api::file::FileError::internal(format!(
                "Failed to build response: {}",
                e
            )))
        })?;

    Ok(response)
}


pub async fn list_root_sub_folders() -> Result<Json<Value>, Response> {
    let result = boxagnts_gateway::api::file::list_folders(None)
        .await
        .map_err(|e| error_into_response(e))?;
    Ok(Json(result))
}