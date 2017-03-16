#!/usr/bin/env python3.5

# standard libraries
import sys, os

# blackrose libraries
from lib.preproc import preprocessor as preproc
import lib.error as pyerror
import lib.blackroseerrors as error


def main(args):
    if len(args) > 2:
        error.execute(error.ArgumentError('TERM', 'TERM', ' '.join(args), 1, len(args)-1))
    elif len(args) == 2:
        runFile(open(sys.argv[1], 'r'))
    else:
        runPrompt(['radon'])

def runFile(s):
    try:
        print(preproc(s.readlines()))
    finally:
        s.close()

def runPrompt(prompt):
    while True:
        try:
            t = input(':'.join(prompt))
            print(preproc([t]))
        except KeyboardInterrupt:
            print('\nexit')
            sys.exit(0)

main(sys.argv)
