use money::{currency::Currency, money::Money};
use rust_decimal::prelude::*;

#[test]
fn test_integration_money_operations() {
    let usd_currency = Currency::USD;
    let eur_currency = Currency::EUR;

    let money1 = Money::new(Decimal::new(200, 0), usd_currency.clone());
    let money2 = Money::new(Decimal::new(100, 0), usd_currency.clone());
    let money2_clone = money2.clone();
    let result = (money1 + money2).unwrap();

    assert_eq!(result.amount(), Decimal::new(300, 0));
    assert_eq!(result.currency(), &usd_currency);

    let money3 = Money::new(Decimal::new(50, 0), eur_currency.clone());
    let result_sub = money2_clone - money3;
    assert!(result_sub.is_err());
}

#[test]
fn test_integration_multiplication() {
    let gbp_currency = Currency::GBP;
    let money = Money::new(Decimal::new(100, 0), gbp_currency.clone());
    let result = money * Decimal::new(4, 0);

    assert_eq!(result.amount(), Decimal::new(400, 0));
    assert_eq!(result.currency(), &gbp_currency);
}
