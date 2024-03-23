use std::sync::Arc;

use redis::{
    aio::MultiplexedConnection,
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands, Client, FromRedisValue, RedisResult,
};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{stream_events::{StreamMessage, MESSAGE_STREAM_KEY}, StreamEvent};

#[derive(Debug)]
pub struct ServiceEventConsumer {
    con: Arc<Mutex<MultiplexedConnection>>,
    read_options: StreamReadOptions,
    group_name: String,
}

impl ServiceEventConsumer {
    pub async fn create(
        connection_string: &str,
        group_name: &str,
    ) -> anyhow::Result<ServiceEventConsumer> {
        let client = Client::open(connection_string)?;

        let mut con = client.get_multiplexed_tokio_connection().await?;

        let _: RedisResult<String> = con
            .xgroup_create_mkstream(MESSAGE_STREAM_KEY, group_name, "0")
            .await;

        let consumer_name = Uuid::new_v4().to_string();

        let read_options = StreamReadOptions::default()
            .group(group_name, &consumer_name)
            .block(1000)
            .count(16);

        tracing::info!(
            MESSAGE_STREAM_KEY,
            group_name,
            consumer_name,
            "📨 create service event consumer"
        );

        Ok(ServiceEventConsumer {
            con: Arc::new(Mutex::new(con)),
            read_options,
            group_name: group_name.to_string(),
        })
    }

    pub async fn read_events(&self) -> anyhow::Result<Vec<StreamMessage>> {
        let mut con = self.con.lock().await;

        let read_results: RedisResult<StreamReadReply> = con
            .xread_options(&[MESSAGE_STREAM_KEY], &[">"], &self.read_options)
            .await;

        match read_results {
            Ok(read_reply) => Ok(parse_messages_from_redis_reply(read_reply)),
            Err(err) => {
                tracing::error!(?err, "error reading event stream");
                Err(anyhow::Error::msg("redis read error"))
            }
        }
    }

    pub async fn ack_events(&self, stream_events: &[StreamMessage]) -> anyhow::Result<()> {
        if stream_events.len() == 0 {
            return Ok(())
        }

        let mut con = self.con.lock().await;

        let ids: Vec<_> = stream_events.iter().map(|se| &se.id).collect();

        let ack_resp: RedisResult<i32> = con.xack(MESSAGE_STREAM_KEY, &self.group_name, &ids).await;
        if let Err(err) = ack_resp {
            tracing::error!(?err, ?ids, "couldn't xack stream events");
        }

        Ok(())
    }
}

fn parse_messages_from_redis_reply(stream_read_reply: StreamReadReply) -> Vec<StreamMessage> {
    stream_read_reply
        .keys
        .iter()
        .flat_map(|stream_key| &stream_key.ids)
        .map(|stream_id| {
            let id = stream_id.id.clone();

            let evt = stream_id
                .map
                .get("evt")
                .map(|evt| String::from_redis_value(evt).ok())
                .flatten()
                .and_then(|evt| serde_json::from_str::<StreamEvent>(&evt).ok());

            let ctx = stream_id
                .map
                .get("ctx")
                .map(|evt| String::from_redis_value(evt).ok())
                .flatten();

            StreamMessage { id, evt, ctx }
        })
        .collect()
}
