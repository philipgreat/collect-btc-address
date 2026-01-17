all:
	cargo build --release
	mkdir -p data
	cp target/release/collect-btc-address ./data/
	time ./data/collect-btc-address

deploy:
	cp target/release/collect-btc-address ./data/
test:
	./data/collect-btc-address

dist:
	rsync -avz  -e "ssh -p 6543" --exclude target ./*   philip@t420.doublechaintech.cn:~/githome/collect-btc-address/
