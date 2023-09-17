use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub canvas_size: u32,
    pub colors_count: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IbcExecuteMsg {
    SetPixel {
        pixel: u32,
        color: u8,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetPixel {
        pixel: u32,
        color: u8,
    },
    SetChannel {
        channel: String,
    }
}
