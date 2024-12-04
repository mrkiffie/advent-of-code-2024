work day part:
	cargo watch -x "check -p {{day}}" -s "just test {{day}} {{part}}"
lint:
	cargo clippy
test day part:
	cargo test -p {{day}} -- --nocapture {{part}}
dhat day part:
	cargo run --profile dhat --features dhat-heap -- --day {{day}} --part {{part}}
bench day:
	cargo bench -p {{day}} --features bench
generate day:
	cp -R template day-{{day}}
	sed -i '' 's/xx/{{day}}/' day-{{day}}/Cargo.toml
	sed -i '' 's/xx/{{day}}/' day-{{day}}/benches/benchmark.rs
	cargo add day-{{day}} --path day-{{day}}
