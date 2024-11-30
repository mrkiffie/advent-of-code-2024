work day part:
	cargo watch -x "check -p {{day}}" -s "just test {{day}} {{part}}"
lint:
	cargo clippy
test day part:
	cargo test -p {{day}} {{part}}
dhat day part:
	cargo run --profile dhat --features dhat-heap -- --day {{day}} --part {{part}}
generate day:
	cp -R template {{day}}
	sed -i '' 's/"day-xx"/"{{day}}"/' {{day}}/Cargo.toml
	cargo add {{day}} --path {{day}}
