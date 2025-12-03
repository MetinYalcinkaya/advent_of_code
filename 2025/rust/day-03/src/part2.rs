#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut nums: Vec<u32> = vec![];

    for line in lines {
        let (mut first_num, mut pos) = (0, 0);
        let mut sec_num = 0;
        for (index, byte) in line.bytes().enumerate() {
            if byte > first_num && index >= pos && index != line.len() - 1 {
                (first_num, pos) = (byte, index);
            }
        }

        for byte in line[pos + 1..].bytes() {
            if byte > sec_num {
                sec_num = byte;
            }
        }

        nums.push(
            (format!("{}{}", first_num as char, sec_num as char))
                .parse()
                .unwrap(),
        );
    }
    let sum: u32 = nums.iter().sum();
    Ok(sum.to_string())
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
