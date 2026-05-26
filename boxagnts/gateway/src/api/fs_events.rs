use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

use chrono::Local;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::{broadcast, mpsc, RwLock};

static OP_ID: AtomicU64 = AtomicU64::new(1);

static IGNORE_OPERATIONS: Lazy<RwLock<HashMap<u64, IgnoreOperation>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

static FS_EVENT_BROADCASTER: Lazy<broadcast::Sender<FsChangeEvent>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(1024);
    tx
});

#[derive(Debug, Clone, Serialize)]
pub struct FsChangeEvent {
    pub kind: String,
    pub path: String,
    pub old_path: Option<String>,
    pub source: String,
    pub time: String,
}

#[derive(Debug, Clone)]
struct IgnoreOperation {
    pub paths: Vec<PathBuf>,
    pub expires_at: Instant,
}

#[derive(Debug, Clone, Copy)]
pub struct IgnoreOperationId(pub u64);

pub fn broadcaster() -> broadcast::Sender<FsChangeEvent> {
    FS_EVENT_BROADCASTER.clone()
}

pub fn subscribe() -> broadcast::Receiver<FsChangeEvent> {
    FS_EVENT_BROADCASTER.subscribe()
}

pub async fn begin_ignore_operation(ttl: Duration) -> IgnoreOperationId {
    let id = OP_ID.fetch_add(1, Ordering::Relaxed);
    let op = IgnoreOperation {
        paths: Vec::new(),
        expires_at: Instant::now() + ttl,
    };

    let mut ops = IGNORE_OPERATIONS.write().await;
    ops.insert(id, op);

    IgnoreOperationId(id)
}

pub async fn begin_ignore_paths<I>(paths: I, ttl: Duration) -> IgnoreOperationId
where
    I: IntoIterator<Item = PathBuf>,
{
    let id = OP_ID.fetch_add(1, Ordering::Relaxed);
    let op = IgnoreOperation {
        paths: paths.into_iter().collect(),
        expires_at: Instant::now() + ttl,
    };

    let mut ops = IGNORE_OPERATIONS.write().await;
    ops.insert(id, op);

    IgnoreOperationId(id)
}

pub async fn add_ignore_path(operation_id: IgnoreOperationId, path: PathBuf) {
    let mut ops = IGNORE_OPERATIONS.write().await;
    if let Some(op) = ops.get_mut(&operation_id.0) {
        op.paths.push(path);
    }
}

pub async fn add_ignore_paths<I>(operation_id: IgnoreOperationId, paths: I)
where
    I: IntoIterator<Item = PathBuf>,
{
    let mut ops = IGNORE_OPERATIONS.write().await;
    if let Some(op) = ops.get_mut(&operation_id.0) {
        op.paths.extend(paths);
    }
}

pub async fn extend_ignore_operation(operation_id: IgnoreOperationId, ttl: Duration) {
    let mut ops = IGNORE_OPERATIONS.write().await;
    if let Some(op) = ops.get_mut(&operation_id.0) {
        op.expires_at = Instant::now() + ttl;
    }
}

pub async fn finish_ignore_operation(operation_id: IgnoreOperationId) {
    let mut ops = IGNORE_OPERATIONS.write().await;
    ops.remove(&operation_id.0);
}

pub async fn mark_ignore(path: PathBuf, ttl: Duration) {
    let _ = begin_ignore_paths(vec![path], ttl).await;
}

pub async fn mark_ignore_many<I>(paths: I, ttl: Duration)
where
    I: IntoIterator<Item = PathBuf>,
{
    let _ = begin_ignore_paths(paths, ttl).await;
}

pub async fn mark_ignore_with_parent(path: &Path, ttl: Duration) {
    let mut paths = vec![path.to_path_buf()];
    if let Some(parent) = path.parent() {
        paths.push(parent.to_path_buf());
    }
    let _ = begin_ignore_paths(paths, ttl).await;
}

async fn cleanup_expired_operations() {
    let now = Instant::now();
    let mut ops = IGNORE_OPERATIONS.write().await;
    ops.retain(|_, op| op.expires_at > now);
}

fn path_matches_ignored_path(event_path: &Path, ignored_path: &Path) -> bool {
    event_path == ignored_path
        || event_path.starts_with(ignored_path)
        || ignored_path.starts_with(event_path)
        || ignored_path
        .parent()
        .map(|parent| parent == event_path)
        .unwrap_or(false)
        || event_path
        .parent()
        .map(|parent| parent == ignored_path)
        .unwrap_or(false)
}

pub async fn should_ignore(path: &Path) -> bool {
    cleanup_expired_operations().await;

    let ops = IGNORE_OPERATIONS.read().await;
    for op in ops.values() {
        for ignored_path in &op.paths {
            if path_matches_ignored_path(path, ignored_path) {
                return true;
            }
        }
    }
    false
}

pub async fn should_ignore_any<'a, I>(paths: I) -> bool
where
    I: IntoIterator<Item = &'a Path>,
{
    cleanup_expired_operations().await;

    let incoming: Vec<&Path> = paths.into_iter().collect();
    let ops = IGNORE_OPERATIONS.read().await;

    for op in ops.values() {
        for ignored_path in &op.paths {
            for path in &incoming {
                if path_matches_ignored_path(path, ignored_path) {
                    return true;
                }
            }
        }
    }

    false
}

fn to_relative_path(root_dir: &Path, path: &Path) -> String {
    path.strip_prefix(root_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn event_kind_to_string(kind: &EventKind) -> String {
    format!("{:?}", kind)
}

fn is_ignored_filename(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == "file.rs")
        .unwrap_or(false)
}

async fn send_fs_event(
    tx: &broadcast::Sender<FsChangeEvent>,
    root_dir: &Path,
    kind: String,
    path: &Path,
    old_path: Option<&Path>,
) {
    let _ = tx.send(FsChangeEvent {
        kind,
        path: to_relative_path(root_dir, path),
        old_path: old_path.map(|p| to_relative_path(root_dir, p)),
        source: "external".to_string(),
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    });
}

async fn handle_notify_event(
    tx: &broadcast::Sender<FsChangeEvent>,
    root_dir: &Path,
    event: Event,
) {
    if event.paths.is_empty() {
        return;
    }

    if event.paths.iter().any(|p| is_ignored_filename(p)) {
        return;
    }

    let kind = event_kind_to_string(&event.kind);

    if should_ignore_any(event.paths.iter().map(|p| p.as_path())).await {
        return;
    }

    if event.paths.len() >= 2 {
        let old_path = &event.paths[0];
        let new_path = &event.paths[1];

        send_fs_event(tx, root_dir, kind, new_path, Some(old_path)).await;
        return;
    }

    let path = &event.paths[0];
    send_fs_event(tx, root_dir, kind, path, None).await;
}

pub async fn start_watcher(root_dir: PathBuf) -> notify::Result<()> {
    let tx = broadcaster();
    let (event_tx, mut event_rx) = mpsc::channel::<Event>(2048);

    let root_dir_for_task = root_dir.clone();
    let tx_for_task = tx.clone();

    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            handle_notify_event(&tx_for_task, &root_dir_for_task, event).await;
        }
    });

    let event_tx_for_cb = event_tx.clone();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| match res {
            Ok(event) => {
                let _ = event_tx_for_cb.try_send(event);
            }
            Err(err) => {
                eprintln!("[fs_events] watch error: {}", err);
            }
        },
        Config::default(),
    )?;

    watcher.watch(&root_dir, RecursiveMode::Recursive)?;

    tokio::spawn(async move {
        let _watcher = watcher;
        futures::future::pending::<()>().await;
    });

    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            cleanup_expired_operations().await;
        }
    });

    Ok(())
}
