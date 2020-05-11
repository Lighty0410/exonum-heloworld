use exonum::runtime::{ExecutionContext, ExecutionError};
use exonum_derive::{exonum_interface, interface_method, ServiceDispatcher, ServiceFactory};
use exonum_rust_runtime::{api::ServiceApiBuilder, DefaultInstance, Service};

use crate::{
    api::CryptocurrencyApi,
    error::Error,
    schema::{CurrencySchema, Wallet},
    transactions::{CreateWallet, TxTransfer},
};

const INIT_BALANCE: u64 = 100;

#[exonum_interface]
pub trait CryptocurrencyInterface<Ctx> {
    type Output;

    #[interface_method(id = 0)]
    fn create_wallet(&self, ctx: Ctx, arg: CreateWallet) -> Self::Output;

    #[interface_method(id = 1)]
    fn transfer(&self, ctx: Ctx, arg: TxTransfer) -> Self::Output;
}

#[derive(Debug, ServiceFactory, ServiceDispatcher)]
#[service_dispatcher(implements("CryptocurrencyInterface"))]
#[service_factory(proto_sources = "crate::proto")]
pub struct CryptocurrencyService;

impl CryptocurrencyInterface<ExecutionContext<'_>> for CryptocurrencyService {
    type Output = Result<(), ExecutionError>;

    fn create_wallet(&self, ctx: ExecutionContext<'_>, arg: CreateWallet) -> Self::Output {
        let author = ctx
            .caller()
            .author()
            .expect("Wrong `TxCreateWallet` initiator");

        let mut schema = CurrencySchema::new(ctx.service_data());

        if schema.wallets.get(&author).is_some() {
            return Err(Error::WalletAlreadyExists.into());
        }

        let wallet = Wallet::new(&author, &arg.name, INIT_BALANCE);
        println!("Created wallet: {:?}", wallet);
        schema.wallets.put(&author, wallet);
        Ok(())
    }

    fn transfer(&self, ctx: ExecutionContext<'_>, arg: TxTransfer) -> Self::Output {
        let author = ctx.caller().author().expect("Wrong txTransfer initiator");

        if author == arg.to {
            return Err(Error::SenderSameAsReceiver.into());
        }

        let mut schema = CurrencySchema::new(ctx.service_data());

        let sender = schema.wallets.get(&author).ok_or(Error::SenderNotFound)?;
        let receiver = schema.wallets.get(&arg.to).ok_or(Error::ReceiverNotFound)?;

        let amount = arg.amount;

        if sender.balance < amount {
            return Err(Error::InsufficientCurrencyAmount.into());
        }

        let sender = sender.decrease(amount);
        let receiver = receiver.increase(amount);

        println!("Transfer between wallets: {:?} -> {:?}", sender, receiver);

        schema.wallets.put(&author, sender);
        schema.wallets.put(&arg.to, receiver);

        Ok(())
    }
}

impl Service for CryptocurrencyService {
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        CryptocurrencyApi::wire(builder);
    }
}

impl DefaultInstance for CryptocurrencyService {
    const INSTANCE_ID: u32 = 101;
    const INSTANCE_NAME: &'static str = "cryptocurrency";
}
