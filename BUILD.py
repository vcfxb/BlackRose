#!/usr/bin/env python3.5
import subprocess, sys, os
run = subprocess.run
output_info = subprocess.CompletedProcess
PIPE = subprocess.PIPE
dir_path = os.path.dirname(os.path.realpath(__file__))

cmnd = run(["rustc", "--version"], stdout=PIPE, universal_newlines=True)
if cmnd.returncode != 0:
    sys.exit("Rust is not Installed!")
elif int(cmnd.stdout[8:10]) >= 16:
    pass
else:
    sys.exit("Rust version 1.16.0 or higher required. Your version is {}".format(cmnd1.stdout[8:12]))

for a in ["blackrose", "radon"]:
    cmnd = run(["chmod", "+x", "{}/bin/{}".format(dir_path, a)])
    if cmnd.returncode != 0:
        sys.exit("Error in making bin files executable!")

os.chdir("{}/BlackRose".format(dir_path))
cmnd = run(["cargo", "build", "--release"], stdout=PIPE, universal_newlines=True)
if cmnd.returncode != 0:
    print(cmnd.stdout)
    sys.exit("\n\nCould not build BlackRose!\n\n")

os.chdir("{}/RadonVM".format(dir_path))
cmnd = run(["cargo", "build", "--release"], stdout=PIPE, universal_newlines=True)
if cmnd.returncode != 0:
    print(cmnd.stdout)
    sys.exit("\n\nCould not build Radon!\n\n")

print("\nMake sure to add BlackRose's bin to your PATH variable.\n")