# Money library

Simple money dealing library

## Usage

A Money object is created by supplying an amount and a currency. Amounts can be specified in string types but will be stored as precise decimals internally. Here's a quick example of how to use

```rust
use money;

fn main() {
    let m1 = money::Money::new(100, 50, "USD");
    let m2 = money::Money::new(200, 75, "USD");
    let m3 = m1.add(&m2);
    println!("{}", m3.format()); // prints "301.25 USD"
    
    let m4 = m3.subtract(&m1);
    println!("{}", m4.format()); // prints "200.75 USD"
    
    let m5 = m4.multiply(0.5);
    println!("{}", m5.format()); // prints "100.38 USD"
    
    let m6 = m5.divide(2.0);
    println!("{}", m6.format()); // prints "50.19 USD"
    
    let m7 = money::Money::from_currency(123.45, "USD");
    println!("{}", m7.format()); // prints "123.45 USD"
}
```