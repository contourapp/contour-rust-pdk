use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Command<V, Re, M> {
    Cron(Cron<V>),
    Manual(Manual<V>),
    Request(Request<Re, Option<M>>),
    Email(Email),
    Created(Created<V>),
    Updated(Updated<V>),
    Deleted(Deleted),
}

impl<V, Re, M> std::fmt::Display for Command<V, Re, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Cron(_) => write!(f, "Cron"),
            Command::Manual(_) => write!(f, "Manual"),
            Command::Request(_) => write!(f, "Request",),
            Command::Email(_) => write!(f, "Email"),
            Command::Created(_) => write!(f, "Created"),
            Command::Updated(_) => write!(f, "Updated"),
            Command::Deleted(_) => write!(f, "Deleted"),
        }
    }
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
pub struct Request<B, M> {
    pub status_code: u16,
    pub body: B,
    pub metadata: M,
}

impl<B, M> Request<B, M> {
    pub fn new(status_code: u16, body: B, metadata: M) -> Self {
        Self {
            status_code,
            body,
            metadata,
        }
    }
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
