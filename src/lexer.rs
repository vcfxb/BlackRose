fn lex() {
    // Prepeare for this stage of development
}

//  old python program.
// def oldlex(linelist):
//     final = []
//     for x, line in enumerate(linelist):
//         temp = ''
//         stemp = []
//         fmode = False
//         smode = False
//         for y, char in enumerate(line):
//             if smode == True:
//                 if char in ["'", '"']:
//                     stemp.append(char)
//                     smode = False
//                     final.append(''.join(stemp))
//                     stemp = []
//                     continue
//                 else:
//                     stemp.append(char)
//                     continue
//             elif smode == False:
//                 if char in ["'", '"']:
//                     stemp.append(char)
//                     smode = True
//                     continue
//                 else:
//                     pass
//             if 48 <= ord(char) <= 57:
//                 temp = temp + char
//                 if 48 <= ord(line[y+1]) <= 57:
//                     continue
//                     continue
//                 else:
//                     if fmode == False:
//                         continue
//                 else:
//                     if fmode == False:
//                         final.append(int(temp))
//                         temp = ''
//                     elif fmode == True:
//                         final.append(float(temp))
//                         temp = ''
//                         fmode = False
//             elif char == '.':
//                 if (48 <= ord(line[y-1]) <= 57) & (48 <= ord(line[y+1]) <= 57):
//                     temp = temp+char
//                 else:
//                     final.append(char)
//             elif (0x41 <= ord(char) <= 0x5A) | (0x61 <= ord(char) <= 0x7A):
//                 temp = temp+char
//                 if (0x41 <= ord(line[y+1]) <= 0x5A) | (0x61 <= ord(line[y+1]) <= 0x7A):
//                     continue
//                 else:
//                     if temp == 'True':
//                         final.append(True)
//                     elif temp == 'False':
//                         final.append(False)
//                     else:
//                         final.append(temp)
//                     temp = ''
//             elif char == ' ':
//                 continue
//             elif char == '\n':
//                 continue
//             else:
//                 final.append(char)
//     return final
//
// def lex(enumeratedlines):
//     worklist = []
//     final = []
//     for pair in enumeratedlines:
//         for x in pair[1].split(';'):
//             worklist.append([pair[0], x])
//     for n, pair in enumerate(worklist):
//         if pair[1] in ['\n','']:
//             worklist[n] = None
//     for x in range(worklist.count(None)):
//         del worklist[worklist.index(None)]
//     for
//     final = worklist
//     return worklist
