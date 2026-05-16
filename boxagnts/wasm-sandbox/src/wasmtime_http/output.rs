use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use tokio::io::{self, AsyncWrite};
use wasmtime_wasi::p2::{StreamError, StreamResult};

#[derive(Clone)]
pub enum Output {
    Stdout,
    Stderr,
}

impl Output {
    fn write_all(&self, buf: &[u8]) -> io::Result<()> {
        use std::io::Write;

        match self {
            Output::Stdout => std::io::stdout().write_all(buf),
            Output::Stderr => std::io::stderr().write_all(buf),
        }
    }
}

#[derive(Clone)]
pub struct LogStream {
    output: Output,
    state: Arc<LogStreamState>,
}

struct LogStreamState {
    prefix: String,
    needs_prefix_on_next_write: AtomicBool,
}

impl LogStream {
    pub fn new(prefix: String, output: Output) -> LogStream {
        LogStream {
            output,
            state: Arc::new(LogStreamState {
                prefix,
                needs_prefix_on_next_write: AtomicBool::new(true),
            }),
        }
    }

    fn write_all(&mut self, mut bytes: &[u8]) -> io::Result<()> {
        while !bytes.is_empty() {
            if self
                .state
                .needs_prefix_on_next_write
                .load(Ordering::Relaxed)
            {
                self.output.write_all(self.state.prefix.as_bytes())?;
                self.state
                    .needs_prefix_on_next_write
                    .store(false, Ordering::Relaxed);
            }
            match bytes.iter().position(|b| *b == b'\n') {
                Some(i) => {
                    let (a, b) = bytes.split_at(i + 1);
                    bytes = b;
                    self.output.write_all(a)?;
                    self.state
                        .needs_prefix_on_next_write
                        .store(true, Ordering::Relaxed);
                }
                None => {
                    self.output.write_all(bytes)?;
                    break;
                }
            }
        }

        Ok(())
    }
}

impl wasmtime_wasi::cli::StdoutStream for LogStream {
    fn p2_stream(&self) -> Box<dyn wasmtime_wasi::p2::OutputStream> {
        Box::new(self.clone())
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(self.clone())
    }
}

impl wasmtime_wasi::cli::IsTerminal for LogStream {
    fn is_terminal(&self) -> bool {
        match &self.output {
            Output::Stdout => std::io::stdout().is_terminal(),
            Output::Stderr => std::io::stderr().is_terminal(),
        }
    }
}

impl wasmtime_wasi::p2::OutputStream for LogStream {
    fn write(&mut self, bytes: bytes::Bytes) -> StreamResult<()> {
        self.write_all(&bytes)
            .map_err(|e| StreamError::LastOperationFailed(e.into()))?;
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

#[async_trait::async_trait]
impl wasmtime_wasi::p2::Pollable for LogStream {
    async fn ready(&mut self) {}
}

impl AsyncWrite for LogStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(self.write_all(buf).map(|_| buf.len()))
    }
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}