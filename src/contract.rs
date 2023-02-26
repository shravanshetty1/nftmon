#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw721::Cw721Execute;
use cw721_base::{ContractError, Cw721Contract, ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::msg::CustomExtension;
// use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nftmon";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let tract = Cw721Contract::<CustomExtension, Empty, Empty, Empty>::default();
    tract.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<CustomExtension, Empty>,
) -> Result<Response, ContractError> {
    let tract = Cw721Contract::<CustomExtension, Empty, Empty, Empty>::default();
    let resp = match msg {
        // on successfull transfer update score
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => {
            let mut token = tract.tokens.load(deps.storage, &token_id)?;

            // score update will be larger if the token hasnt been transferred recently
            let mut update_score_by = env.block.time.nanos() - token.extension.last_action.nanos();

            // if transfered to a new address double the update
            if token.owner != recipient {
                update_score_by *= 2;
            }

            token.extension.score += update_score_by;
            token.extension.last_action = env.block.time;
            tract.tokens.save(deps.storage, &token_id, &token)?;

            // if transfer fails score wont be updated, since smart contract is atomic
            tract.transfer_nft(deps, env, info, recipient, token_id)?
        }
        // ensure score is zero on mint
        ExecuteMsg::Mint(mut msg) => {
            msg.extension.score = 0;
            msg.extension.last_action = env.block.time;
            tract.mint(deps, env, info, msg)?
        }
        _ => tract.execute(deps, env, info, msg)?,
    };

    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
    let tract = Cw721Contract::<CustomExtension, Empty, Empty, Empty>::default();
    tract.query(deps, env, msg)
}
