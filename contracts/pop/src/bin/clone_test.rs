use abstract_interface::Abstract;
use cw_orch::daemon::networks::OSMOSIS_1;
use cw_orch::prelude::*;
use cw_orch::prelude::networks::PION_1;
use cw_orch_clone_testing::CloneTesting;
use pop::POP_ID;

use abstract_app::objects::namespace::Namespace;
use abstract_app::std::ABSTRACT_EVENT_TYPE;
use abstract_app::std::app::AppQueryMsg;
use abstract_client::{AbstractClient, Publisher};
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::parse_network, prelude::*, tokio::runtime::Runtime};
use pop::contract::interface::PopInterface;
use pop::msg::{PopExecuteMsgFns, PopInstantiateMsg, PopQueryMsg, PopQueryMsgFns};
use pop::state::{OrderType, Trade};
use slinky::SlinkyInstantiateMsg;

fn main() -> anyhow::Result<()> {
    let chain_info = PION_1;
    let mut chain = CloneTesting::new(chain_info)?;
    // Set the sender to the host proxy
    let abs = Abstract::load_from(chain.clone())?;

    // Create an [`AbstractClient`]
    let abstract_client: AbstractClient<_> = AbstractClient::new(chain.clone())?;

    let test_account = abstract_client
        .account_builder()
        // .sub_account(false)
        .build()?;

    // let slinky = test_account
    //     .install_adapter::<slinky::SlinkyInterface::<Daemon>>(&[])?;

    let pop_app = test_account
        .install_app_with_dependencies::<PopInterface::<_>>(&PopInstantiateMsg {
            count: 3,
        }, Empty {}, &[])?;

    let resp = pop_app.query_trade("BTC", "USD")?;

    println!("{:#?}", resp);
    Ok(())
}
