#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let res: u64 = lines
        .iter()
        .map(|&b| {
            let mut max = 0;
            let mut start = 0;
            (0..12).fold(0, |j, d| {
                let end = b.len() - 12 + d + 1;
                (max, start) = (start..end).fold((0, 0), |(max, start), i| {
                    if b[i] > max {
                        (b[i], i + 1)
                    } else {
                        (max, start)
                    }
                });
                10 * j + (max - b'0') as u64
            })
        })
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
