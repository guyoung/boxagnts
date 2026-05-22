mod dashboard;
mod sites;
mod web_server;

use clap::Parser;
use std::path::PathBuf;


#[derive(Parser)]
#[command(name = "Boxagnts")]
#[command(
    about = "BoxAgnts is an open-source AI Agent ToolBox built with Rust.",
    long_about = "BoxAgnts is an open-source AI Agent ToolBox built with Rust, dedicated to delivering an ultimate out-of-the-box experience. Leveraging WebAssembly sandbox, it provides a runtime environment that balances security and flexibility, helping users effortlessly tackle a wide range of complex tasks and thus becoming an efficient and trustworthy personal AI assistant.
"
)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Port to run the web server on
    #[arg(long, value_name = "PORT", default_value = "30001")]
    port: u16,

    /// Host to bind to (0.0.0.0 for all interfaces)
    #[arg(long, value_name = "HOST", default_value = "127.0.0.1")]
    host: String,

    /// Set workspace dir, default current dir
    #[arg(long, value_name = "DIR")]
    workspace_dir: Option<String>,

    /// Set app dir, default Boxagnts executable file dir
    #[arg(long, value_name = "DIR")]
    app_dir: Option<String>,

    /// Set admin username
    #[arg(long, value_name = "USERNAME")]
    admin_user: Option<String>,

    /// Set admin passwrod
    #[arg(long, value_name = "PASSWORD")]
    admin_pass: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    if !is_local_host(&args.host) && (args.admin_user.is_none() || args.admin_pass.is_none()) {
        eprintln!("❌ When host is not local, --admin-user and --admin-pass are required.");
        std::process::exit(1);
    }

    println!("🚀 Starting Boxagnts Web Server...");

    let workspace_dir = if let Some(workspace_dir) = args.workspace_dir {
        PathBuf::from(workspace_dir)
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };

    boxagnts_workspace::path::set_worksapce_dir(&workspace_dir).await;

    let app_dir = if let Some(app_dir) = args.app_dir {
        PathBuf::from(app_dir)
    } else {
        boxagnts_workspace::path::get_default_app_dir()
    };

    boxagnts_workspace::path::set_app_dir(&app_dir).await;

    if let Err(e) =
        web_server::start_web_server(args.host, args.port, args.admin_user, args.admin_pass).await
    {
        eprintln!("❌ Failed to start Boxagnts Web Server: {}", e);
        std::process::exit(1);
    }
}

fn is_local_host(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}
