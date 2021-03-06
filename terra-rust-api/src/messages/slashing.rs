use crate::core_types::MsgInternal;

use crate::messages::Message;
use serde::Serialize;

/// Unjail message
#[derive(Serialize, Debug)]
pub struct MsgUnjail {
    pub address: String,
}
impl MsgInternal for MsgUnjail {}
impl MsgUnjail {
    pub fn create(address: String) -> anyhow::Result<Message> {
        let internal = MsgUnjail { address };
        Ok(Message {
            s_type: "slashing/MsgUnjail".into(),
            value: serde_json::to_value(internal)?,
        })
    }
}
