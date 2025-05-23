// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Dispatcher
//!
//! This module provides the dispatcher functionality for message consumption and handling.
//!
//! The dispatcher is responsible for routing incoming messages to the appropriate handler
//! based on the message type and subscription information. It manages the registration of
//! handlers and the consumption of messages from the broker.

use crate::{errors::MessagingError, handler::ConsumerHandler};
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(feature = "mocks")]
use mockall::*;

/// Defines a subscription for message consumption.
///
/// A dispatcher definition includes a name (typically a queue or topic name)
/// and an optional message type for filtering messages.
#[derive(Debug, Clone)]
pub struct DispatcherDefinition {
    /// The name of the queue or topic to subscribe to.
    pub name: String,

    /// Optional message type for filtering messages.
    pub msg_type: Option<String>,
}

impl DispatcherDefinition {
    /// Creates a new dispatcher definition.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the queue or topic to subscribe to.
    /// * `msg_type` - Optional message type for filtering messages.
    ///
    /// # Returns
    ///
    /// A new `DispatcherDefinition` instance.
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

/// Defines the interface for message dispatching.
///
/// Implementations of this trait are responsible for consuming messages
/// from the broker and routing them to the appropriate handler.
#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait Dispatcher: Send + Sync {
    /// Registers a handler for a specific dispatcher definition.
    ///
    /// # Arguments
    ///
    /// * `definition` - The dispatcher definition specifying what to subscribe to.
    /// * `handler` - The handler to process messages matching the definition.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining.
    fn register(self, definition: &DispatcherDefinition, handler: Arc<dyn ConsumerHandler>)
    -> Self;

    /// Starts consuming messages in a blocking manner.
    ///
    /// This method will block the current thread/task and continuously process
    /// incoming messages until an error occurs or the dispatcher is stopped.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if consumption fails.
    async fn consume_blocking(&self) -> Result<(), MessagingError>;
}
