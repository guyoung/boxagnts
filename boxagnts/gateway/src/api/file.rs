use std::path::PathBuf;
use std::path::Path;
use std::path::Component;
use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use serde_json::{json, Value};
use tokio::fs;


#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    BadRequest,
    NotFound,
    Conflict,
    Internal,
}

impl ErrorCode {
    pub fn as_u16(self) -> u16 {
        match self {
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::Internal => 500,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::BadRequest => "bad_request",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::Internal => "internal_error",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileError {
    pub code: ErrorCode,
    pub message: String,
}

impl FileError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::BadRequest, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Conflict, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Internal, message)
    }

    pub fn http_status_code(&self) -> u16 {
        self.code.as_u16()
    }

    pub fn error_name(&self) -> &'static str {
        self.code.as_str()
    }

    pub fn to_json(&self) -> Value {
        json!({
            "code": self.http_status_code(),
            "error": self.error_name(),
            "message": self.message,
            "data": null
        })
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: {}", self.http_status_code(), self.error_name(), self.message)
    }
}

impl Error for FileError {}


#[derive(Debug, Serialize)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MkdirRequest {
    pub path: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameRequest {
    pub path: String,
    pub new_name: String,
}

#[derive(Debug, Clone)]
pub struct UploadFile {
    pub file_name: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct DownloadFile {
    pub file_name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadQuery {
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct FolderItem {
    pub name: String,
    pub path: String,
}


fn success(data: Value) -> Value {
    json!({
        "code": 0,
        "message": "success",
        "data": data
    })
}

fn validate_name(name: &str) -> Result<(), FileError> {
    if name.trim().is_empty() {
        return Err(FileError::bad_request("Name cannot be empty"));
    }

    if name.contains('/') || name.contains('\\') {
        return Err(FileError::bad_request("Name cannot contain path separators"));
    }

    if name == "." || name == ".." {
        return Err(FileError::bad_request("Invalid name"));
    }

    Ok(())
}

fn sanitize_relative_path(input: Option<&str>) -> Result<PathBuf, FileError> {
    let raw = input.unwrap_or("").trim();

    if raw.is_empty() {
        return Ok(PathBuf::new());
    }

    let path = Path::new(raw);

    if path.is_absolute() {
        return Err(FileError::bad_request("Absolute path is not allowed"));
    }

    let mut clean = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(FileError::bad_request("Parent path is not allowed"));
            }
            _ => {
                return Err(FileError::bad_request("Invalid path"));
            }
        }
    }

    Ok(clean)
}

fn get_root_dir() -> Result<PathBuf, FileError> {
    let workspace_dir = futures::executor::block_on(async {
        boxagnts_workspace::path::get_workspace_dir().await
    });

    let root_dir = workspace_dir.join("root");

    if !root_dir.exists() {
        std::fs::create_dir_all(&root_dir)
            .map_err(|e| FileError::internal(format!("{:?}", e)))?;
    }

    Ok(root_dir)

}

fn resolve_path(relative: Option<&str>) -> Result<PathBuf, FileError> {
    let root_dir = get_root_dir()?;
    let clean = sanitize_relative_path(relative)?;
    Ok(root_dir.join(clean))
}

fn format_modified_time(metadata: &std::fs::Metadata) -> Option<String> {
    let modified = metadata.modified().ok()?;
    let datetime: DateTime<Local> = modified.into();
    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn guess_content_type(path: &Path) -> String {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("txt") => "text/plain",
        Some("json") => "application/json",
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("pdf") => "application/pdf",
        Some("zip") => "application/zip",
        _ => "application/octet-stream",
    }
        .to_string()
}

pub async fn list_files(
    relative_path: Option<&str>,
) -> Result<Value, FileError> {
    let root_dir = get_root_dir()?;
    let dir_path = resolve_path(relative_path)?;

    let metadata = fs::metadata(&dir_path)
        .await
        .map_err(|_| FileError::not_found("Directory not found"))?;

    if !metadata.is_dir() {
        return Err(FileError::bad_request("Target path is not a directory"));
    }

    let mut entries = fs::read_dir(&dir_path)
        .await
        .map_err(|e| FileError::internal(format!("Failed to read directory: {}", e)))?;

    let mut items = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| FileError::internal(format!("Failed to read directory entry: {}", e)))?
    {
        let path = entry.path();

        let entry_metadata = entry
            .metadata()
            .await
            .map_err(|e| FileError::internal(format!("Failed to read metadata: {}", e)))?;

        let relative = path
            .strip_prefix(&root_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");

        let modified = std::fs::metadata(&path)
            .ok()
            .and_then(|m| format_modified_time(&m));

        items.push(FileItem {
            name: entry.file_name().to_string_lossy().to_string(),
            path: relative,
            is_dir: entry_metadata.is_dir(),
            size: if entry_metadata.is_file() {
                entry_metadata.len()
            } else {
                0
            },
            modified,
        });
    }

    items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(success(json!({
        "path": relative_path.unwrap_or(""),
        "items": items
    })))
}

pub async fn mkdir(
    payload: MkdirRequest,
) -> Result<Value, FileError> {
    validate_name(&payload.name)?;

    let root_dir = get_root_dir()?;
    let parent_dir = resolve_path(payload.path.as_deref())?;

    let parent_metadata = fs::metadata(&parent_dir)
        .await
        .map_err(|_| FileError::not_found("Parent directory not found"))?;

    if !parent_metadata.is_dir() {
        return Err(FileError::bad_request("Parent path is not a directory"));
    }

    let target_dir = parent_dir.join(&payload.name);

    if fs::metadata(&target_dir).await.is_ok() {
        return Err(FileError::conflict("Target already exists"));
    }

    fs::create_dir(&target_dir)
        .await
        .map_err(|e| FileError::internal(format!("Failed to create directory: {}", e)))?;

    Ok(success(json!({
        "path": target_dir
            .strip_prefix(&root_dir)
            .unwrap_or(&target_dir)
            .to_string_lossy()
            .replace('\\', "/"),
        "name": payload.name
    })))
}

pub async fn upload(
    relative_path: Option<&str>,
    files: Vec<UploadFile>,
) -> Result<Value, FileError> {
    if files.is_empty() {
        return Err(FileError::bad_request("No files provided"));
    }

    let root_dir = get_root_dir()?;
    let target_dir = resolve_path(relative_path)?;

    let metadata = fs::metadata(&target_dir)
        .await
        .map_err(|_| FileError::not_found("Target directory not found"))?;

    if !metadata.is_dir() {
        return Err(FileError::bad_request("Target path is not a directory"));
    }

    let mut saved_files = Vec::new();

    for file in files {
        validate_name(&file.file_name)?;

        let file_path = target_dir.join(&file.file_name);

        /*
        if fs::metadata(&file_path).await.is_ok() {
            return Err(FileError::conflict(format!(
                "File already exists: {}",
                file.file_name
            )));
        }
         */

        fs::write(&file_path, &file.data)
            .await
            .map_err(|e| FileError::internal(format!("Failed to save file: {}", e)))?;

        saved_files.push(json!({
            "name": file.file_name,
            "path": file_path
                .strip_prefix(&root_dir)
                .unwrap_or(&file_path)
                .to_string_lossy()
                .replace('\\', "/"),
            "size": file.data.len()
        }));
    }

    Ok(success(json!({
        "files": saved_files
    })))
}

pub async fn delete(
    payload: DeleteRequest,
) -> Result<Value, FileError> {
    let normalized = sanitize_relative_path(Some(&payload.path))?;
    if normalized.as_os_str().is_empty() {
        return Err(FileError::bad_request("Root directory cannot be deleted"));
    }

    let root_dir = get_root_dir()?;
    let target_path = root_dir.join(&normalized);

    let metadata = fs::metadata(&target_path)
        .await
        .map_err(|_| FileError::not_found("Target not found"))?;

    if metadata.is_dir() {
        fs::remove_dir_all(&target_path)
            .await
            .map_err(|e| FileError::internal(format!("Failed to delete directory: {}", e)))?;
    } else {
        fs::remove_file(&target_path)
            .await
            .map_err(|e| FileError::internal(format!("Failed to delete file: {}", e)))?;
    }

    Ok(success(json!({
        "path": normalized.to_string_lossy().replace('\\', "/")
    })))
}

pub async fn rename(
    payload: RenameRequest,
) -> Result<Value, FileError> {
    validate_name(&payload.new_name)?;

    let normalized = sanitize_relative_path(Some(&payload.path))?;
    if normalized.as_os_str().is_empty() {
        return Err(FileError::bad_request("Root directory cannot be renamed"));
    }

    let root_dir = get_root_dir()?;
    let old_path = root_dir.join(&normalized);

    let metadata = fs::metadata(&old_path)
        .await
        .map_err(|_| FileError::not_found("Target not found"))?;

    let parent = old_path
        .parent()
        .ok_or_else(|| FileError::bad_request("Invalid target path"))?;

    let new_path = parent.join(&payload.new_name);

    if fs::metadata(&new_path).await.is_ok() {
        return Err(FileError::conflict("Target name already exists"));
    }

    fs::rename(&old_path, &new_path)
        .await
        .map_err(|e| FileError::internal(format!("Failed to rename target: {}", e)))?;

    let new_relative = new_path
        .strip_prefix(&root_dir)
        .unwrap_or(&new_path)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(success(json!({
        "old_path": normalized.to_string_lossy().replace('\\', "/"),
        "new_path": new_relative,
        "is_dir": metadata.is_dir()
    })))
}

pub async fn download(
    relative_path: &str,
) -> Result<DownloadFile, FileError> {
    let normalized = sanitize_relative_path(Some(relative_path))?;
    if normalized.as_os_str().is_empty() {
        return Err(FileError::bad_request("A file path is required"));
    }

    let root_dir = get_root_dir()?;
    let file_path = root_dir.join(&normalized);

    let metadata = fs::metadata(&file_path)
        .await
        .map_err(|_| FileError::not_found("File not found"))?;

    if !metadata.is_file() {
        return Err(FileError::bad_request("Target path is not a file"));
    }

    let data = fs::read(&file_path)
        .await
        .map_err(|e| FileError::internal(format!("Failed to read file: {}", e)))?;

    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| FileError::internal("Invalid file name"))?
        .to_string();


    Ok(DownloadFile {
        file_name,
        content_type: guess_content_type(&file_path),
        data,
    })
}



pub async fn list_sub_folders(
    relative_path: Option<&str>,
) -> Result<Value, FileError> {
    let root_dir = get_root_dir()?;
    let dir_path = resolve_path(relative_path)?;

    let metadata = fs::metadata(&dir_path)
        .await
        .map_err(|_| FileError::not_found("Directory not found"))?;

    if !metadata.is_dir() {
        return Err(FileError::bad_request("Target path is not a directory"));
    }

    let mut folders = Vec::new();
    let mut stack = vec![dir_path];

    while let Some(current_dir) = stack.pop() {
        let mut entries = fs::read_dir(&current_dir)
            .await
            .map_err(|e| FileError::internal(format!("Failed to read directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| FileError::internal(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();

            let metadata = entry
                .metadata()
                .await
                .map_err(|e| FileError::internal(format!("Failed to read metadata: {}", e)))?;

            if metadata.is_dir() {
                let relative = path
                    .strip_prefix(&root_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");

                folders.push(FolderItem {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: relative,
                });

                stack.push(path);
            }
        }
    }

    folders.sort_by(|a, b| a.path.to_lowercase().cmp(&b.path.to_lowercase()));

    Ok(success(json!({
        "path": relative_path.unwrap_or(""),
        "folders": folders
    })))
}