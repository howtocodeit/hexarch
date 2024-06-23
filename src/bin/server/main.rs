use hexarch::config::Config;
use hexarch::domain::author::service::Service;
use hexarch::inbound::http::{HttpServer, HttpServerConfig};
use hexarch::outbound::sqlite::Sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let author_repo = Sqlite::new(&config.database_url).await?;
    let author_service = Service::new(author_repo);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(author_service, server_config).await?;
    http_server.run().await
}
