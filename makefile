time:
	RUSTFLAGS="-C target-cpu=native" cargo build --release && for i in `seq 1 20`; do time target/release/graveler; done
