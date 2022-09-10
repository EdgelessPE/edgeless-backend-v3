use actix::prelude::*;
use serde::{Serialize, Deserialize};
use crate::class::HelloResponse;
use crate::class::AlphaResponse;
use std::time::SystemTime;
use crate::class::EptFileNode;
use std::collections::HashMap;
use std::sync::Arc;

pub type EptFileRoot = HashMap<String, Vec<EptFileNode>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefreshMessage {
  Refresh { force: bool }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchMessage { force: bool }

impl Into<RefreshMessage> for FetchMessage {
  fn into(self) -> RefreshMessage {
    RefreshMessage::Refresh { force: self.force }
  }
}

// 你又在偷看我力。
impl Message for FetchMessage {
  type Result = anyhow::Result<ResponseData>;
}

impl Message for RefreshMessage {
  type Result = anyhow::Result<UpdateResponseCacheMessage>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseData {
  pub hello: HelloResponse,
  pub alpha: AlphaResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResponseCacheMessage {
  pub data: Option<ResponseData>,
  pub time: SystemTime
}

impl Message for UpdateResponseCacheMessage {
  type Result = ();
}

#[derive(Debug)]
pub struct EptResponder {
  pub daemon: Recipient<RefreshMessage>,
  pub cache: Option<ResponseData>
}

impl Actor for EptResponder {
  type Context = Context<EptResponder>;
}

impl Handler<FetchMessage> for EptResponder {
  type Result = anyhow::Result<ResponseData>;
  fn handle(&mut self, n: FetchMessage, ctx: &mut Self::Context) -> Self::Result {
    let message = {
      if let None = self.cache {
        self.daemon.do_send(RefreshMessage::Refresh { force: true }).await??
      } else {
        self.daemon.send(n.into()).into_actor(a)
      }
    };

    todo!()
  }
}

#[derive(Debug)]
pub struct Daemon {
  pub last_updated: SystemTime,

  pub responder: Recipient<UpdateResponseCacheMessage>
}

impl Actor for Daemon {
  type Context = Context<Daemon>;
}
