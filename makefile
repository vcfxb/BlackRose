all: blackrose rust rustFiles path

blackrose:
	chmod +x bin/blackrose

path:
	sh lib/path.sh

rust:
	rustc --version

rustFiles:
	cargo build --release
