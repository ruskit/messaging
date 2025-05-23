// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Errors
//!
//! This module defines the error types that can occur during messaging operations.
//!
//! The `MessagingError` enum represents all possible error conditions that can arise
//! when working with the messaging abstractions, including connection issues,
//! serialization problems, and handler failures.

use thiserror::Error;

/// Represents errors that can occur in messaging operations.
///
/// This enum contains variants for all possible error conditions that may arise
/// when interacting with messaging systems, from connection failures to serialization issues.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum MessagingError {
    /// An unexpected internal error occurred.
    #[error("internal error")]
    InternalError,

    /// No handler was registered for the received message type.
    #[error("there is no handler registered")]
    UnregisteredHandler,

    /// Failed to establish a connection to the messaging broker.
    #[error("failure to connect")]
    ConnectionError,

    /// Failed to create a consumer instance.
    #[error("failure to create the consumer")]
    CreatingConsumerError,

    /// Failed to serialize message data.
    #[error("serializing error")]
    SerializingError,

    /// Failed to deserialize message data.
    #[error("deserializing error")]
    DeserializingError,

    /// An error occurred while handling a message.
    #[error("error to handle message")]
    HandlerError,

    /// Failed to consume a message, with an optional error message.
    #[error("failure to consume message `{0}`")]
    ConsumerError(String),

    /// Failed to publish a message.
    #[error("failure to publish message")]
    PublisherError,
}
