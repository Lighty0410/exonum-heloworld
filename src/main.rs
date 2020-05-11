mod api;
mod contracts;
mod error;
mod proto;
mod schema;
mod transactions;

use crate::contracts::CryptocurrencyService;
use exonum_cli::{NodeBuilder, Spec};

#[macro_use]
extern crate serde_derive;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    exonum::helpers::init_logger()?;

    NodeBuilder::development_node()?
        .with(Spec::new(CryptocurrencyService).with_default_instance())
        .run()
        .await
}
