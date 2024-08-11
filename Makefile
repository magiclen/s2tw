EXECUTABLE_NAME := s2tw

all: ./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME)

./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME): $(shell find . -type f -iname '*.rs' -o -name 'Cargo.toml' | grep -v ./target | sed 's/ /\\ /g')
	PWD=$$(pwd)
	cd $$OPENCC_PATH && bash build.sh
	cd $$PWD
	OPENCC_LIB_DIRS="$$MUSL_PATH/x86_64-linux-musl/lib:$$OPENCC_PATH/linux/lib" OPENCC_INCLUDE_DIRS="$$OPENCC_PATH/linux/include/opencc" OPENCC_STATIC=1 OPENCC_LIBS=marisa:opencc OPENCC_STATIC_STDCPP=1 cargo build --release --target x86_64-unknown-linux-musl

install:
	$(MAKE)
	sudo cp ./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME) /usr/local/bin/$(EXECUTABLE_NAME)
	sudo chown root: /usr/local/bin/$(EXECUTABLE_NAME)
	sudo chmod 0755 /usr/local/bin/$(EXECUTABLE_NAME)
	
test:
	cargo test --verbose
	
clean:
	cargo clean
