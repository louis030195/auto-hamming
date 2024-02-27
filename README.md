
# AutoHamming

AutoHamming is a Rust crate that ensures the integrity of your data by automatically applying Hamming code protection. With AutoHamming, every variable is seamlessly encoded with Hamming code upon assignment and decoded upon access, providing error detection and correction without any manual intervention.

## Features

- Automatic error correction and detection for your data.
- Seamless integration into Rust projects with minimal changes.
- Custom Rust types that handle encoding and decoding behind the scenes.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto_hamming = "0.1.0"
```

## Quick Start

### Using AutoHamming

Import and use the provided types from AutoHamming to define your variables. These types will automatically apply Hamming code protection.

```rust
use auto_hamming::ProtectedInt;

fn main() {
    // Define a protected integer
    let my_protected_int = ProtectedInt::new(15);

    // Use the protected integer as normal
    println!("Protected value + 10 = {}", my_protected_int.get() + 10);

    // Behind the scenes, AutoHamming encodes and decodes the integer,
    // providing automatic error detection and correction.
}
```

## How It Works

AutoHamming provides custom Rust types (e.g., `ProtectedInt`) that override the default behavior for storing and retrieving values. When you assign a value to a variable of these custom types, AutoHamming encodes the value using Hamming code. When you access the variable, AutoHamming decodes the value, correcting any errors that can be fixed.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest features.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
