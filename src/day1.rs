
pub fn solve() -> anyhow::Result<()> {
    println!("Reading input.");
    let input = read_input()?;
    let mut dial = Dial::new();
    let mut zero_count : u32 = 0;
    for rot in &input {
        dial.rotate(*rot);
        if dial.at() == 0 {
            zero_count += 1;
        }
    }
    println!("ALl rotations completed.");
    println!("Dial pointed zero for {} times", zero_count);

    // ========= PART 2 ============== //
    println!("Rotating again using password method 0x434C49434B");
    let mut dial = Dial::new();
    let mut total_zero_count = 0;
    for rot in &input {
        total_zero_count += dial.rotate_and_count(*rot);
    }
    println!("Total zero count: {}", total_zero_count);
    Ok(())
}

const DIAL_SIZE : u32 = 100;
const DIAL_INITIAL_POINT : u32 = 50;

#[derive(Debug)]
struct Dial {
    pointing : u32,
}

impl Dial {
    fn new() -> Self {
        Self { pointing: DIAL_INITIAL_POINT }
    }
    fn at(&self) -> u32 {
        self.pointing
    }
    fn rotate(&mut self, rotation : DialRotation) {
        match rotation {
            DialRotation::Left(n) => {
                let n = DIAL_SIZE - (n % DIAL_SIZE);
                self.pointing = (self.pointing + n) % DIAL_SIZE;
            }
            DialRotation::Right(n) => {
                let n = n % DIAL_SIZE;
                self.pointing = (self.pointing + n) % DIAL_SIZE;
            }
        };
    }
    fn rotate_and_count(&mut self, rotation : DialRotation) -> u32 {
        let complete_tours = rotation.count() / DIAL_SIZE;
        let prev_point = self.pointing;
        self.rotate(rotation);
        if prev_point == self.pointing {
            return complete_tours;
        }
        let mut zero_count = complete_tours;
        match rotation {
            DialRotation::Left(_) => {
                if prev_point < self.pointing && prev_point != 0 {
                    zero_count += 1;
                } else if self.pointing == 0 {
                    zero_count += 1;
                }
            }
            DialRotation::Right(_) => {
                if self.pointing < prev_point {
                    zero_count += 1;
                }
            }
        }
        zero_count
    }
}

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum DialRotation {
    Left(u32),
    Right(u32),
}

impl DialRotation {
    fn count(&self) -> u32 {
        match self {
            DialRotation::Left(n) => *n,
            DialRotation::Right(n) => *n,
        }
    }
}
fn read_input() -> anyhow::Result<Vec<DialRotation>> {
    let content = std::fs::read_to_string("inputs/input1.txt")?;
    let mut rots = Vec::new();
    for line in content.lines() {
        let count : u32 = line[1..].parse()?;
        if line.starts_with('R') {
            rots.push(DialRotation::Right(count));
        } else {
            rots.push(DialRotation::Left(count));
        }
    }
    Ok(rots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_right_rotation() {
        let mut dial = Dial::new(); // Starts at 50

        // 50 -> 60 (No zero crossing)
        let count = dial.rotate_and_count(DialRotation::Right(10));
        assert_eq!(dial.at(), 60);
        assert_eq!(count, 0, "Right 10 from 50 should not hit zero");
    }

    #[test]
    fn test_right_crossing_zero() {
        let mut dial = Dial { pointing: 90 };

        // 90 -> 10 (Crosses 0)
        let count = dial.rotate_and_count(DialRotation::Right(20));
        assert_eq!(dial.at(), 10);
        assert_eq!(count, 1, "Right 20 from 90 should hit zero once");
    }

    #[test]
    fn test_right_landing_on_zero() {
        let mut dial = Dial { pointing: 90 };

        // 90 -> 0 (Lands on 0)
        let count = dial.rotate_and_count(DialRotation::Right(10));
        assert_eq!(dial.at(), 0);
        assert_eq!(count, 1, "Landing on zero from right should count");
    }

    #[test]
    fn test_basic_left_rotation() {
        let mut dial = Dial { pointing: 20 };

        // 20 -> 10 (No zero crossing)
        let count = dial.rotate_and_count(DialRotation::Left(10));
        assert_eq!(dial.at(), 10);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_left_crossing_zero() {
        let mut dial = Dial { pointing: 10 };

        // 10 -> 90 (Crosses 0 going backwards)
        let count = dial.rotate_and_count(DialRotation::Left(20));
        assert_eq!(dial.at(), 90);
        assert_eq!(count, 1, "Left 20 from 10 should hit zero once");
    }

    #[test]
    fn test_left_landing_on_zero() {
        let mut dial = Dial { pointing: 10 };

        // 10 -> 0
        let count = dial.rotate_and_count(DialRotation::Left(10));
        assert_eq!(dial.at(), 0);
        assert_eq!(count, 1, "Landing on zero from left should count");
    }

    #[test]
    fn test_left_from_zero_boundary() {
        // This is the bug we fixed!
        let mut dial = Dial { pointing: 0 };

        // 0 -> 90 (Moves AWAY from zero)
        let count = dial.rotate_and_count(DialRotation::Left(10));
        assert_eq!(dial.at(), 90);
        assert_eq!(count, 0, "Moving Left FROM zero should NOT count as hitting it");
    }

    #[test]
    fn test_right_from_zero_boundary() {
        let mut dial = Dial { pointing: 0 };

        // 0 -> 10 (Moves AWAY from zero)
        let count = dial.rotate_and_count(DialRotation::Right(10));
        assert_eq!(dial.at(), 10);
        assert_eq!(count, 0, "Moving Right FROM zero should NOT count");
    }

    #[test]
    fn test_full_tours() {
        let mut dial = Dial { pointing: 50 };

        // Right 250 (2 full circles + 50) -> Lands on 0 (50 + 50 = 100%100 = 0)
        // 2 full tours = 2 counts.
        // Landing on zero = 1 count.
        // Total = 3.
        let count = dial.rotate_and_count(DialRotation::Right(250));
        assert_eq!(dial.at(), 0);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_full_tours_left() {
        let mut dial = Dial { pointing: 10 };

        // Left 110 (1 full circle + 10) -> Lands on 0
        // 1 full tour = 1 count.
        // Landing on zero = 1 count.
        // Total = 2.
        let count = dial.rotate_and_count(DialRotation::Left(110));
        assert_eq!(dial.at(), 0);
        assert_eq!(count, 2);
    }
}
