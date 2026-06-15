fn score_grade(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "D"
    }
}

fn sum_to(limit: u32) -> u32 {
    let mut total = 0;
    for value in 1..=limit {
        total += value;
    }
    total
}

fn countdown(start: u32) -> Vec<u32> {
    let mut values = Vec::new();
    let mut current = start;
    while current > 0 {
        values.push(current);
        current -= 1;
    }
    values
}

fn count_until(target: u32) -> u32 {
    let mut count = 0;
    loop {
        count += 1;
        if count == target {
            break count;
        }
    }
}

fn main() {
    println!("score 92 => {}", score_grade(92));
    println!("sum 1..=5 => {}", sum_to(5));
    println!("countdown: {:?}", countdown(3));
    println!("loop result: {}", count_until(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_scores() {
        assert_eq!(score_grade(92), "A");
        assert_eq!(score_grade(82), "B");
        assert_eq!(score_grade(70), "C");
        assert_eq!(score_grade(60), "D");
    }

    #[test]
    fn sums_range() {
        assert_eq!(sum_to(5), 15);
    }

    #[test]
    fn creates_countdown() {
        assert_eq!(countdown(3), vec![3, 2, 1]);
    }

    #[test]
    fn loop_returns_value() {
        assert_eq!(count_until(4), 4);
    }
}
