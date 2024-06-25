use hexarch::config::Config;
use hexarch::domain::author::service::Service;
use hexarch::inbound::http::{HttpServer, HttpServerConfig};
use hexarch::outbound::email_client::EmailClient;
use hexarch::outbound::prometheus::Prometheus;
use hexarch::outbound::sqlite::Sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let sqlite = Sqlite::new(&config.database_url).await?;
    let metrics = Prometheus::new();
    let email_client = EmailClient::new();
    let author_service = Service::new(sqlite, metrics, email_client);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(author_service, server_config).await?;
    http_server.run().await
}
