all: rust blackrose radon path

blackrose:
	chmod +x bin/blackrose
	make -C BlackRose

radon:
	chmod +x bin/radon
	make -C RadonVM

path:
	printf "\n\nAdd BlackRose's bin folder to your PATH!\n\n"

rust:
	cargo --version


core: radon blackrose