def preprocessor(stringlist):
    # remove comments
    #single line comments first
    workl = stringlist
    for n, line in enumerate(stringlist):
        if '#' not in line:
            pass
        else:
            workl[n] = workl[n][0:line.index('#')]
    # multiline comments
    del stringlist
    workl = (''.join(workl)).split('###')
    worklf = []
    for x in range(0,len(workl),2):
        worklf.append(workl[x])
    del workl
    for n in range(worklf.count('')):
        del worklf[worklf.index('')]
    return worklf
