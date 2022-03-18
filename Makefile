EXECUTABLE_NAME := s2tw

all: ./target/release/$(EXECUTABLE_NAME)

./target/release/$(EXECUTABLE_NAME): $(shell find . -type f -iname '*.rs' -o -name 'Cargo.toml' | sed 's/ /\\ /g')
	PWD=$$(pwd)
	cd $$OPENCC_PATH && bash build.sh
	cd $$PWD
	OPENCC_LIB_DIRS="$$OPENCC_PATH/linux/lib:$$OPENCC_PATH/OpenCC/Release/deps/marisa-0.2.6" OPENCC_INCLUDE_DIRS="$$OPENCC_PATH/linux/include/opencc" OPENCC_STATIC=1 OPENCC_LIBS=opencc:marisa OPENCC_DYLIB_STDCPP=1 cargo build --release
	strip ./target/release/$(EXECUTABLE_NAME)
	
install:
	$(MAKE)
	sudo cp ./target/release/$(EXECUTABLE_NAME) /usr/local/bin/$(EXECUTABLE_NAME)
	sudo chown root: /usr/local/bin/$(EXECUTABLE_NAME)
	sudo chmod 0755 /usr/local/bin/$(EXECUTABLE_NAME)
	
test:
	cargo test --verbose
	
clean:
	cargo clean
