all: main

main:
	python BUILD.py

travis: theblackrose radon

theblackrose:
	chmod +x bin/blackrose
	make -C BlackRose

radon:
	chmod +x bin/radon
	make -C RadonVM
