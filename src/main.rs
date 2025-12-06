use anyhow::bail;
use aoc25::*;
use std::time::Instant; // Import Instant

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let day: u32 = match args.get(1) {
        Some(d) => d.parse()?,
        None => {
            eprintln!("Usage: cargo run -- <day:u32>");
            return Ok(());
        }
    };

    // 1. Start the timer
    let start_time = Instant::now();

    // 2. Run the solution and capture the result
    let result = match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        _ => {
            bail!("The day has not been lived yet");
        }
    };

    // 3. Stop the timer
    let duration = start_time.elapsed();

    // 4. Print time only if the solution ran successfully
    if result.is_ok() {
        // .as_secs_f64() * 1000.0 gives you precise milliseconds
        println!("[Finished in {:.2} ms]", duration.as_secs_f64() * 1000.0);
    }

    // 5. Return the result (propagate errors if any)
    result
}
