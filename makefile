all: rust blackrose radon path

blackrose:
	chmod +x bin/blackrose
	cd BlackRose
	cargo build --release

radon:
	chmod +x bin/radon
	cd RadonVM
	cargo build --release

path:
	printf "\n\nAdd BlackRose's bin folder to your PATH!\n\n"

rust:
	cargo --version

