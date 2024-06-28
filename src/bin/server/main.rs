use hexarch::config::Config;
use hexarch::domain::blog::service::Service;
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
    let prometheus = Prometheus::new();
    let email_client = EmailClient::new();
    let blog_service = Service::new(sqlite, prometheus, email_client);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(blog_service, server_config).await?;
    http_server.run().await
}
