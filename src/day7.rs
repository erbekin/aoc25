use std::collections::{HashMap, HashSet};

pub fn solve() -> anyhow::Result<()> {
    let manifold = read_input()?;
    let mut split_count = 0;
    let mut current_beams = HashSet::new();
    current_beams.insert(manifold.start_loc);
    split_beams(&mut current_beams, &manifold, 0, &mut split_count);
    println!("Total splits: {}", split_count);

    println!("Timeline count: {}", timeline_count(&manifold));
    Ok(())
}

#[derive(Debug)]
struct Manifold {
    /// location of S
    start_loc: i32,
    /// splitters\[i\] gives splitter locations of level i
    splitters: Vec<HashSet<i32>>,
}

fn split_beams(
    current_beams: &mut HashSet<i32>,
    manifold: &Manifold,
    current_level: usize,
    split_count: &mut usize,
) {
    if current_level >= manifold.splitters.len() {
        return;
    }
    let splitters = &manifold.splitters[current_level];
    for splitter in splitters {
        if current_beams.contains(splitter) {
            current_beams.remove(splitter);
            current_beams.insert(splitter + 1);
            current_beams.insert(splitter - 1);
            *split_count += 1;
        }
    }
    split_beams(current_beams, manifold, current_level + 1, split_count);
}

fn timeline_count(manifold: &Manifold) -> u64 {

    let mut current_beams = HashMap::new();
    current_beams.insert(manifold.start_loc, 1);

    for splitters in &manifold.splitters {
        let mut next_beams = HashMap::new();
        for (col, count) in &current_beams {
            if splitters.contains(col) {
                *next_beams.entry(col - 1).or_insert(0) += count;
                *next_beams.entry(col + 1).or_insert(0) += count;
            } else {
                *next_beams.entry(*col).or_insert(0) += count;
            }
        }
        current_beams = next_beams;
    }
    current_beams.values().sum()
}

fn read_input() -> anyhow::Result<Manifold> {
    let content = std::fs::read_to_string("inputs/input7.txt")?;
    let mut manifold = Manifold {
        start_loc: 0,
        splitters: Vec::new(),
    };

    for line in content.lines() {
        // start new level
        manifold.splitters.push(HashSet::new());
        for (loc, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                'S' => {
                    manifold.start_loc = loc as _;
                }
                '^' => {
                    // push to current level
                    let current_level = manifold.splitters.len() - 1;
                    manifold.splitters[current_level].insert(loc as _);
                }
                _ => {
                    anyhow::bail!("unexpected character in input");
                }
            }
        }
        if manifold.splitters.last().unwrap().is_empty() {
            manifold.splitters.pop();
        }
    }
    Ok(manifold)
}
