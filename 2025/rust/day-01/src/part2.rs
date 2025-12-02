#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut zero_count = 0;
    let mut dial = 50;
    for dir in input.lines() {
        let num = dir[1..].parse::<i32>().unwrap();
        if dir.starts_with("L") {
            zero_count += (num / 100).abs();
            if dial != 0 && num % 100 >= dial {
                zero_count += 1;
            }
            dial = (dial - num).rem_euclid(100);
        } else {
            dial += num;
            zero_count += dial / 100;
            dial = dial.rem_euclid(100);
        }
    }

    Ok(zero_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
