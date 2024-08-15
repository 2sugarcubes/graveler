time:
	RUSTFLAGS="-C target-cpu=native" cargo build --release && time target/release/graveler
