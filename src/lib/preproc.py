def preprocessor(f):
    # remove comments
    workl = f
    for n, line in enumerate(f):
        if '#' not in line:
            pass
        else:
            workl[n] = workl[n][0:line.index('#')]
    for n in range(workl.count('')):
        del workl[workl.index('')]
    return workl
