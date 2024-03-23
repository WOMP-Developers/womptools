pub mod message_processing;
pub mod data_processing;

use message_bus::{ServiceEventConsumer, ServiceEventProducer};
use service_sso_api::ServiceSSO;

use crate::database::Database;

pub struct Services {
    pub database: Database,
    pub consumer: ServiceEventConsumer,
    pub producer: ServiceEventProducer,
    pub service_sso: ServiceSSO,
}
