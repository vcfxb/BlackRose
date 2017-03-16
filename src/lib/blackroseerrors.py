import sys
errors = {                  #exit status, internal status
    "ArgumentError": (1,0)
}

class ArgumentError:
    TYPE = "ArgumentError"
    def __init__(self, alinenum, aloc, aline, expectedargs, receivedargs):
        if (alinenum == "TERM") & (aloc == "TERM"):
            self.location = 'command prompt'
        else:
            self.location = str(alinenum)+':'+str(aloc)
        self.line = aline
        self.info = "Expected "+str(expectedargs)+" arguments. Received "+str(receivedargs)+" arguments."

def execute(ERR):
    final = []
    final.append(ERR.TYPE+" at "+ERR.location+" :\n")
    final.append("  "+ERR.line)
    if ERR.location == 'command prompt':
        final.append('  '+('-'*len(ERR.line)))
    else:
        final.append('  '+('-'*(int(ERR.location.split(':')[1])-1)+'^'))
    final.append('\n'+ERR.info)
    print('\n'.join(final))
    sys.exit(errors[ERR.TYPE][0])
