all: main

main:
	python BUILD.py

blackrose:
	chmod +x bin/blackrose
	make -C BlackRose

radon:
	chmod +x bin/radon
	make -C RadonVM


core: blackrose radon