use crate::domain::author::ports::AuthorMetrics;

/// An unimplemented example of an adapter to [AuthorMetrics].
#[derive(Debug, Clone)]
pub struct Prometheus;

impl Prometheus {
    pub fn new() -> Self {
        Self
    }
}

impl AuthorMetrics for Prometheus {
    async fn record_creation_success(&self) {}

    async fn record_creation_failure(&self) {}
}
