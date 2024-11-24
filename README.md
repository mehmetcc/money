# Money

This library provides an implementation of the Money Pattern in Rust, based on the [Money Pattern defined by Martin Fowler](https://martinfowler.com/eaaCatalog/money.html), supporting various currencies, operator overloading for common arithmetic operations, and user-defined currencies.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
money = { git = "https://github.com/mehmetcc/money" }
```

## Example Usage

```rust
use money::{Money, Currency, Decimal};

fn main() {
    // Creating a new Currency for Bitcoin
    let bitcoin = Currency::new("BTC", "₿");
    let usd_currency = Currency::USD;
    let eur_currency = Currency::EUR;
    let gbp_currency = Currency::GBP;

    let money1 = Money::new(Decimal::new(100, 0), usd_currency);
    let money2 = Money::new(Decimal::new(50, 0), Currency::USD);
    let money3 = (money1.clone() + money2.clone()).unwrap();
    let money4 = (money3.clone() - money2.clone()).unwrap();
    let money5 = money2.clone() * Decimal::new(3, 0);

    println!("Money 1: {}", money1);
    println!("Money 2: {}", money2);
    println!("Money 3 (Money 1 + Money 2): {}", money3);
    println!("Money 4 (Money 3 - Money 2): {}", money4);
    println!("Money 5 (Money 2 * 3): {}", money5);

    // Example with different currencies
    let money6 = Money::new(Decimal::new(100, 0), eur_currency);
    println!("Money 6: {}", money6);

    // Example with custom Bitcoin currency
    let btc_money = Money::new(Decimal::new(1, 0), bitcoin);
    println!("Bitcoin Money: {}", btc_money);
}
```

Command to run tests:
`cargo test`
