use exonum::crypto::PublicKey;
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

use crate::schema::{CurrencySchema, Wallet};

#[derive(Debug, Clone, Copy)]
pub struct CryptocurrencyApi;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WalletQuery {
    pub pub_key: PublicKey,
}

impl CryptocurrencyApi {
    pub async fn get_wallet(state: ServiceApiState, query: WalletQuery) -> api::Result<Wallet> {
        let schema = CurrencySchema::new(state.service_data());

        schema
            .wallets
            .get(&query.pub_key)
            .ok_or_else(|| api::Error::not_found().title("Wallet not found"))
    }

    pub async fn get_wallets(state: ServiceApiState, _query: ()) -> api::Result<Vec<Wallet>> {
        let schema = CurrencySchema::new(state.service_data());

        Ok(schema.wallets.values().collect())
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/wallet", Self::get_wallet)
            .endpoint("v1/wallets", Self::get_wallets);
    }
}
