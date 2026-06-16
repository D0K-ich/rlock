add_deps:
	cd crypter & cargo add rdev

rc:
	cd crypter & cargo run

rl:
	cd locker_builder & cargo run

bc:
	cd crypter & cargo build

bcs:
	cd crypter & cargo build --release

bcd:
	cd crypter & set RUSTFLAGS=-C target-feature=+crt-static && cargo build --release

bss:
	cd locker_builder & cargo build --release

cd:
	objdump -p client/target/release/client.exe | Select-String DLL