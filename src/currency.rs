use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    pub code: &'static str,
    pub symbol: &'static str,
}

impl Currency {
    pub fn new(code: &'static str, symbol: &'static str) -> Self {
        Self { code, symbol }
    }

    pub const GBP: Self = Self {
        code: "GBP",
        symbol: "£",
    };

    pub const USD: Self = Self {
        code: "USD",
        symbol: "$",
    };

    pub const EUR: Self = Self {
        code: "EUR",
        symbol: "€",
    };

    pub const CNY: Self = Self {
        code: "CNY",
        symbol: "元",
    };

    pub const JPY: Self = Self {
        code: "JPY",
        symbol: "¥",
    };
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.symbol, self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_constants() {
        assert_eq!(Currency::USD.code, "USD");
        assert_eq!(Currency::USD.symbol, "$");
    }

    #[test]
    fn test_custom_currency_creation() {
        let btc = Currency::new("BTC", "₿");
        assert_eq!(btc.code, "BTC");
        assert_eq!(btc.symbol, "₿");
    }
}
