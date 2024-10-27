use std::collections::BTreeMap;

use anyhow;
use cosmwasm_std::{Addr, HexBinary, Uint128};
use serde::{Deserialize, Serialize};
use pop::state::DecryptedTrade;

#[derive(Clone, Debug)]
pub struct State {
    pub state: BTreeMap<Addr, Uint128>,
}

#[derive(Clone, Debug,Deserialize, Serialize)]
pub struct TradeState {
    pub state: Vec<DecryptedTrade>,
}



#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RawState {
    pub state: BTreeMap<Addr, Uint128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RawTradeState {
    pub state: HexBinary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawEncryptedState {
    pub ciphertext: HexBinary,
}

impl From<State> for RawState {
    fn from(o: State) -> Self {
        Self { state: o.state }
    }
}

impl From<TradeState> for RawTradeState {
    fn from(o: TradeState) -> Self {
        Self { state: HexBinary::from(serde_json::to_vec(&o).unwrap()) }
    }
}


impl TryFrom<RawState> for State {
    type Error = anyhow::Error;

    fn try_from(o: RawState) -> Result<Self, anyhow::Error> {
        Ok(Self { state: o.state })
    }
}

impl TryFrom<RawTradeState> for TradeState {
    type Error = anyhow::Error;

    fn try_from(o: RawTradeState) -> Result<Self, anyhow::Error> {
        let a = serde_json::from_slice::<Self>(&o.state.to_vec())?;
        Ok(Self { state:a.state  })
    }
}

#[derive(Clone, Debug)]
pub struct Balance {
    pub balance: Uint128,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RawBalance {
    pub balance: Uint128,
}

impl From<Balance> for RawBalance {
    fn from(o: Balance) -> Self {
        Self { balance: o.balance }
    }
}

impl TryFrom<RawBalance> for Balance {
    type Error = anyhow::Error;

    fn try_from(o: RawBalance) -> Result<Self, anyhow::Error> {
        Ok(Self { balance: o.balance })
    }
}
