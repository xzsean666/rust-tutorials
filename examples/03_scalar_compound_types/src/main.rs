fn temperature_report(celsius: f64) -> (f64, bool) {
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    let is_hot = celsius >= 30.0;
    (fahrenheit, is_hot)
}

fn first_and_last(values: [i32; 4]) -> (i32, i32) {
    (values[0], values[values.len() - 1])
}

fn main() {
    let celsius = 30.0;
    let (fahrenheit, is_hot) = temperature_report(celsius);
    let (first, last) = first_and_last([10, 20, 30, 40]);

    println!("{celsius:.0}°C = {fahrenheit:.0}°F, hot: {is_hot}");
    println!("first = {first}, last = {last}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_temperature() {
        let (fahrenheit, is_hot) = temperature_report(30.0);
        assert!((fahrenheit - 86.0).abs() < f64::EPSILON);
        assert!(is_hot);
    }

    #[test]
    fn reads_array_edges() {
        assert_eq!(first_and_last([1, 2, 3, 4]), (1, 4));
    }
}
