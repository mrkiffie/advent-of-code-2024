use clap::Parser;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// Advent of code 2024!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long)]
    day: String,

    /// Which part to run
    #[arg(short, long)]
    part: u8,
}

#[tracing::instrument(level = "trace", skip())]
fn main() {
    let args = Args::parse();

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let result = match (args.day.as_str(), args.part) {
        ("day-01", 1) => day_01::part1::run(),
        ("day-01", 2) => day_01::part2::run(),
        ("day-02", 1) => day_02::part1::run(),
        ("day-02", 2) => day_02::part2::run(),
        ("day-03", 1) => day_03::part1::run(),
        ("day-03", 2) => day_03::part2::run(),
        _ => String::from("Unimplemented"),
    };

    println!("{}", result);
}
