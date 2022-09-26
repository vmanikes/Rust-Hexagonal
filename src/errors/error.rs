pub enum Error {
    UnableToBeginTransaction(u32),
    UnableToCommitTransaction(u32),
    UnableToUpdateBalance(u32),
    UnableToCreateBankTransaction(u32),
    UnableToFetchTransactions(u32),
    UnableToScanRows(u32),
}

pub const ERR_UNABLE_TO_BEGIN_TRANSACTION: Error = Error::UnableToBeginTransaction(1000);
pub const ERR_UNABLE_TO_COMMIT_TRANSACTION: Error = Error::UnableToCommitTransaction(1000);
pub const ERR_UNABLE_TO_UPDATE_BALANCE: Error = Error::UnableToUpdateBalance(1000);
pub const ERR_UNABLE_TO_CREATE_BANK_TRANSACTION: Error = Error::UnableToCreateBankTransaction(1000);
pub const ERR_UNABLE_TO_FETCH_TRANSACTIONS: Error = Error::UnableToFetchTransactions(1000);