const TOTAL_STAGES: u32 = 16;

fn advance_stage(current: u32) -> u32 {
    let current = current + 1;
    current.min(TOTAL_STAGES)
}

fn completed_message(completed_examples: u32) -> String {
    format!("已完成示例: {completed_examples}")
}

fn main() {
    let current_stage = 1;
    let current_stage = advance_stage(current_stage);

    let mut completed_examples = 0;
    completed_examples += 1;

    println!("当前阶段: {current_stage}/{TOTAL_STAGES}");
    println!("{}", completed_message(completed_examples));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_does_not_exceed_total() {
        assert_eq!(advance_stage(16), 16);
    }

    #[test]
    fn formats_completed_message() {
        assert_eq!(completed_message(3), "已完成示例: 3");
    }
}
