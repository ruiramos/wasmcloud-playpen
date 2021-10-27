// This file is generated automatically using wasmcloud-weld and smithy model definitions
//

#![allow(clippy::ptr_arg)]
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Cow, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TranslateInput {
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub target_lang: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TranslateOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub success: bool,
}

/// Description of Translate service
/// wasmbus.actorReceive
#[async_trait]
pub trait Translate {
    /// Converts the input string to a result
    async fn convert(&self, ctx: &Context, arg: &TranslateInput) -> RpcResult<TranslateOutput>;
}

/// TranslateReceiver receives messages defined in the Translate service trait
/// Description of Translate service
#[doc(hidden)]
#[async_trait]
pub trait TranslateReceiver: MessageDispatch + Translate {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Convert" => {
                let value: TranslateInput = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Translate::convert(self, ctx, &value).await?;
                let buf = Cow::Owned(serialize(&resp)?);
                Ok(Message {
                    method: "Translate.Convert",
                    arg: buf,
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Translate::{}",
                message.method
            ))),
        }
    }
}

/// TranslateSender sends messages to a Translate service
/// Description of Translate service
/// client for sending Translate messages
#[derive(Debug)]
pub struct TranslateSender<T: Transport> {
    transport: T,
}

impl<T: Transport> TranslateSender<T> {
    /// Constructs a TranslateSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> TranslateSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl TranslateSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Translate for TranslateSender<T> {
    #[allow(unused)]
    /// Converts the input string to a result
    async fn convert(&self, ctx: &Context, arg: &TranslateInput) -> RpcResult<TranslateOutput> {
        let arg = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Translate.Convert",
                    arg: Cow::Borrowed(&arg),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Convert", e)))?;
        Ok(value)
    }
}
