all: ./target/release/s2tw

./target/release/s2tw: $(shell find . -type f -iname '*.rs' -o -name 'Cargo.toml' | sed 's/ /\\ /g')
	PWD=$$(pwd)
	cd $$OPENCC_PATH && bash build.sh
	cd $$PWD
	OPENCC_LIB_DIRS="$$OPENCC_PATH/linux/lib:$$OPENCC_PATH/OpenCC/Release/deps/marisa-0.2.5" OPENCC_INCLUDE_DIRS="$$OPENCC_PATH/linux/include/opencc" OPENCC_STATIC=1 OPENCC_LIBS=opencc:marisa OPENCC_DYLIB_STDCPP=1 cargo build --release
	strip ./target/release/s2tw
	
install:
	$(MAKE)
	sudo cp ./target/release/s2tw /usr/local/bin/s2tw
	sudo chown root. /usr/local/bin/s2tw
	sudo chmod 0755 /usr/local/bin/s2tw
	
test:
	cargo test --verbose
	
clean:
	cargo clean
