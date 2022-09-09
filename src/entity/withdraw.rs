use rust_decimal::Decimal;

pub struct Withdraw {
    pub amount: Decimal,
    pub account_number: i64,
    pub user_id: String,
}
