use rust_decimal::Decimal;

pub struct Transfer {
    pub amount: Decimal,
    pub current_account_number: i64,
    pub destination_account_number: i64,
}
