use exonum_derive::ExecutionFail;

#[derive(Debug, ExecutionFail)]
pub enum Error {
    WalletAlreadyExists = 0,
    SenderNotFound = 1,
    ReceiverNotFound = 2,
    InsufficientCurrencyAmount = 3,
    SenderSameAsReceiver = 4,
}
