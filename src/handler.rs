// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Handler
//!
//! This module provides the handler interface and message structure for consuming messages.
//!
//! The `ConsumerHandler` trait defines how incoming messages should be processed, while
//! the `ConsumerMessage` struct represents a received message with its metadata.

use crate::errors::MessagingError;
use async_trait::async_trait;
use opentelemetry::Context;
use std::collections::HashMap;

#[cfg(feature = "mocks")]
use mockall::*;

/// Represents a message received from a message broker.
///
/// This struct contains the message content along with metadata such as the source,
/// message type, and headers.
#[derive(Clone, Default)]
pub struct ConsumerMessage {
    /// The source of the message (e.g., queue or exchange name).
    pub from: String,

    /// The type of the message, used for routing or processing decisions.
    pub msg_type: String,

    /// The raw binary data of the message.
    pub data: Box<[u8]>,

    /// Optional headers associated with the message.
    pub headers: Option<HashMap<String, String>>,
}

impl ConsumerMessage {
    /// Creates a new consumer message.
    ///
    /// # Arguments
    ///
    /// * `from` - The source of the message.
    /// * `msg_type` - The type of the message.
    /// * `data` - The raw binary data of the message.
    /// * `headers` - Optional headers associated with the message.
    ///
    /// # Returns
    ///
    /// A new `ConsumerMessage` instance.
    pub fn new<T>(
        from: T,
        msg_type: T,
        data: &[u8],
        headers: Option<HashMap<String, String>>,
    ) -> Self
    where
        T: Into<String>,
    {
        ConsumerMessage {
            from: from.into(),
            msg_type: msg_type.into(),
            data: data.into(),
            headers,
        }
    }
}

/// Defines the interface for handling consumed messages.
///
/// Implementations of this trait process incoming messages and define
/// the business logic for handling specific message types.
#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait ConsumerHandler: Send + Sync {
    /// Executes the handler logic for a received message.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The OpenTelemetry context for tracing and monitoring.
    /// * `msg` - The received message to process.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if handling fails.
    async fn exec(&self, ctx: &Context, msg: &ConsumerMessage) -> Result<(), MessagingError>;
}
