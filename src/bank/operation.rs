use super::bank::Bank;
use crate::entity::deposit::Deposit;
use crate::entity::transaction::Transaction;
use crate::entity::transfer::Transfer;
use crate::entity::withdraw::Withdraw;
use crate::errors::error::Error;

struct BankImpl {}

impl Bank for BankImpl {
    fn deposit(deposit: &Deposit) -> Result<(), Error> {
        todo!()
    }

    fn withdraw(withdraw: &Withdraw) -> Result<(), Error> {
        todo!()
    }

    fn transfer(transfer: &Transfer) -> Result<(), Error> {
        todo!()
    }

    fn get_transactions(
        user_id: &String,
        filter_type: &String,
        filter_value: &String,
    ) -> Result<Vec<Transaction>, Error> {
        todo!()
    }
}
