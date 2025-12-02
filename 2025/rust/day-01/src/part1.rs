#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let dirs: Vec<(&str, i32)> = input
        .lines()
        .map(|s| {
            let iter = s.split_at(1);
            let (first, second) = (iter.0, iter.1.parse::<i32>().unwrap());
            (first, second)
        })
        .collect();
    let mut dial = Dial { current: 50 };
    let mut zero_count: i32 = 0;
    dbg!(&dirs);

    for (dir, num) in dirs {
        if dir == "L" {
            dial.turn_left(num);
        } else {
            dial.turn_right(num);
        }

        if dial.current == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count.to_string())
}

// dial has bounds: (0, 99)
struct Dial {
    current: i32,
}

impl Dial {
    fn turn_left(&mut self, num: i32) {
        let mut dial = self.current;
        dial = (dial - num).rem_euclid(100);
        self.current = dial;
    }

    fn turn_right(&mut self, num: i32) {
        let mut dial = self.current;
        dial = (dial + num).rem_euclid(100);
        self.current = dial;
    }
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
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
