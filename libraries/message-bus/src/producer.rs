use std::{collections::HashMap, sync::Arc};

use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use tokio::sync::Mutex;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use opentelemetry::global as otel;

use crate::stream_events::{StreamEvent, MESSAGE_STREAM_KEY};

#[derive(Debug, Clone)]
pub struct ServiceEventProducer {
    con: Arc<Mutex<MultiplexedConnection>>,
}

impl ServiceEventProducer {
    pub async fn create(
        connection_string: &str,
    ) -> anyhow::Result<ServiceEventProducer> {
        let client = Client::open(connection_string)?;
        let con = client.get_multiplexed_tokio_connection().await?;

        tracing::info!(MESSAGE_STREAM_KEY, "📨 create service event producer");

        Ok(ServiceEventProducer {
            con: Arc::new(Mutex::new(con)),
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn send_event(&self, evt: StreamEvent) -> anyhow::Result<()> {
        let evt = serde_json::to_string(&evt)?;
        let ctx = Span::current().context();

        let mut injected_context = HashMap::new();

        otel::get_text_map_propagator(|prop| {
            prop.inject_context(&ctx, &mut injected_context)
        });

        let distributed_context = serde_json::to_string(&injected_context)?;

        let mut con = self.con.lock().await;

        con.xadd(MESSAGE_STREAM_KEY, "*", &[("evt", evt), ("ctx", distributed_context)]).await?;

        Ok(())
    }
}
