//! Publishes the module to the Abstract platform by uploading it and registering it on the app store.
//!
//! Info: The mnemonic used to register the module must be the same as the owner of the account that claimed the namespace.
//!
//! ## Example
//!
//! ```bash
//! $ just publish slinky uni-6 osmo-test-5
//! ```
use slinky::{
    contract::interface::SlinkyInterface, msg::SlinkyInstantiateMsg, SLINKY_ID,
};

use abstract_adapter::objects::namespace::Namespace;
use abstract_client::{AbstractClient, AccountBuilder, Publisher, PublisherBuilder};
use clap::Parser;
use cw_orch::{anyhow, daemon::networks::parse_network, prelude::*, tokio::runtime::Runtime};

fn publish(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    // run for each requested network
    for network in networks {
        // Setup
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network)
            .handle(rt.handle())
            .build()?;

        let adapter_namespace = Namespace::from_id(SLINKY_ID)?;

        // let abstr = Abstract::load_from(chain.clone())?;

        // Create an [`AbstractClient`]
        let abstract_client: AbstractClient<Daemon> = AbstractClient::new(chain.clone())?;

        // // Get the [`Publisher`] that owns the namespace, otherwise create a new one and claim the namespace
        // let publisher_acc = abstract_client
        //     .account_builder()
        //     .namespace(adapter_namespace)
        //     .build()?;


        // Get the [`Publisher`] that owns the namespace, otherwise create a new one and claim the namespace
        let publisher: Publisher<_> = abstract_client
            .publisher_builder(adapter_namespace)
            .build()?;

        // let publisher = PublisherBuilder::new(AccountBuilder::new(&abstr), namespace)

        if publisher.account().owner()? != chain.sender_addr() {
            panic!("The current sender can not publish to this namespace. Please use the wallet that owns the Account that owns the Namespace.")
        }

        // Publish the Adapter to the Abstract Platform
        publisher.publish_adapter::<SlinkyInstantiateMsg, SlinkyInterface<Daemon>>(
            SlinkyInstantiateMsg {},
        )?;
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
    publish(networks).unwrap();
}
