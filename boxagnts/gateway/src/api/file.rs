use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use std::{error::Error, fmt};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::fs;

use crate::api::fs_events::{begin_ignore_paths, finish_ignore_operation};

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

#[derive(Debug, Deserialize)]
pub struct CopyRequest {
    pub path: String,
    pub target_path: Option<String>,
    pub new_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub path: String,
    pub target_path: Option<String>,
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
    let workspace_dir =
        futures::executor::block_on(async { boxagnts_workspace::path::get_workspace_dir().await });

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

fn relative_string(root_dir: &Path, path: &Path) -> String {
    path.strip_prefix(root_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn paths_with_parents(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut result = Vec::new();

    for path in paths {
        result.push(path.clone());
        if let Some(parent) = path.parent() {
            result.push(parent.to_path_buf());
        }
    }

    result.sort();
    result.dedup();
    result
}

fn copy_recursively_sync(src: &Path, dst: &Path) -> Result<(), FileError> {
    let metadata = std::fs::metadata(src).map_err(|_| FileError::not_found("Source not found"))?;

    if metadata.is_file() {
        std::fs::copy(src, dst)
            .map_err(|e| FileError::internal(format!("Failed to copy file: {}", e)))?;
        return Ok(());
    }

    std::fs::create_dir(dst)
        .map_err(|e| FileError::internal(format!("Failed to create target directory: {}", e)))?;

    let entries = std::fs::read_dir(src)
        .map_err(|e| FileError::internal(format!("Failed to read source directory: {}", e)))?;

    for entry in entries {
        let entry = entry
            .map_err(|e| FileError::internal(format!("Failed to read directory entry: {}", e)))?;

        let child_src = entry.path();
        let child_dst = dst.join(entry.file_name());

        let child_metadata = entry
            .metadata()
            .map_err(|e| FileError::internal(format!("Failed to read metadata: {}", e)))?;

        if child_metadata.is_dir() {
            copy_recursively_sync(&child_src, &child_dst)?;
        } else {
            std::fs::copy(&child_src, &child_dst)
                .map_err(|e| FileError::internal(format!("Failed to copy file: {}", e)))?;
        }
    }

    Ok(())
}

pub async fn list_files(relative_path: Option<&str>) -> Result<Value, FileError> {
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

        let modified = std::fs::metadata(&path)
            .ok()
            .and_then(|m| format_modified_time(&m));

        items.push(FileItem {
            name: entry.file_name().to_string_lossy().to_string(),
            path: relative_string(&root_dir, &path),
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
        "path": sanitize_relative_path(relative_path)?
            .to_string_lossy()
            .replace('\\', "/"),
        "items": items
    })))
}

pub async fn list_folders(relative_path: Option<&str>) -> Result<Value, FileError> {
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
        let entry_metadata = entry
            .metadata()
            .await
            .map_err(|e| FileError::internal(format!("Failed to read metadata: {}", e)))?;

        if entry_metadata.is_dir() {
            let path = entry.path();
            items.push(FolderItem {
                name: entry.file_name().to_string_lossy().to_string(),
                path: relative_string(&root_dir, &path),
            });
        }
    }

    items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(success(json!({
        "path": sanitize_relative_path(relative_path)?
            .to_string_lossy()
            .replace('\\', "/"),
        "items": items
    })))
}

pub async fn mkdir(payload: MkdirRequest) -> Result<Value, FileError> {
    validate_name(&payload.name)?;

    let parent_dir = resolve_path(payload.path.as_deref())?;
    let metadata = fs::metadata(&parent_dir)
        .await
        .map_err(|_| FileError::not_found("Parent directory not found"))?;

    if !metadata.is_dir() {
        return Err(FileError::bad_request("Target path is not a directory"));
    }

    let new_dir = parent_dir.join(&payload.name);

    if fs::metadata(&new_dir).await.is_ok() {
        return Err(FileError::conflict("Directory already exists"));
    }

    let op = begin_ignore_paths(
        paths_with_parents(vec![new_dir.clone()]),
        Duration::from_secs(4),
    )
        .await;

    let result = fs::create_dir(&new_dir)
        .await
        .map_err(|e| FileError::internal(format!("Failed to create directory: {}", e)));

    if result.is_ok() {
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    finish_ignore_operation(op).await;

    result?;

    let root_dir = get_root_dir()?;
    Ok(success(json!({
        "name": payload.name,
        "path": relative_string(&root_dir, &new_dir)
    })))
}

pub async fn upload(relative_path: Option<&str>, files: Vec<UploadFile>) -> Result<Value, FileError> {
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

    let all_paths = paths_with_parents(
        files.iter()
            .map(|f| target_dir.join(&f.file_name))
            .collect::<Vec<_>>(),
    );

    let op = begin_ignore_paths(all_paths, Duration::from_secs(5)).await;

    let mut saved_files = Vec::new();

    for file in files {
        validate_name(&file.file_name)?;

        let file_path = target_dir.join(&file.file_name);

        fs::write(&file_path, &file.data)
            .await
            .map_err(|e| FileError::internal(format!("Failed to save file: {}", e)))?;

        saved_files.push(json!({
            "name": file.file_name,
            "path": relative_string(&root_dir, &file_path),
            "size": file.data.len()
        }));
    }

    tokio::time::sleep(Duration::from_millis(500)).await;
    finish_ignore_operation(op).await;

    Ok(success(json!({
        "files": saved_files
    })))
}

pub async fn download(relative_path: &str) -> Result<DownloadFile, FileError> {
    let normalized = sanitize_relative_path(Some(relative_path))?;
    if normalized.as_os_str().is_empty() {
        return Err(FileError::bad_request("Root directory cannot be downloaded"));
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
        .and_then(|n| n.to_str())
        .unwrap_or("download")
        .to_string();

    Ok(DownloadFile {
        file_name,
        content_type: guess_content_type(&file_path),
        data,
    })
}

pub async fn delete(payload: DeleteRequest) -> Result<Value, FileError> {
    let normalized = sanitize_relative_path(Some(&payload.path))?;
    if normalized.as_os_str().is_empty() {
        return Err(FileError::bad_request("Root directory cannot be deleted"));
    }

    let root_dir = get_root_dir()?;
    let target_path = root_dir.join(&normalized);

    let metadata = fs::metadata(&target_path)
        .await
        .map_err(|_| FileError::not_found("Target not found"))?;

    let op = begin_ignore_paths(
        paths_with_parents(vec![target_path.clone()]),
        Duration::from_secs(8),
    )
        .await;

    let result = if metadata.is_dir() {
        fs::remove_dir_all(&target_path)
            .await
            .map_err(|e| FileError::internal(format!("Failed to delete directory: {}", e)))
    } else {
        fs::remove_file(&target_path)
            .await
            .map_err(|e| FileError::internal(format!("Failed to delete file: {}", e)))
    };

    if result.is_ok() {
        tokio::time::sleep(Duration::from_millis(800)).await;
    }

    finish_ignore_operation(op).await;

    result?;

    Ok(success(json!({
        "path": relative_string(&root_dir, &target_path)
    })))
}

pub async fn rename(payload: RenameRequest) -> Result<Value, FileError> {
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

    let op = begin_ignore_paths(
        paths_with_parents(vec![old_path.clone(), new_path.clone()]),
        Duration::from_secs(6),
    )
        .await;

    let result = fs::rename(&old_path, &new_path)
        .await
        .map_err(|e| FileError::internal(format!("Failed to rename target: {}", e)));

    if result.is_ok() {
        tokio::time::sleep(Duration::from_millis(600)).await;
    }

    finish_ignore_operation(op).await;

    result?;

    Ok(success(json!({
        "old_path": relative_string(&root_dir, &old_path),
        "new_path": relative_string(&root_dir, &new_path),
        "name": payload.new_name,
        "is_dir": metadata.is_dir()
    })))
}

pub async fn copy(payload: CopyRequest) -> Result<Value, FileError> {
    validate_name(&payload.new_name)?;

    let normalized_src = sanitize_relative_path(Some(&payload.path))?;
    if normalized_src.as_os_str().is_empty() {
        return Err(FileError::bad_request("Root directory cannot be copied"));
    }

    let root_dir = get_root_dir()?;
    let src_path = root_dir.join(&normalized_src);

    let src_metadata = fs::metadata(&src_path)
        .await
        .map_err(|_| FileError::not_found("Source not found"))?;

    let target_parent = resolve_path(payload.target_path.as_deref())?;
    let target_parent_metadata = fs::metadata(&target_parent)
        .await
        .map_err(|_| FileError::not_found("Target directory not found"))?;

    if !target_parent_metadata.is_dir() {
        return Err(FileError::bad_request("Target path is not a directory"));
    }

    let dst_path = target_parent.join(&payload.new_name);

    if fs::metadata(&dst_path).await.is_ok() {
        return Err(FileError::conflict("Target name already exists"));
    }

    if src_metadata.is_dir() && dst_path.starts_with(&src_path) {
        return Err(FileError::bad_request(
            "Cannot copy a directory into its own subdirectory",
        ));
    }

    let op = begin_ignore_paths(
        paths_with_parents(vec![dst_path.clone()]),
        Duration::from_secs(12),
    )
        .await;

    let result = if src_metadata.is_dir() {
        copy_recursively_sync(&src_path, &dst_path)
    } else {
        std::fs::copy(&src_path, &dst_path)
            .map(|_| ())
            .map_err(|e| FileError::internal(format!("Failed to copy file: {}", e)))
    };

    if result.is_ok() {
        tokio::time::sleep(Duration::from_millis(1200)).await;
    }

    finish_ignore_operation(op).await;

    result?;

    Ok(success(json!({
        "source_path": relative_string(&root_dir, &src_path),
        "target_path": relative_string(&root_dir, &dst_path),
        "name": payload.new_name,
        "is_dir": src_metadata.is_dir()
    })))
}

pub async fn move_item(payload: MoveRequest) -> Result<Value, FileError> {
    validate_name(&payload.new_name)?;

    let normalized_src = sanitize_relative_path(Some(&payload.path))?;
    if normalized_src.as_os_str().is_empty() {
        return Err(FileError::bad_request("Root directory cannot be moved"));
    }

    let root_dir = get_root_dir()?;
    let src_path = root_dir.join(&normalized_src);

    let src_metadata = fs::metadata(&src_path)
        .await
        .map_err(|_| FileError::not_found("Source not found"))?;

    let target_parent = resolve_path(payload.target_path.as_deref())?;
    let target_parent_metadata = fs::metadata(&target_parent)
        .await
        .map_err(|_| FileError::not_found("Target directory not found"))?;

    if !target_parent_metadata.is_dir() {
        return Err(FileError::bad_request("Target path is not a directory"));
    }

    let dst_path = target_parent.join(&payload.new_name);

    if src_path == dst_path {
        return Err(FileError::bad_request("Source and destination are the same"));
    }

    if fs::metadata(&dst_path).await.is_ok() {
        return Err(FileError::conflict("Target name already exists"));
    }

    if src_metadata.is_dir() && dst_path.starts_with(&src_path) {
        return Err(FileError::bad_request(
            "Cannot move a directory into its own subdirectory",
        ));
    }

    let op = begin_ignore_paths(
        paths_with_parents(vec![src_path.clone(), dst_path.clone()]),
        Duration::from_secs(8),
    )
        .await;

    let result = fs::rename(&src_path, &dst_path)
        .await
        .map_err(|e| FileError::internal(format!("Failed to move target: {}", e)));

    if result.is_ok() {
        tokio::time::sleep(Duration::from_millis(800)).await;
    }

    finish_ignore_operation(op).await;

    result?;

    Ok(success(json!({
        "source_path": relative_string(&root_dir, &src_path),
        "target_path": relative_string(&root_dir, &dst_path),
        "name": payload.new_name,
        "is_dir": src_metadata.is_dir()
    })))
}
