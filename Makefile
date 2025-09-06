all:
	cargo build --release
	cp target/release/collect-btc-address ./data/
	time ./data/collect-btc-address
deploy:
	cp target/release/collect-btc-address ./data/
test:
	./data/collect-btc-address
