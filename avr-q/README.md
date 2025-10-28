# Fixed point arithmetic for AVR

This library provides
[fixed-point arithmetic types](https://en.wikipedia.org/wiki/Q_(number_format))
for AVR microcontrollers.

## The fixed point formats supported are

- **Q7.8 format**: A 16-bit fixed-point number with 7 integer bits and 8 fractional bits (`Q7p8`).
- **Q15.8 format**: A 24-bit fixed-point number with 15 integer bits and 8 fractional bits (`Q15p8`).

## The supported operations are

- Basic arithmetic operations: addition, subtraction, multiplication, and division.
- Macros for easy construction of fixed-point numbers from integers or fractions.
- Conversions between fixed-point types and integer types.
- Optional `curveipo` feature for curve interpolation.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
avr-q = "1"
```

## Examples

### Creating fixed-point numbers

```rust
use avr_q::{q7p8, Q7p8};

// From an integer
let a = q7p8!(const 5);
assert_eq!(a.to_int(), 5);

// From a fraction
let b = q7p8!(const 1 / 2); // 0.5
let c = q7p8!(const 10 / 3); // 3.33...

// From variables
let numerator = 10_i16;
let denominator = 3_i16;
let d = q7p8!(numerator / denominator);
```

### Arithmetic operations

```rust
use avr_q::{q7p8, Q7p8};

let a = q7p8!(const 1 / 2); // 0.5
let b = q7p8!(const 1 / 4); // 0.25

let sum = a + b;
assert_eq!(sum, q7p8!(const 3 / 4)); // 0.75

let diff = a - b;
assert_eq!(diff, q7p8!(const 1 / 4)); // 0.25

let prod = a * b;
assert_eq!(prod, q7p8!(const 1 / 8)); // 0.125

let quot = a / b;
assert_eq!(quot, q7p8!(const 2)); // 2.0

let neg = -a;
assert_eq!(neg, q7p8!(const -1 / 2));

let abs = neg.abs();
assert_eq!(abs, q7p8!(const 1 / 2));
```

## License

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
