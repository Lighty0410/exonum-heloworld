use exonum::{
    crypto::PublicKey,
    merkledb::{
        access::{Access, FromAccess},
        MapIndex,
    },
};

use exonum_derive::{BinaryValue, FromAccess, ObjectHash};
use exonum_proto::ProtobufConvert;

use crate::proto;

// Wallet persists the information within a service.
#[derive(Clone, Debug, Serialize, Deserialize, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Wallet")]
pub struct Wallet {
    pub pub_key: PublicKey,
    pub name: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new(&public_key: &PublicKey, name: &str, balance: u64) -> Self {
        Self {
            pub_key: public_key,
            name: name.to_owned(),
            balance,
        }
    }

    pub fn increase(&self, amount: u64) -> Self {
        let balance = self.balance + amount;
        Self::new(&self.pub_key, &self.name, balance)
    }

    pub fn decrease(&self, amount: u64) -> Self {
        let balance = self.balance - amount;
        Self::new(&self.pub_key, &self.name, balance)
    }
}

#[derive(Debug, FromAccess)]
pub struct CurrencySchema<T: Access> {
    pub wallets: MapIndex<T::Base, PublicKey, Wallet>,
}

impl<T: Access> CurrencySchema<T> {
    pub fn new(access: T) -> Self {
        Self::from_root(access).unwrap()
    }
}
