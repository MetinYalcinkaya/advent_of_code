#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .split(',')
        .map(|r| {
            let (lower, upper) = r.split_once('-').unwrap();
            let lower: u64 = lower.trim().parse().unwrap();
            let upper: u64 = upper.trim().parse().unwrap();
            (lower..=upper)
                .filter(|&num| {
                    let num_str = num.to_string();
                    if num_str.len() % 2 != 0 {
                        return false;
                    }
                    let mid = num_str.len() / 2;
                    let (left, right) = num_str.split_at(mid);
                    left == right
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
