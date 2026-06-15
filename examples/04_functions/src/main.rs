fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}

fn passing_label(score: u32) -> &'static str {
    if score >= 60 { "pass" } else { "retry" }
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
    println!("area = {}", rectangle_area(4, 6));
    println!("score 88 => {}", passing_label(88));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn calculates_area() {
        assert_eq!(rectangle_area(4, 6), 24);
    }

    #[test]
    fn labels_passing_score() {
        assert_eq!(passing_label(88), "pass");
        assert_eq!(passing_label(59), "retry");
    }
}
