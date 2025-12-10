use clap::{Parser, ValueEnum};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{Request, Response, StatusCode, service::service_fn};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Debug, Clone, ValueEnum)]
enum LogFormat {
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Serve files from this directory
    #[arg(long)]
    serve_dir: Option<PathBuf>,

    /// Log output format
    #[arg(short, long, value_enum, default_value = "json")]
    format: LogFormat,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    match args.format {
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_level(false)
                .with_target(false)
                .init();
        }
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = TcpListener::bind(addr).await?;

    let serve_dir = args.serve_dir.map(Arc::new);

    match &serve_dir {
        Some(dir) => info!(
            port = args.port,
            dir = %dir.display(),
            "starting server in file-serve mode"
        ),
        None => info!(port = args.port, "starting server in test/log mode"),
    }

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!(?peer_addr, "accepted connection");

        let serve_dir = serve_dir.clone();
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            let service = service_fn(move |req| {
                let serve_dir = serve_dir.clone();
                async move { handle(req, serve_dir).await }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                warn!(?err, "error serving connection");
            }
        });
    }
}

async fn handle(
    req: Request<Incoming>,
    serve_dir: Option<Arc<PathBuf>>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();

    let body_bytes = req.collect().await?.to_bytes();

    if body_bytes.is_empty() {
        info!(?method, ?uri, ?headers, "incoming request");
    } else {
        let body = String::from_utf8_lossy(&body_bytes);
        info!(?method, ?uri, ?headers, %body, "incoming request");
    }

    // File serving mode
    if let Some(dir) = serve_dir {
        return serve_file(&dir, uri.path()).await;
    }

    // Default mode
    Ok(Response::new(Full::new(Bytes::from("OK\n"))))
}

async fn serve_file(dir: &Path, uri_path: &str) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let path = uri_path.trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    let full_path = dir.join(path);

    match fs::read(&full_path).await {
        Ok(bytes) => {
            info!(path = %full_path.display(), "serving file");
            Ok(Response::new(Full::new(Bytes::from(bytes))))
        }
        Err(e) => {
            warn!(path = %full_path.display(), error = %e, "file not found");
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .expect("failed to build response"))
        }
    }
}
