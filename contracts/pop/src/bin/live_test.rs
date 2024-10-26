//! Publishes the module to the Abstract platform by uploading it and registering it on the app store.
//!
//! Info: The mnemonic used to register the module must be the same as the owner of the account that claimed the namespace.
//!
//! ## Example
//!
//! ```bash
//! $ just publish pop uni-6 osmo-test-5
//! ```
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
use slinky::msg::{SlinkyQueryMsgFns};
use abstract_app::std::account::QueryMsgFns as AccountQueryMsgFns;
use slinky::SlinkyInterface;
use abstract_app::abstract_interface::AccountExecFns;

const TESTACCOUNT: &str = "testaccount9";

fn test(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    // run for each requested network
    for network in networks {
        // Setup
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network)
            .handle(rt.handle())
            .build()?;
        // let chain = Clone::new(network)
        //     .handle(rt.handle())
        //     .build()?;

        let app_namespace = Namespace::from_id(POP_ID)?;

        // Create an [`AbstractClient`]
        let abstract_client: AbstractClient<Daemon> = AbstractClient::new(chain.clone())?;


        let test_account =  match abstract_client
            .fetch_account(TESTACCOUNT.try_into()?) {
            Ok(acc) => acc,
            Err(_) => abstract_client
                .account_builder()
                .namespace(TESTACCOUNT.try_into()?)
                .build()?
        };

        // test_account.as_ref().uninstall_module(POP_ID)?;
        // test_account.as_ref().uninstall_module(slinky::SLINKY_ID)?;

        let pop_app = test_account
            .install_app_with_dependencies::<PopInterface::<_>>(&PopInstantiateMsg {
                count: 3,
            }, Empty {}, &[])?;



        pop_app.upgrade(None)?;

        let slinky = test_account.application::<SlinkyInterface<_>>()?;
        let pairs = slinky.pairs()?;



        println!("pairs: {:?}",pairs);
        // let resp = pop_app.query(&abstract_app::std::app::QueryMsg::Module(PopQueryMsg::Trade{
        //     base: "BTC".into(),
        //     quote: "USD".into()
        // }))?;
        // println!("{:#?}", resp);

        let resp = pop_app.query_trade("BTC", "USD")?;

        println!("{:#?}", resp);

        // resp.get_attribute_from_logs(ABSTRACT_EVENT_TYPE,"");
        // let events = resp.get_events(ABSTRACT_EVENT_TYPE);
        // println!("{:#?}", resp);
        // println!("{:#?}", events);
    }
    Ok(())
}

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to publish on
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    network_ids: Vec<String>,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let args = Arguments::parse();
    let networks = args
        .network_ids
        .iter()
        .map(|n| parse_network(n).unwrap())
        .collect();
    test(networks).unwrap();
}
