#[derive(Debug, PartialEq)]
pub struct Money {
    amount: i32,
    cents: i32,
    currency: String,
}

impl Money {
    pub fn new(amount: i32, cents: i32, currency: &str) -> Money {
        Money {
            amount,
            cents,
            currency: currency.to_string(),
        }
    }

    pub fn from_currency(amount: f32, currency: &str) -> Money {
        let mut cents = (amount * 100.0) as i32;
        let mut amount = 0;
        if cents >= 100 {
            amount += cents / 100;
            cents %= 100;
        }
        Money {
            amount,
            cents,
            currency: currency.to_string(),
        }
    }

    pub fn add(&self, other: &Money) -> Money {
        let mut amount = self.amount + other.amount;
        let mut cents = self.cents + other.cents;
        if cents >= 100 {
            amount += cents / 100;
            cents %= 100;
        }
        Money {
            amount,
            cents,
            currency: self.currency.clone(),
        }
    }

    pub fn subtract(&self, other: &Money) -> Money {
        let mut amount = self.amount - other.amount;
        let mut cents = self.cents - other.cents;
        if cents < 0 {
            amount -= 1;
            cents += 100;
        }
        Money {
            amount,
            cents,
            currency: self.currency.clone(),
        }
    }

    pub fn multiply(&self, factor: f32) -> Money {
        let mut amount = (self.amount as f32 * factor) as i32;
        let mut cents = (self.cents as f32 * factor) as i32;
        if cents >= 100 {
            amount += cents / 100;
            cents %= 100;
        }
        Money {
            amount,
            cents,
            currency: self.currency.clone(),
        }
    }

    pub fn divide(&self, divisor: f32) -> Money {
        let mut amount = (self.amount as f32 / divisor) as i32;
        let mut cents = (self.cents as f32 / divisor) as i32;
        if cents >= 100 {
            amount += cents / 100;
            cents %= 100;
        }
        Money {
            amount,
            cents,
            currency: self.currency.clone(),
        }
    }

    pub fn format(&self) -> String {
        format!("{}.{:02} {}", self.amount, self.cents, self.currency)
    }
}
