use anyhow::bail;
use ahash::AHashSet;
pub fn solve() -> anyhow::Result<()> {
    let mut input = read_input()?;
    println!(
        "Total accesible rolls (PART 1): {}",
        input.accessible_rolls()
    );
    let mut total_removed = 0;
    while let c = input.remove_accesibles()
        && c > 0
    {
        total_removed += c;
    }
    println!("Total removed rolls (PART 2): {}", total_removed);
    Ok(())
}

use ahash::AHashMap;
use std::collections::VecDeque;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Roll {
    pos: (i32, i32),
}

impl Roll {
    #[inline]
    fn neighbors(&self) -> impl Iterator<Item = Roll> {
        let (x, y) = self.pos;
        (-1..=1).flat_map(move |dy| {
            (-1..=1).filter_map(move |dx| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    Some(Roll { pos: (x + dx, y + dy) })
                }
            })
        })
    }
}

#[derive(Debug, Clone)]
struct RollGrid {
    rolls: AHashSet<Roll>,
}

impl RollGrid {
    fn accessible_rolls(&self) -> usize {
        self.rolls
            .iter()
            .filter(|r| self.count_neighbors(*r) < 4)
            .count()
    }

    #[inline]
    fn count_neighbors(&self, roll: &Roll) -> usize {
        roll.neighbors().filter(|n| self.rolls.contains(n)).count()
    }

    /// SUPER-OPTIMIZED VERSION
    fn remove_accesibles(&mut self) -> usize {
        // neighbor count cache
        let mut neigh = AHashMap::<Roll, u8>::new();
        neigh.reserve(self.rolls.len());

        // compute neighbor counts once
        for r in &self.rolls {
            let count = r.neighbors().filter(|n| self.rolls.contains(n)).count();
            neigh.insert(*r, count as u8);
        }

        // queue of rolls currently <4 neighbors
        let mut q = VecDeque::new();
        for (r, c) in neigh.iter() {
            if *c < 4 {
                q.push_back(*r);
            }
        }

        let mut removed = 0;

        // cascading removals
        while let Some(r) = q.pop_front() {
            if !self.rolls.remove(&r) {
                continue; // already removed
            }
            removed += 1;

            // update neighbors
            for adj in r.neighbors() {
                if let Some(count) = neigh.get_mut(&adj) {
                    if *count > 0 {
                        *count -= 1;
                        if *count < 4 {
                            q.push_back(adj);
                        }
                    }
                }
            }
        }

        removed
    }
}

fn read_input() -> anyhow::Result<RollGrid> {
    let content = std::fs::read_to_string("inputs/input4.txt")?;
    let mut rolls = AHashSet::new();
    let mut y = 0;
    for line in content.lines() {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '@' => {
                    rolls.insert(Roll { pos: (x, y) });
                }
                '.' => {}
                x => bail!("Unexpected character in input: {}", x),
            }
            x += 1;
        }
        y += 1;
    }
    Ok(RollGrid { rolls })
}
