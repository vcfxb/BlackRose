// old Python file
// def preprocessor(stringlist):
//     # remove comments
//     workl = stringlist
//     linenum = 0
//     for n, line in enumerate(stringlist):
//         if '#' not in line:
//             workl[n] = (linenum, line)
//             linenum += 1
//         else:
//             workl[n] = (linenum, line[0:line.index('#')])
//             linenum += 1
//     return workl
