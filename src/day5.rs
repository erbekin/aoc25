use std::str::FromStr;

use anyhow::{Context, anyhow};

pub fn solve() -> anyhow::Result<()> {
    let (input_ivs, input_points) = read_input()?;
    let root = construct_interval_node(input_ivs.clone()).ok_or(anyhow!("Cannot construct interval tree"))?;

    let mut total_fresh_ingredients = 0;
    for point in input_points {
        if overlap_any_point(Some(&root), point) {
            total_fresh_ingredients += 1;
        }
    }
    println!("Total fresh ingredients (PART 1): {}", total_fresh_ingredients);

    let union = union(input_ivs);
    let mut total_range = 0;
    for iv in union {
        total_range += iv.size();
    }
    println!("Total range (PART 2): {}", total_range);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: u64,
    end: u64,
}

impl Interval {
    #[inline]
    fn intersects(&self, point: u64) -> bool {
        self.start <= point && point <= self.end
    }

    #[inline]
    fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl FromStr for Interval {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('-').collect();
        let start: u64 = parts
            .get(0)
            .ok_or(anyhow::anyhow!("invalid interval format: {}", s))?
            .parse()
            .context("interval")?;
        let end: u64 = parts
            .get(1)
            .ok_or(anyhow::anyhow!("invalid interval format: {}", s))?
            .parse()
            .context("interval")?;

        Ok(Self { start, end })
    }
}

fn read_input() -> anyhow::Result<(Vec<Interval>, Vec<u64>)> {
    let content = std::fs::read_to_string("inputs/input5.txt")?;
    let mut intervals = Vec::new();
    let mut ids = Vec::new();
    let mut intervals_end = false;
    for line in content.lines() {
        if line.is_empty() {
            intervals_end = true;
            continue;
        }
        if intervals_end {
            ids.push(line.parse()?);
        } else {
            intervals.push(line.parse()?);
        }
    }
    Ok((intervals, ids))
}

struct IntervalTreeNode {
    center: u64,
    left_set: Option<Box<IntervalTreeNode>>,
    right_set: Option<Box<IntervalTreeNode>>,
    intervals_start: Vec<Interval>,
    intervals_end: Vec<Interval>,
}

fn find_pivot(intervals: &[Interval]) -> u64 {
    let mut points: Vec<_> = intervals.iter().flat_map(|i| [i.start, i.end]).collect();
    points.sort();
    points[points.len() / 2]
}

fn overlap_any_point(node: Option<&IntervalTreeNode>, point: u64) -> bool {
    let Some(node) = node else {
        return false;
    };
    if point < node.center {
        match node
            .intervals_start
            .iter()
            .take_while(|iv| iv.start <= point)
            .find(|iv| iv.intersects(point))
        {
            Some(_) => true,
            None => overlap_any_point(node.left_set.as_deref(), point),
        }
    } else if point > node.center {
        match node
            .intervals_end
            .iter()
            .rev()
            .take_while(|iv| iv.end >= point)
            .find(|iv| iv.intersects(point))
        {
            Some(_) => true,
            None => overlap_any_point(node.right_set.as_deref(), point),
        }
    } else {
        !node.intervals_start.is_empty()
    }
}

fn construct_interval_node(intervals: Vec<Interval>) -> Option<Box<IntervalTreeNode>> {
    // Stop.
    if intervals.is_empty() {
        return None;
    }

    // find pivot
    let pivot = find_pivot(&intervals);

    // sort out
    let mut left_ivs = Vec::new();
    let mut right_ivs = Vec::new();
    let mut center_ivs = Vec::new();

    for iv in intervals {
        if iv.end < pivot {
            left_ivs.push(iv);
        } else if iv.start > pivot {
            right_ivs.push(iv);
        } else {
            center_ivs.push(iv);
        }
    }

    // two list: sorted with start and sorted with end
    let mut ivs_start = center_ivs.clone();
    ivs_start.sort_by_key(|iv| iv.start);

    let mut ivs_end = center_ivs.clone();
    ivs_end.sort_by_key(|iv| iv.end);

    // construct left and right nodes
    let left_set = construct_interval_node(left_ivs);
    let right_set = construct_interval_node(right_ivs);
    Some(Box::new(IntervalTreeNode {
        center: pivot,
        left_set,
        right_set,
        intervals_start: ivs_start,
        intervals_end: ivs_end,
    }))
}

// ============ PART 2 ============ //
fn union(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return vec![];
    }
    // First sort the intervals
    intervals.sort_by_key(|iv| iv.start);
    // contains discrete intervals
    let mut set = Vec::new();
    // bounds of largest continous interval range
    let mut low = intervals[0].start;
    let mut high = intervals[0].end;

    for iv in &intervals[1..] {
        // discrete interval found
        if iv.start > high {
            // push current
            set.push(Interval {start: low, end : high});
            low = iv.start;
            high = iv.end;
        } else if iv.end > high {
            high = iv.end;
        }
    }
    // push last
    set.push(Interval { start: low, end: high });
    set
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intervals() {
        let intervals : Vec<Interval >= ["0-4", "2-5", "3-6", "1-7"].iter().map(|s| s.parse().unwrap()).collect();

        let root = construct_interval_node(intervals).unwrap();
        assert!(overlap_any_point(Some(&root), 1));
        assert!(overlap_any_point(Some(&root), 2));
        assert!(overlap_any_point(Some(&root), 4));

    }
}
