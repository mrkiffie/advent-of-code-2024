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
        ("day-04", 1) => day_04::part1::run(),
        ("day-04", 2) => day_04::part2::run(),
        ("day-05", 1) => day_05::part1::run(),
        ("day-05", 2) => day_05::part2::run(),
        ("day-06", 1) => day_06::part1::run(),
        ("day-06", 2) => day_06::part2::run(),
        ("day-07", 1) => day_07::part1::run(),
        ("day-07", 2) => day_07::part2::run(),
        ("day-08", 1) => day_08::part1::run(),
        ("day-08", 2) => day_08::part2::run(),
        ("day-09", 1) => day_09::part1::run(),
        ("day-09", 2) => day_09::part2::run(),
        ("day-10", 1) => day_10::part1::run(),
        ("day-10", 2) => day_10::part2::run(),
        ("day-11", 1) => day_11::part1::run(),
        ("day-11", 2) => day_11::part2::run(),
        ("day-12", 1) => day_12::part1::run(),
        ("day-12", 2) => day_12::part2::run(),
        ("day-13", 1) => day_13::part1::run(),
        ("day-13", 2) => day_13::part2::run(),
        ("day-14", 1) => day_14::part1::run(),
        ("day-14", 2) => unimplemented!("Run `cargo run -p day-14` instead"),
        ("day-15", 1) => day_15::part1::run(),
        ("day-15", 2) => day_15::part2::run(),
        ("day-16", 1) => day_16::part1::run(),
        ("day-16", 2) => day_16::part2::run(),
        ("day-17", 1) => day_17::part1::run(),
        ("day-17", 2) => day_17::part2::run(),
        ("day-18", 1) => day_18::part1::run(),
        ("day-18", 2) => day_18::part2::run(),
        ("day-19", 1) => day_19::part1::run(),
        ("day-19", 2) => day_19::part2::run(),
        ("day-20", 1) => day_20::part1::run(),
        ("day-20", 2) => day_20::part2::run(),
        ("day-21", 1) => day_21::part1::run(),
        ("day-21", 2) => day_21::part2::run(),
        ("day-22", 1) => day_22::part1::run(),
        ("day-22", 2) => day_22::part2::run(),
        ("day-23", 1) => day_23::part1::run(),
        ("day-23", 2) => day_23::part2::run(),
        ("day-24", 1) => day_24::part1::run(),
        ("day-24", 2) => day_24::part2::run(),
        _ => unimplemented!("{} not implemented in src/main.rs", args.day),
    };

    println!("{}", result);
}
