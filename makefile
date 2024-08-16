time:
	RUSTFLAGS="-C target-cpu=native" cargo bench

flame:
	RUSTFLAGS="-C target-cpu=native" CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph && firefox flamegraph.svg
