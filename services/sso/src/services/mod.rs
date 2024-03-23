pub mod message_processing;
pub mod credentials_manager;

use message_bus::ServiceEventConsumer;

use self::credentials_manager::CredentialsManager;

pub struct Services {
    pub consumer: ServiceEventConsumer,
    pub credentials_manager: CredentialsManager,
}
