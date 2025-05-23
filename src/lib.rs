// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Messaging
//!
//! A generic messaging broker abstraction library that provides a common interface
//! for implementing various messaging backends such as RabbitMQ, MQTT, Kafka, etc.
//!
//! This crate defines the core abstractions and interfaces that concrete implementations
//! can build upon, providing a consistent API for messaging operations across different
//! messaging platforms.
//!
//! ## Main Components
//!
//! - [`dispatcher`]: Message consumption and handler registration.
//! - [`publisher`]: Message publishing capabilities.
//! - [`handler`]: Consumer handler traits and message structures.
//! - [`errors`]: Error types specific to messaging operations.

pub mod dispatcher;
pub mod errors;
pub mod handler;
pub mod publisher;
