all: blackrose rust rustFiles path

blackrose:
	chmod +x bin/blackrose

path:
	printf "\n\nAdd BlackRose's folder to your PATH!\n\n"

rust:
	rustc --version

rustFiles:
	cargo build --release
