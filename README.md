# Ruskit Messaging

A Rust library providing a generic messaging broker abstraction that can be implemented for various messaging systems like RabbitMQ, MQTT, Kafka, and more.

## Overview

This library offers a standardized interface for messaging operations, allowing applications to work with different message brokers interchangeably using a common API. The abstraction layer enables switching between messaging implementations with minimal code changes.

## Key Features

- **Broker-Agnostic API**: Interact with any messaging system through a unified interface
- **Flexible Message Handling**: Register handlers for specific message types
- **Asynchronous Operations**: Built with async/await support for non-blocking operations
- **OpenTelemetry Integration**: Built-in support for tracing and monitoring
- **Error Handling**: Comprehensive error types for messaging operations
- **Mock Support**: Testing utilities available through the `mocks` feature

## Core Components

### Publisher

Publishes messages to the messaging broker:

```rust
#[async_trait]
pub trait Publisher: Send + Sync {
    async fn publish(&self, ctx: &Context, msg: &PublishMessage) -> Result<(), MessagingError>;
}
```

### Dispatcher

Consumes messages and dispatches them to registered handlers:

```rust
#[async_trait]
pub trait Dispatcher: Send + Sync {
    fn register(self, definition: &DispatcherDefinition, handler: Arc<dyn ConsumerHandler>) -> Self;
    async fn consume_blocking(&self) -> Result<(), MessagingError>;
}
```

### Consumer Handler

Processes incoming messages:

```rust
#[async_trait]
pub trait ConsumerHandler: Send + Sync {
    async fn exec(&self, ctx: &Context, msg: &ConsumerMessage) -> Result<(), MessagingError>;
}
```

## Usage Example

```rust
use messaging::{
    dispatcher::{Dispatcher, DispatcherDefinition},
    handler::{ConsumerHandler, ConsumerMessage},
    publisher::{Publisher, PublishMessage},
    errors::MessagingError,
};
use opentelemetry::Context;
use std::{collections::HashMap, sync::Arc};

// Implement a message handler
struct MyHandler;

#[async_trait::async_trait]
impl ConsumerHandler for MyHandler {
    async fn exec(&self, ctx: &Context, msg: &ConsumerMessage) -> Result<(), MessagingError> {
        println!("Received message of type: {}", msg.msg_type);
        // Process message...
        Ok(())
    }
}

// Using with a concrete implementation (example)
async fn example<D: Dispatcher, P: Publisher>(dispatcher: D, publisher: P) {
    // Register handler
    let handler = Arc::new(MyHandler);
    let definition = DispatcherDefinition::new("my_queue", Some("my_message_type"));
    let dispatcher = dispatcher.register(&definition, handler);
    
    // Start consuming in background
    tokio::spawn(async move {
        dispatcher.consume_blocking().await.expect("Failed to consume");
    });
    
    // Publish a message
    let data = b"Hello, world!";
    let msg = PublishMessage::new(
        Some("my_service"),
        "my_queue",
        None,
        Some("my_message_type"),
        data,
        None,
    );
    
    publisher.publish(&Context::current(), &msg).await.expect("Failed to publish");
}
```

## Implementing a New Backend

To add support for a new messaging system:

1. Create a new crate that depends on this one
2. Implement the `Publisher` and `Dispatcher` traits for your messaging system
3. Provide factory methods to construct your implementations

Example implementation for a hypothetical messaging system:

```rust
use messaging::{
    dispatcher::{Dispatcher, DispatcherDefinition},
    handler::ConsumerHandler,
    publisher::{Publisher, PublishMessage},
    errors::MessagingError,
};

pub struct MySystemPublisher {
    // Implementation-specific fields
}

#[async_trait::async_trait]
impl Publisher for MySystemPublisher {
    async fn publish(&self, ctx: &Context, msg: &PublishMessage) -> Result<(), MessagingError> {
        // Implementation-specific publishing logic
        Ok(())
    }
}

pub struct MySystemDispatcher {
    // Implementation-specific fields
}

#[async_trait::async_trait]
impl Dispatcher for MySystemDispatcher {
    fn register(mut self, definition: &DispatcherDefinition, handler: Arc<dyn ConsumerHandler>) -> Self {
        // Implementation-specific registration logic
        self
    }

    async fn consume_blocking(&self) -> Result<(), MessagingError> {
        // Implementation-specific consumption logic
        Ok(())
    }
}
```

## Feature Flags

- `mocks`: Enables mock implementations of traits for testing

## Testing

The library provides mock implementations when the `mocks` feature is enabled:

```rust
use messaging::{
    dispatcher::MockDispatcher,
    handler::MockConsumerHandler,
    publisher::MockPublisher,
};

#[tokio::test]
async fn test_publisher() {
    let mut mock = MockPublisher::new();
    mock.expect_publish()
        .returning(|_, _| Ok(()));
    
    // Test with the mock
}
```

## License

MIT License - Copyright (c) 2025, The Ruskit Authors