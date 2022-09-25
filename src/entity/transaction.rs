use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub transaction_type: TransactionType,
    pub balance: Decimal,
    pub amount: Decimal,
    pub created_on: DateTime<Utc>,
}

pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}
