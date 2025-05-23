// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

use crate::errors::MessagingError;
use async_trait::async_trait;
use opentelemetry::Context;
use std::collections::HashMap;

#[cfg(feature = "mocks")]
use mockall::*;

#[derive(Clone)]
pub enum HeaderValues {
    ShortString(String),
    LongString(String),
    Int(i8),
    LongInt(i32),
    LongLongInt(i64),
    Uint(u8),
    LongUint(u32),
    LongLongUint(u64),
}

impl From<HeaderValues> for String {
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

#[derive(Clone)]
pub struct PublishMessage {
    pub from: Option<String>,
    pub to: String,
    pub key: Option<String>,
    pub msg_type: Option<String>,
    pub data: Box<[u8]>,
    pub headers: Option<HashMap<String, HeaderValues>>,
}

impl PublishMessage {
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

#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait Publisher: Send + Sync {
    async fn publish(&self, ctx: &Context, msg: &PublishMessage) -> Result<(), MessagingError>;
}
