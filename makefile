all: main

main:
	python BUILD.py

travis: blackrose radon

blackrose:
	chmod +x bin/blackrose
	make -C BlackRose

radon:
	chmod +x bin/radon
	make -C RadonVM
