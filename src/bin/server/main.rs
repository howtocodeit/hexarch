use hexarch::config::Config;
use hexarch::inbound::http::{HttpServer, HttpServerConfig};
use hexarch::outbound::sqlite::Sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let post_repo = Sqlite::new(&config.database_url).await?;

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(post_repo, server_config).await?;
    http_server.run().await
}
