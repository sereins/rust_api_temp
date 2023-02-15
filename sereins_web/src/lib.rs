extern crate core;

use state::Container;
use crate::initialize::init_application_config;

pub mod apps;
pub mod initialize;

/// 整个项目上下文ApplicationContext
pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub async fn init_context(){
    init_application_config().await;

}