use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CanonicalAddr};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: CanonicalAddr,
    pub canvas_size: u32,
    pub colors_count: u8,
    pub ibc_channel: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const PIXELS: Map<u32, u8> = Map::new("p");
