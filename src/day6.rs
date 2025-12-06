use anyhow::{Context, bail};

pub fn solve() -> anyhow::Result<()> {
    let (all_operands, ops) = read_input_as_rows()?;
    let grand_total = execute_as_rows(all_operands, ops)?;
    println!("Grand total (PART 1): {}", grand_total);

    let (all_operands, ops) = read_input_as_columns()?;
    let grand_total = execute_as_columns(all_operands, ops)?;
    println!("Grand total (PART 2): {}", grand_total);

    Ok(())
}

fn calculate(operands: Vec<u64>, op: Op) -> u64 {
    match op {
        Op::Add => operands.iter().sum::<u64>(),
        Op::Multiply => operands.iter().product::<u64>(),
    }
}

fn execute_as_rows(all_operands: Vec<Vec<u64>>, ops: Vec<Op>) -> anyhow::Result<u64> {
    let mut curr = 0;
    let len = ops.len();
    let mut grand_total: u64 = 0;
    while curr < len {
        let mut operands = Vec::new();
        for numbers in &all_operands {
            operands.push(numbers[curr]);
        }
        let op = ops[curr];
        grand_total += calculate(operands, op);
        curr += 1;
    }
    Ok(grand_total)
}

fn execute_as_columns(all_operands: Vec<Vec<u64>>, ops: Vec<Op>) -> anyhow::Result<u64> {

    let mut grand_total: u64 = 0;
    for (i, operands) in all_operands.into_iter().enumerate() {
        println!("{:?} FOR {:?}", operands, ops[i]);
        grand_total += calculate(operands, ops[i]);
    }
    Ok(grand_total)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
}

fn read_input_as_rows() -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    let content = std::fs::read_to_string("inputs/input6.txt")?;

    let mut numbers_v = Vec::new();
    for line in content.lines() {
        let mut numbers = Vec::new();
        if line.starts_with('*') || line.starts_with('+') {
            // read last line
            let mut ops = Vec::new();
            for op in line.split(' ') {
                if op.is_empty() {
                    continue;
                }
                if op == "*" {
                    ops.push(Op::Multiply);
                } else if op == "+" {
                    ops.push(Op::Add);
                } else {
                    bail!("Unexpected char in op line: {}", op);
                }
            }
            return Ok((numbers_v, ops));
        }
        for num_part in line.split(' ') {
            if num_part.is_empty() {
                continue;
            }
            numbers.push(num_part.parse().context("input error")?);
        }
        numbers_v.push(numbers);
    }
    bail!("input error");
}

fn read_input_as_columns() -> anyhow::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    let content = std::fs::read_to_string("inputs/input6.txt")?;
    let lines: Vec<_> = content.lines().collect();
    let op_line = lines[lines.len() - 1];
    let num_lines: Vec<_> = lines[..(lines.len() - 1)]
        .iter()
        .map(|s| s.as_bytes())
        .collect();

    let mut ops = Vec::new();
    let mut op_lens: Vec<u32> = Vec::new();
    {
        let mut curr_len = 0;
        for ch in op_line.bytes() {
            match ch {
                b'*' => {
                    if curr_len > 1 {
                        op_lens.push(curr_len);
                        curr_len = 0;
                    }
                    ops.push(Op::Multiply);
                }
                b'+' => {
                    if curr_len > 1 {
                        op_lens.push(curr_len);
                        curr_len = 0;
                    }
                    ops.push(Op::Add);
                }
                b' ' => {
                    curr_len += 1;
                }
                x => bail!("unexpected byte: {}", x),
            }
        }
        op_lens.push(curr_len + 1);
    }

    fn read_operands(num_lines: &[&[u8]], mut start: usize, op_len: u32) -> Vec<u64> {
        let mut operands = Vec::new();
        for _ in 0..op_len {
            let mut vertical_digits = Vec::new();
            for &line in num_lines {
                match line[start] {
                    b' ' => {},
                    x => {
                        vertical_digits.push(x - b'0');
                    },
                };
            }
            let operand = vertical_digits.into_iter()
                .fold(0, |acc, digit| acc * 10 + digit as u64);
            operands.push(operand);
            start += 1;
        }
        operands
    }
    let mut all_operands = Vec::new();
    let mut start = 0;
    for op_len in op_lens {
        let operands = read_operands(&num_lines, start, op_len);
        all_operands.push(operands);
        start += op_len as usize + 1;
    }
    Ok((all_operands, ops))
}
