pub mod msg;
pub mod query;
pub mod state;
pub mod ibc;
pub mod error;

pub mod entry {
    use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response, entry_point};
    use cw2::{get_contract_version, set_contract_version};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::{CONFIG, Config, PIXELS};
    use semver::Version;
    use crate::error::ContractError;

    const CONTRACT_NAME: &str = "crates.io:multi-place-receiver";
    const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {

        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let config = Config {
            admin: deps.api.addr_canonicalize(info.sender.as_str())?,
            canvas_size: msg.canvas_size,
            colors_count: msg.colors_count.unwrap_or(16),
            ibc_channel: "".to_string(),
        };

        CONFIG.save(deps.storage,&config)?;

        return Ok(Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("owner", info.sender)
            .add_attribute("canvas_square", msg.canvas_size.to_string())
        )
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
        let new_version: Version = CONTRACT_VERSION.parse().unwrap();
        let current_version = get_contract_version(deps.storage)?;

        if current_version.contract != CONTRACT_NAME {
            return Err(ContractError::TypeMismatch {});
        }

        if current_version.version.parse::<Version>().unwrap() >= new_version {
            return Err(ContractError::BadVersion {});
        }

        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::new()
            .add_attribute("method", "migrate")
        )
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {

        match msg {
            ExecuteMsg::SetPixel { pixel, color } => try_set_pixel(
                deps,
                pixel,
                color,
            ),
            ExecuteMsg::SetChannel {channel} => try_set_channel(
                deps,
                info,
                channel
            )
        }
    }

    pub fn try_set_pixel(
        deps: DepsMut,
        pixel: u32,
        color: u8
    ) -> Result<Response, ContractError>  {

        let config = CONFIG.load(deps.storage)?;

        if color > config.colors_count {
            return Err(ContractError::UnknownColor {});
        }

        if pixel > config.canvas_size {
            return Err(ContractError::PixelOutOfBounds {});
        }

        PIXELS.save(deps.storage, pixel, &color)?;

        return Ok(Response::new()
            .add_attribute("method", "set_pixel")
        );
    }

    pub fn try_set_channel(
        deps: DepsMut,
        info: MessageInfo,
        channel: String
    ) -> Result<Response, ContractError> {
        let mut config = CONFIG.load(deps.storage)?;

        if deps.api.addr_canonicalize(info.sender.as_str())? != config.admin {
            return Err(ContractError::Unauthorized {});
        }

        config.ibc_channel = channel;
        CONFIG.save(deps.storage, &config)?;

        return Ok(Response::new()
            .add_attribute("method", "set_channel")
        );
    }

}
