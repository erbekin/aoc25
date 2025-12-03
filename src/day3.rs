use std::str::FromStr;

pub fn solve() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut total_joltage = 0;
    for bank in &input {
        total_joltage += bank.max_joltage_part1();
    }
    println!("Total output joltage (PART 1): {}", total_joltage);

    let mut total_joltage = 0;
    for bank in &input {
        total_joltage += bank.max_joltage_part2();
    }
    println!("Total output joltage (PART 2): {}", total_joltage);
    Ok(())
}

/// Represents line of batteries, each battery is 1-9 ascii digit.
struct Bank {
    batteries: Vec<u8>,
    len : usize,
}
impl FromStr for Bank {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut batteries = Vec::new();
        for b in s.bytes() {
            if b.is_ascii_digit() && b != b'0' {
                batteries.push(b - b'0');
            } else {
                anyhow::bail!("invalid battery digit in line: '{}'", b);
            }
        }
        let len = batteries.len();
        Ok(Self { batteries, len })
    }
}
impl Bank {
    /// Calculates and returns max joltage as specified in part 1.
    fn max_joltage_part1(&self) -> u32 {
        if self.len == 1 {
            return self.batteries[0] as u32;
        }
        if self.len == 0 {
            return 0;
        }
        // find max, the first in equality
        let (pos, max_left) = find_first_max(&self.batteries[..(self.len - 1)]).unwrap();
        let &max_right = self.batteries[(pos + 1)..].iter().max().unwrap();
        (*max_left as u32) * 10 + (max_right as u32)
    }
    //987654321111111
    /// Calculates max joltage, as specified in part 2.
    fn max_joltage_part2(&self) -> u64 {
        assert!(self.len > 12, "In part2, Bank length should be greater than 12");
        let mut bats = Vec::new();
        let mut pos : Option<usize> = None;
        for offset in (0..12).rev() {
            let start = pos.map(|p| p + 1).unwrap_or(0);
            let slice = &self.batteries[start..(self.len - offset)];
            let (max_pos, max_val) = find_first_max(slice).unwrap(); // SAFETY: slice not empty
            bats.push(*max_val as u64);
            pos = Some(max_pos + start);
        }
        bats.into_iter()
            .fold(0, |acc, digit| acc * 10 + digit)
    }
}

fn read_input() -> anyhow::Result<Vec<Bank>> {
    std::fs::read_to_string("inputs/input3.txt")?
        .lines()
        .map(|line| line.parse()) // parse returns Result
        .collect() // collect handles Result<Vec> -> Vec<Result> transformation automagically
}

fn find_first_max<T : Ord>(slice : &[T]) -> Option<(usize, &T)> {
    let max = slice.iter().max()?;
    let pos = slice.iter().position(|i| i == max)?;
    Some((pos, max))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_joltage() {
        let bank : Bank = "987654321111111".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 98);
        let bank : Bank = "811111111111119".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 89);
        let bank : Bank = "234234234234278".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 78);
        let bank : Bank = "111111111111111".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 11);
        let bank : Bank = "123456789".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 89);
        let bank : Bank = "9191".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 99);
        let bank : Bank = "12".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 12);
        let bank : Bank = "1".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 1);
        let bank : Bank = "".parse().unwrap();
        assert_eq!(bank.max_joltage_part1(), 0);
    }
    #[test]
    fn test_joltage_p2() {
        let bank : Bank = "987654321111111".parse().unwrap();
        assert_eq!(bank.max_joltage_part2(), 987654321111);

        let bank : Bank = "811111111111119".parse().unwrap();
        assert_eq!(bank.max_joltage_part2(), 811111111119);

        let bank : Bank = "234234234234278".parse().unwrap();
        assert_eq!(bank.max_joltage_part2(), 434234234278);

        let bank : Bank = "818181911112111".parse().unwrap();
        assert_eq!(bank.max_joltage_part2(), 888911112111);

    }
}
