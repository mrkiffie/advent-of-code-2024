# Advent of code 2024

## Usage

Run `cargo run -- --help` to get an overview of the commands that are available in the top level bin.

Running a specific day's code can be achieved using the following
```
cargo run -- --day day-01 --part 1
```
or
```
cargo run -- -d day-01 -p 1
```

Both of these commands will output the results of a specific day and part's processing.

## Project structure

This repo is a cargo workspace that contains a unique library package per day.

Each library consists of both `part1.rs` and `part2.rs`. These are exposed to the top level binary by exposing them in `lib.rs`.

The input for `part1.rs` and `part2.rs` are to be put in `input.txt`. The contents of this file is inlined into the library using `include_str!()`.

## Adding a new day

A new day lib can be created from the `template` directory by running `cp -R template day-02` and then updating the package's `name` in the `Cargo.toml` file.

```diff
diff --git day-02/Cargo.toml day-02/Cargo.toml
index c09e583..6f0f217 100644
--- day-02/Cargo.toml
+++ day-02/Cargo.toml
@@ -1,5 +1,5 @@
 [package]
-name = "day-xx"
+name = "day-02"
 version = "0.1.0"
 edition = "2021"
```


The new day will need to be added as a dependency to the top level binary

```diff
diff --git Cargo.toml Cargo.toml
index 8b4f08b..28f847c 100644
--- Cargo.toml
+++ Cargo.toml
@@ -12,3 +12,4 @@ edition = "2021"
 clap = { version = "4.4.10", features = ["derive"] }

 day-01 = { path = "day-01" }
+day-02 = { path = "day-02" }
```

Update `main.rs`'s match block to include new day and parts.

```diff
diff --git src/main.rs src/main.rs
index 8a849f0..3dd122b 100644
--- src/main.rs
+++ src/main.rs
@@ -19,6 +19,8 @@ fn main() {
     let result = match (args.day.as_str(), args.part) {
         ("day-01", 1) => day_01::part1::run(),
         ("day-01", 2) => day_01::part2::run(),
+        ("day-02", 1) => day_02::part1::run(),
+        ("day-02", 2) => day_02::part2::run(),
         _ => String::from("Unimplemented"),
     };

```

## Dependencies

The workspace is intended to define the dependencies, and the workspace packages should references the dependencies from the workspace.
