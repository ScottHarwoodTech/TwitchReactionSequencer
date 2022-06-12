use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReactionSequenceItemSequence {
    #[doc = " Arguments to be passed to action"]
    pub arguments: Vec<serde_json::Value>,
    #[doc = " id of action to perform"]
    #[serde(rename = "deviceActionId")]
    pub device_action_id: String,
    #[doc = " id of device associated with step"]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = " id of step"]
    pub id: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ReactionSequence {
    #[doc = " Unique Id of reaction sequence"]
    pub id: String,
    #[doc = " Name of reaction sequence"]
    pub name: String,
    #[doc = " List of actions to run when reacting"]
    pub sequence: Vec<ReactionSequenceItemSequence>,
}
