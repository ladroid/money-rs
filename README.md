# Money library

Simple money dealing library

## Usage

A Money object is created by supplying an amount and a currency. Amounts can be specified in string types but will be stored as precise decimals internally. Here's a quick example of how to use

```rust
use money;

fn main() {
    let money1 = money::Money::new(10, 50, "USD", "en-US", "USD");
    let money2 = money::Money::new(5, 25, "USD", "en-US", "USD");
    let money3 = money1.add(&money2);
    println!("{}", money3.format()); // Prints "16.75 USD"

    let money4 = money::Money::new(20, 0, "EUR", "fr-FR", "EUR");
    let money5 = money4.multiply(2.0);
    println!("{}", money5.format()); // Prints "40.00 €"

    let money6 = money::Money::new(10, 0, "JPY", "ja-JP", "JPY");
    let money7 = money6.divide(3.0);
    println!("{}", money7.format()); // Prints "3.33 ¥"
}
```