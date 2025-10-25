# base10_exponent

A Rust library for computing the base-10 exponent and digit count of floating-point numbers (f32 and f64).

## Features

- **Exponent calculation**: Compute the base-10 exponent of floating-point numbers.
- **Digit count**: Determine the number of digits in the exponent.
- **Support for f32 and f64**: Works with both single and double precision floating-point types.
- **No dependencies**: Lightweight and self-contained.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
base10_exponent = "0.1.0"
```

## Usage

### Basic Exponent Calculation

```rust
use base10_exponent::{exp10, exp10_digits, exp10_with_digits};

fn main() {
    // Get the base-10 exponent
    let exp = exp10::<f64>(100.0); // Returns 2
    println!("Exponent of 100.0: {}", exp);

    // Get the digit count of the exponent
    let digits = exp10_digits::<f64>(100.0); // Returns 1
    println!("Digits in exponent: {}", digits);

    // Get both exponent and digits
    let (exp, digits) = exp10_with_digits::<f64>(100.0); // Returns (2, 1)
    println!("Exponent: {}, Digits: {}", exp, digits);
}
```

### Examples

```rust
use base10_exponent::exp10;

// Large numbers
assert_eq!(exp10::<f64>(1e100), 100);

// Small numbers
assert_eq!(exp10::<f64>(0.01), -2);

// Negative numbers (absolute value is used)
assert_eq!(exp10::<f64>(-1000.0), 3);

// Special values
assert_eq!(exp10::<f64>(0.0), 0);
assert_eq!(exp10::<f64>(f64::INFINITY), 0);
assert_eq!(exp10::<f64>(f64::NAN), 0);
```

## API Reference

- `exp10<F: FloatOps>(x: F) -> i32`: Returns the base-10 exponent of the absolute value of `x`.
- `exp10_digits<F: FloatOps>(x: F) -> u32`: Returns the number of digits in the base-10 exponent.
- `exp10_with_digits<F: FloatOps>(x: F) -> (i32, u32)`: Returns both the exponent and digit count.

Where `F` is either `f32` or `f64`.

## Performance

This library uses precomputed tables and bit manipulation for efficient computation, making it suitable for performance-critical applications.

### Benchmark Results

Benchmarks were run using `criterion` comparing `base10_exponent::exp10` with the standard `log10` method. Note that while `base10_exponent` provides exact integer exponents, the standard `log10` is optimized at the hardware level and may be faster for simple floating-point operations.

- **base10_exponent::exp10<f64>**: ~12ns per operation
- **Standard log10<f64>**: ~1.7ns per operation
- **base10_exponent::exp10<f32>**: ~9.4ns per operation
- **Standard log10<f32>**: ~1.3ns per operation

The standard `log10` function benefits from CPU-level optimizations, which can make it faster despite the table-based approach of `base10_exponent`. However, `base10_exponent` is designed for scenarios where precise integer exponent calculation is needed without the overhead of floating-point logarithms.

**Note**: While the library employs interesting techniques like `LazyLock` for lazy initialization and table lookups for efficiency, it may not always outperform hardware-optimized functions like `log10`. The library focuses on algorithmic efficiency and providing exact integer exponents without runtime floating-point operations.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
