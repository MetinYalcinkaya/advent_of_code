#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ranges = input.split(',').map(|r| {
        let mut iter = r.split('-');
        (iter.next().unwrap(), iter.next().unwrap())
    });
    let mut invalid_ids = 0;
    for range in ranges {
        let lower = range.0.trim().parse::<u64>().unwrap();
        let upper = range.1.trim().parse::<u64>().unwrap();
        for i in lower..upper + 1 {
            let num_str = i.to_string();
            let len = &num_str.len();
            if len % 2 == 0 {
                let mid = len / 2;
                let first_half = &num_str[0..mid];
                let second_half = &num_str[mid..*len];
                // invalid num
                if first_half == second_half {
                    invalid_ids += i;
                }
            } else {
                continue;
            }
        }
    }
    Ok(invalid_ids.to_string())
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
