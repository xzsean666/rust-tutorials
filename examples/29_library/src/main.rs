//! 二进制部分:通过 crate 名引用同包的库。

use rt_29_library::{celsius_to_fahrenheit, fahrenheit_to_celsius};

fn main() {
    println!("0°C = {}°F", celsius_to_fahrenheit(0.0));
    println!("212°F = {}°C", fahrenheit_to_celsius(212.0));
}
