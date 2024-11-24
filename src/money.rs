use crate::currency::Currency;
use rust_decimal::Decimal;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    fn ensure_same_currency(&self, other: &Money) -> Result<(), String> {
        if self.currency != other.currency {
            Err(format!(
                "Currency mismatch: {} vs {}",
                self.currency.code, other.currency.code
            ))
        } else {
            Ok(())
        }
    }
}

impl Add for Money {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Result<Self, String> {
        self.ensure_same_currency(&other)?;
        Ok(Self {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }
}

impl Sub for Money {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Result<Self, String> {
        self.ensure_same_currency(&other)?;
        Ok(Self {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }
}

impl Mul<Decimal> for Money {
    type Output = Self;

    fn mul(self, scalar: Decimal) -> Self {
        Self {
            amount: self.amount * scalar,
            currency: self.currency.clone(),
        }
    }
}

impl Mul for Money {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Result<Self, String> {
        if self.currency != other.currency {
            Err(format!(
                "Currency mismatch: {} vs {}",
                self.currency.code, other.currency.code
            ))
        } else {
            Ok(Self {
                amount: self.amount * other.amount,
                currency: self.currency.clone(),
            })
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:.2}", self.currency.symbol, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_money_creation() {
        let usd = Money::new(Decimal::new(100, 2), Currency::USD); // $1.00
        assert_eq!(usd.amount(), Decimal::new(100, 2));
        assert_eq!(usd.currency().code, "USD");
        assert_eq!(usd.currency().symbol, "$");
    }

    #[test]
    fn test_money_addition_success() {
        let usd1 = Money::new(Decimal::new(100, 2), Currency::USD); // $1.00
        let usd2 = Money::new(Decimal::new(200, 2), Currency::USD); // $2.00
        let result = usd1 + usd2;
        assert!(result.is_ok());
        let money = result.unwrap();
        assert_eq!(money.amount(), Decimal::new(300, 2)); // $3.00
        assert_eq!(money.currency().code, "USD");
    }

    #[test]
    fn test_money_addition_currency_mismatch() {
        let usd = Money::new(Decimal::new(100, 2), Currency::USD); // $1.00
        let eur = Money::new(Decimal::new(100, 2), Currency::EUR); // €1.00
        let result = usd + eur;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Currency mismatch: USD vs EUR");
    }

    #[test]
    fn test_money_subtraction_success() {
        let usd1 = Money::new(Decimal::new(300, 2), Currency::USD); // $3.00
        let usd2 = Money::new(Decimal::new(200, 2), Currency::USD); // $2.00
        let result = usd1 - usd2;
        assert!(result.is_ok());
        let money = result.unwrap();
        assert_eq!(money.amount(), Decimal::new(100, 2)); // $1.00
        assert_eq!(money.currency().code, "USD");
    }

    #[test]
    fn test_money_subtraction_currency_mismatch() {
        let usd = Money::new(Decimal::new(100, 2), Currency::USD); // $1.00
        let eur = Money::new(Decimal::new(100, 2), Currency::EUR); // €1.00
        let result = usd - eur;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Currency mismatch: USD vs EUR");
    }

    #[test]
    fn test_money_scalar_multiplication() {
        let usd = Money::new(Decimal::new(100, 2), Currency::USD); // $1.00
        let result = usd * Decimal::new(3, 0); // $1.00 * 3 = $3.00
        assert_eq!(result.amount(), Decimal::new(300, 2)); // $3.00
        assert_eq!(result.currency().code, "USD");
    }

    #[test]
    fn test_money_multiplication_success() {
        let usd1 = Money::new(Decimal::new(200, 2), Currency::USD); // $2.00
        let usd2 = Money::new(Decimal::new(300, 2), Currency::USD); // $3.00
        let result = usd1 * usd2;
        assert!(result.is_ok());
        let money = result.unwrap();
        assert_eq!(money.amount(), Decimal::new(60000, 4)); // $6.0000
        assert_eq!(money.currency().code, "USD");
    }

    #[test]
    fn test_money_multiplication_currency_mismatch() {
        let usd = Money::new(Decimal::new(100, 2), Currency::USD); // $1.00
        let eur = Money::new(Decimal::new(100, 2), Currency::EUR); // €1.00
        let result = usd * eur;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Currency mismatch: USD vs EUR");
    }

    #[test]
    fn test_money_display() {
        let usd = Money::new(Decimal::new(12345, 2), Currency::USD); // $123.45
        let eur = Money::new(Decimal::new(67890, 2), Currency::EUR); // €678.90
        let jpy = Money::new(Decimal::new(5000, 0), Currency::JPY); // ¥5000

        assert_eq!(format!("{}", usd), "$ 123.45");
        assert_eq!(format!("{}", eur), "€ 678.90");
        assert_eq!(format!("{}", jpy), "¥ 5000.00");
    }

    #[test]
    fn test_money_zero_amount() {
        let usd = Money::new(Decimal::new(0, 2), Currency::USD); // $0.00
        assert_eq!(usd.amount(), Decimal::new(0, 2));
        assert_eq!(usd.currency().code, "USD");
        assert_eq!(usd.currency().symbol, "$");
    }

    #[test]
    fn test_money_large_values() {
        let usd = Money::new(Decimal::new(1_000_000_000, 2), Currency::USD); // $10,000,000.00
        let scaled = usd * Decimal::new(2, 0); // $10,000,000.00 * 2 = $20,000,000.00
        assert_eq!(scaled.amount(), Decimal::new(2_000_000_000, 2));
        assert_eq!(scaled.currency().code, "USD");
    }

    #[test]
    fn test_money_negative_values() {
        let usd = Money::new(Decimal::new(-500, 2), Currency::USD); // -$5.00
        assert_eq!(usd.amount(), Decimal::new(-500, 2));
        assert_eq!(usd.currency().code, "USD");
    }
}
