// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

use crate::{errors::MessagingError, handler::ConsumerHandler};
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(feature = "mocks")]
use mockall::*;

#[derive(Debug, Clone)]
pub struct DispatcherDefinition {
    pub name: String,
    pub msg_type: Option<String>,
}

impl DispatcherDefinition {
    pub fn new<T>(name: T, msg_type: Option<T>) -> Self
    where
        T: Into<String>,
    {
        let msg_type = if msg_type.is_some() {
            let v = msg_type.unwrap();
            Some(v.into())
        } else {
            None
        };

        DispatcherDefinition {
            name: name.into(),
            msg_type,
        }
    }
}

#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait Dispatcher: Send + Sync {
    fn register(self, definition: &DispatcherDefinition, handler: Arc<dyn ConsumerHandler>)
    -> Self;

    async fn consume_blocking(&self) -> Result<(), MessagingError>;
}
