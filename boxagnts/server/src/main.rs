


mod web_server;
mod dashboard;
mod sites;

use std::path::PathBuf;
use clap::Parser;



#[derive(Parser)]
#[command(name = "Boxagnts")]
#[command(about = "Boxagnts is an AI coding assistant focused on safe and effective software engineering assistance.")]
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

    println!("🚀 Starting Boxagnts Web Server...");

    let workspace_dir = if let Some( workspace_dir) = args.workspace_dir {
        PathBuf::from(workspace_dir)
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };

    boxagnts_workspace::path::set_worksapce_dir(&workspace_dir).await;


    let app_dir = if let Some( app_dir) = args.app_dir {
        PathBuf::from(app_dir)
    } else {
       boxagnts_workspace::path::get_default_app_dir()
    };

    boxagnts_workspace::path::set_app_dir(&app_dir).await;




    if let Err(e) = web_server::start_web_server(Some(args.port)).await {
        eprintln!("❌ Failed to start Boxagnts Web Server: {}", e);
        std::process::exit(1);
    }
}
