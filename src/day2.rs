use std::ops::AddAssign;

pub fn solve() -> anyhow::Result<()> {
    let input = read_input()?;
    let mut total_invalid_ids: u64 = 0;
    for range in &input{
        for id in *range {
            if !id.is_valid() {
                total_invalid_ids += id.0;
            }
        }
    }
    println!("Total invalid ids (PART 1): {}", total_invalid_ids);

    let mut total_invalid_ids: u64 = 0;
    for range in &input {
        for id in *range {
            if !id.is_valid_p2() {
                total_invalid_ids += id.0;
            }
        }
    }
    println!("Total invalid ids (PART 2): {}", total_invalid_ids);


    Ok(())
}

/// Represents an product ID
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Id(u64);

impl Id {
    fn is_valid(&self) -> bool {
        let mut v = self.0;
        let mut dc: u32 = 0;
        while v > 0 {
            v /= 10;
            dc += 1;
        }
        if dc % 2 != 0 {
            return true;
        }
        let patlen = dc / 2;
        let pow = 10_u64.pow(patlen);
        let left_pat = self.0 / pow;
        let right_pat = self.0 % pow;
        left_pat != right_pat
    }

    fn is_valid_p2(&self) -> bool {
        let s = self.0.to_string().into_bytes();
        let n = s.len();
        assert!(n != 0, "length of an id cannot be zero");
        // return true if there is sequence at least repeated twice using window w
        let check_with_window = |w: usize| {
            if n % w != 0 {
                return false;
            }
            let mut i: usize = 0;
            let ss = &s[i..(i + w)];
            i = i + w;
            let mut at_least_two = false;
            while i < n {
                let ss_next = &s[i..(i + w)];
                if ss_next != ss {
                    return false;
                }
                i += w;
                at_least_two = true;
            }
            return at_least_two;
        };
        for w in 1..n {
            if check_with_window(w) {
                return false;
            }
        }
        return true;
    }
}

impl AddAssign<u64> for Id {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs
    }
}
/// Represents a range. e.g.: 11-22
#[derive(Debug, Clone, Copy)]
struct IdRange {
    first: Id,
    last: Id,
}

impl Iterator for IdRange {
    type Item = Id;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first > self.last {
            None
        } else {
            let id = self.first;
            self.first += 1;
            Some(id)
        }
    }
}

impl IdRange {
    fn new(first: Id, last: Id) -> Self {
        assert!(first <= last, "Expected first <= last");
        Self { first, last }
    }
}

/// Reads input file and returns a [Vec] containing all `IdRange`s in the file.
fn read_input() -> anyhow::Result<Vec<IdRange>> {
    let content = std::fs::read_to_string("inputs/input2.txt")?;
    let mut ranges = Vec::new();
    for range in content.trim().split(',') {
        let ids: Vec<_> = range
            .split('-')
            .map(|s| anyhow::Ok(Id(s.parse()?)))
            .take(2)
            .collect::<anyhow::Result<_>>()?;
        let range = IdRange::new(ids[0], ids[1]);
        ranges.push(range);
    }
    Ok(ranges)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id_validity() {
        let ids = [11, 22, 1010, 123123, 1188511885, 38593859, 446446, 222222];
        for id in ids {
            let x = Id(id);
            assert!(!x.is_valid(), "id: {} is not valid but got valid=true", id);
        }
        let ids = [111, 222, 1231234, 43, 101, 23523];
        for id in ids {
            let x = Id(id);
            assert!(x.is_valid(), "id: {} is valid but got valid=false", id);
        }
    }
    #[test]
    fn test_id_validity_p2() {
        // all invalid
        let ids = [1188511885, 999, 11, 22, 565656, 2121212121, 446446, 222222];
        for id in ids {
            let x = Id(id);
            assert!(!x.is_valid_p2(), "id: {} is not valid for part 2 but got valid=true", id);
        }

        let ids = [1231234, 1, 101, 10001001, 90990999];
        for id in ids {
            let x = Id(id);
            assert!(x.is_valid_p2(), "id: {} is valid for part 2 but got valid=false", id);
        }
    }
}
