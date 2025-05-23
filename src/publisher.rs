// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Publisher
//!
//! This module provides the publisher interface and message structure for sending messages.
//!
//! The `Publisher` trait defines how messages should be published to the messaging broker,
//! while the `PublishMessage` struct represents a message to be sent with its metadata.
//! The module also includes `HeaderValues` which provides type-safe header values for messages.

use crate::errors::MessagingError;
use async_trait::async_trait;
use opentelemetry::Context;
use std::collections::HashMap;

#[cfg(feature = "mocks")]
use mockall::*;

/// Represents the possible data types for message headers.
///
/// This enum allows for strongly-typed header values of different types,
/// which can be useful for protocols that support different data types in headers.
#[derive(Clone)]
pub enum HeaderValues {
    /// A short string value.
    ShortString(String),

    /// A long string value.
    LongString(String),

    /// An 8-bit signed integer value.
    Int(i8),

    /// A 32-bit signed integer value.
    LongInt(i32),

    /// A 64-bit signed integer value.
    LongLongInt(i64),

    /// An 8-bit unsigned integer value.
    Uint(u8),

    /// A 32-bit unsigned integer value.
    LongUint(u32),

    /// A 64-bit unsigned integer value.
    LongLongUint(u64),
}

impl From<HeaderValues> for String {
    /// Converts a header value to its string representation.
    ///
    /// # Arguments
    ///
    /// * `val` - The header value to convert.
    ///
    /// # Returns
    ///
    /// A string representation of the header value.
    fn from(val: HeaderValues) -> Self {
        match val {
            HeaderValues::ShortString(v) => v,
            HeaderValues::LongString(v) => v,
            HeaderValues::Int(v) => v.to_string(),
            HeaderValues::LongInt(v) => v.to_string(),
            HeaderValues::LongLongInt(v) => v.to_string(),
            HeaderValues::Uint(v) => v.to_string(),
            HeaderValues::LongUint(v) => v.to_string(),
            HeaderValues::LongLongUint(v) => v.to_string(),
        }
    }
}

/// Represents a message to be published to a messaging broker.
///
/// This struct contains the message content along with metadata such as the destination,
/// routing key, message type, and headers.
#[derive(Clone)]
pub struct PublishMessage {
    /// Optional source identifier for the message.
    pub from: Option<String>,

    /// The destination for the message (e.g., exchange, topic, or queue name).
    pub to: String,

    /// Optional routing key for the message.
    pub key: Option<String>,

    /// Optional message type identifier.
    pub msg_type: Option<String>,

    /// The raw binary data of the message.
    pub data: Box<[u8]>,

    /// Optional headers associated with the message.
    pub headers: Option<HashMap<String, HeaderValues>>,
}

impl PublishMessage {
    /// Creates a new publish message.
    ///
    /// # Arguments
    ///
    /// * `from` - Optional source identifier for the message.
    /// * `to` - The destination for the message.
    /// * `key` - Optional routing key for the message.
    /// * `msg_type` - Optional message type identifier.
    /// * `data` - The raw binary data of the message.
    /// * `headers` - Optional headers associated with the message.
    ///
    /// # Returns
    ///
    /// A new `PublishMessage` instance.
    pub fn new<T>(
        from: Option<T>,
        to: T,
        key: Option<T>,
        msg_type: Option<T>,
        data: &[u8],
        headers: Option<HashMap<String, HeaderValues>>,
    ) -> Self
    where
        T: Into<String>,
    {
        let from = if from.is_some() {
            let f = from.unwrap();
            Some(f.into())
        } else {
            None
        };

        let key = if key.is_some() {
            let k = key.unwrap();
            Some(k.into())
        } else {
            None
        };

        let msg_type = if msg_type.is_some() {
            let m = msg_type.unwrap();
            Some(m.into())
        } else {
            None
        };

        PublishMessage {
            from,
            to: to.into(),
            key,
            msg_type,
            data: data.into(),
            headers,
        }
    }
}

/// Defines the interface for publishing messages to a messaging broker.
///
/// Implementations of this trait provide the logic for sending messages
/// to specific messaging systems like RabbitMQ, Kafka, MQTT, etc.
#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait Publisher: Send + Sync {
    /// Publishes a message to the messaging broker.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The OpenTelemetry context for tracing and monitoring.
    /// * `msg` - The message to publish.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if publishing fails.
    async fn publish(&self, ctx: &Context, msg: &PublishMessage) -> Result<(), MessagingError>;
}
