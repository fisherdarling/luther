build:
	rustup override set nightly
	cargo build --release --bin LUTHER;
	@cp ./target/release/LUTHER ./LUTHER;
	@chmod +x ./LUTHER

clean:
	cargo clean
	rm *.m
	rm *.cmptt
	rm *.tt
	rm *.dat