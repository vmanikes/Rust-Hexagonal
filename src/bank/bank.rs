use crate::entity::{deposit, transaction, transfer, withdraw};
use crate::errors::error::Error;

pub trait Bank {
    fn deposit(deposit: &deposit::Deposit) -> Result<(), Error>;
    fn withdraw(withdraw: &withdraw::Withdraw) -> Result<(), Error>;
    fn transfer(transfer: &transfer::Transfer) -> Result<(), Error>;
    fn get_transactions(
        user_id: &String,
        filter_type: &String,
        filter_value: &String,
    ) -> Result<Vec<transaction::Transaction>, Error>;
}

// TODO How to use context as first param and cancellation
