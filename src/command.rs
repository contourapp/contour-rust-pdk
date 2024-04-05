use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Command<V> {
    Cron(Cron<V>),
    Manual(Manual<V>),
    Email(Email),
    Created(Created<V>),
    Updated(Updated<V>),
    Deleted(Deleted),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cron<C> {
    pub from: Option<DateTime<Utc>>,
    pub until: DateTime<Utc>,
    #[serde(skip)]
    pub command: PhantomData<C>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manual<C> {
    pub command: C,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    pub content_type: String,
    // base64 encoded
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Email {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub attachments: Vec<Attachment>,
}

impl Email {
    pub fn new(
        from: String,
        to: Vec<String>,
        subject: String,
        body: String,
        attachments: Vec<Attachment>,
    ) -> Self {
        Self {
            from,
            to,
            subject,
            body,
            attachments,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Created<R> {
    pub id: Uuid,
    pub record: R,
}

impl<R> Created<R> {
    pub fn new(id: Uuid, record: R) -> Self {
        Self { id, record }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Updated<R> {
    pub id: Uuid,
    pub record: R,
}

impl<R> Updated<R> {
    pub fn new(id: Uuid, record: R) -> Self {
        Self { id, record }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deleted {
    pub id: Uuid,
}

impl Deleted {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
