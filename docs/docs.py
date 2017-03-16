#imports
import curses, locale, sys, os
from json import loads as Decode

version = sys.argv[1]
locale.setlocale(locale.LC_ALL, '')
code = locale.getpreferredencoding()
homef = open(os.path.dirname(os.path.abspath(__file__))+'/home.txt', 'r')
home = homef.readlines()

# Initialize screen
docs = curses.initscr()
curses.start_color()
(height, width) = docs.getmaxyx()
docs.keypad(1)

#Colors
curses.init_pair(1, curses.COLOR_GREEN, curses.COLOR_BLACK) #body
curses.init_pair(2, curses.COLOR_WHITE, curses.COLOR_RED) #unimplemented
curses.init_pair(3, curses.COLOR_CYAN, curses.COLOR_BLACK) #section
curses.init_pair(4, curses.COLOR_YELLOW, curses.COLOR_BLACK) #key
curses.init_pair(5, curses.COLOR_BLUE, curses.COLOR_BLACK) #search

# Headings (Alphabetized)
body_style = curses.color_pair(1)

key_style = curses.color_pair(4)

search_style = curses.color_pair(5)
search_style |= curses.A_BOLD

subsection_style = curses.color_pair(3)

title_style = curses.color_pair(0)
title_style |= curses.A_BOLD

unimplemented_style = curses.color_pair(2)
unimplemented_style |= curses.A_UNDERLINE

# Draw vars
key = ['q:exit', 'h:home', 's:search', 'S:search including descriptions']


# Drawing
def draw(bodylinelist = home, subsection = ['home'], searchprompt = 'Search documentation: '):
    # defined per instance draw vars
    bodyshift = 0
    searching = False
    descSearch = False
    try:
        while True:
            try:
                # vars
                charsmax = width-3-len(searchprompt)
                # clear buffer
                docs.clear()
                # titles
                docs.addnstr(1, 1, "BlackRose scripting language documentation", width-3, title_style)
                docs.addnstr(2, 1, "BlackRose version "+version, width-3, title_style)
                docs.addnstr(4, 1, "section:"+(':'.join(subsection)), width-3, subsection_style)
                # body
                for n, line in enumerate(bodylinelist[bodyshift:height-11+bodyshift]):
                    docs.addnstr(n+6, 3, line, width-5, body_style)
                #key
                for n, line in enumerate(key):
                    docs.addstr(height-4, 1+len(''.join(key[0:n]))+2*n, line, key_style)
                # Search
                docs.addstr(height-2, 1, searchprompt, search_style)
                # Border
                docs.border(0)  # Border drawn later to reduce alterations to it
                # Cursor
                docs.move(height-2, len(searchprompt)+1)
                if searching == True:
                    curses.curs_set(2)
                elif searching == False:
                    curses.curs_set(0)
                # refresh
                docs.refresh()
                # wait for char
                if searching == False:
                    inchar = docs.getch()
                    if inchar == ord('q'):
                        curses.endwin()
                        sys.exit(0)
                    elif inchar == ord('h'):
                        draw()
                    elif inchar == ord('s'):
                        searching = True
                        continue
                    elif inchar == ord('S'):
                        searching = True
                        descSearch = True
                        continue
                    elif inchar == curses.KEY_UP:
                        if bodyshift == 0:
                            continue
                        else:
                            bodyshift -=1
                    elif inchar == curses.KEY_DOWN:
                        if len(bodylinelist)-1 <= height-11+bodyshift:
                            pass
                        else:
                            bodyshift += 1
                    else:
                        pass
                elif searching == True:
                    instr = docs.getstr().decode(code)
                    drawSearch(instr, descSearch)
                    searching = False
            except KeyboardInterrupt:
                break
    finally:
        curses.endwin()

class SearchResult:
    def __init__(self, aname, adesk, aunimplemented):
        self.name = aname
        self.description = adesk
        self.unimplemented = aunimplemented

class Search:
    def __init__(self, sterm):
        allfiles = []
        for f in os.scandir(os.path.dirname(os.path.abspath(__file__))+'/features'):
            if f.name != '__init__.py':
                allfiles.append(f.path)
        self.term = sterm
        self.files = allfiles
    def find(self, desc=False):
        self.results = []
        for efile in self.files:
            f = open(efile, 'r')
            decoded = Decode(f.read().replace('\n',''))                     # JSON files must have \\n instead of '\n'
            if self.term in decoded["tags"]:
                self.results.append(SearchResult(decoded["name"], decoded["desc"], not decoded["implemented"]))
                self.files[self.files.index(efile)] = None
            if desc == True:
                if self.term in decoded["tags"]:
                    self.results.append(SearchResult(decoded["name"], decoded["desc"], not decoded["implemented"]))
                    self.files[self.files.index(efile)] = None
                elif self.term in ''.join(decoded["desc"]):
                    self.results.append(SearchResult(decoded["name"], decoded["desc"], not decoded["implemented"]))
                    self.files[self.files.index(efile)] = None
            f.close()
        return self.results

def drawSearch(search_term, descriptive = False, searchprompt = 'Search documentation: '):
    # defined per instance draw/search vars
    initsearch = Search(search_term)
    search_results = initsearch.find(descriptive)
    selected = [0,0,0] # 0 - numberresults, index in search_results, shift down
    searching = False
    numberresults = (height-11)//4
    if len(search_results) < numberresults:
        numberresults = len(search_results)
    try:
        while True:
            try:
                # vars
                charsmax = width-3-len(searchprompt)
                # clear buffer
                docs.clear()
                # titles
                docs.addnstr(1, 1, "BlackRose scripting language documentation", width-3, title_style)
                docs.addnstr(2, 1, "BlackRose version "+version, width-3, title_style)
                if descriptive == False:
                    docs.addnstr(4, 1, "Search: "+search_term, width-3, subsection_style)
                else:
                    docs.addnstr(4, 1, "Description search: "+search_term, width-3, subsection_style)
                # body
                if len(search_results) == 0:
                    docs.addnstr(6,3,"Sorry, no results. (Try searching agian?)", width-5, body_style)
                else:
                    for n in range(numberresults):
                        if n == selected[0]:
                            if search_results[selected[1]].unimplemented == True:
                                docs.addnstr(6+4*n, 3, search_results[selected[1]].name, width-5, unimplemented_style | curses.A_BOLD | curses.A_STANDOUT)
                                docs.addnstr(7+4*n, 5, search_results[selected[1]].description[0], width-7, unimplemented_style | curses.A_STANDOUT)
                                docs.addnstr(8+4*n, 5, search_results[selected[1]].description[1], width-7, unimplemented_style | curses.A_STANDOUT)
                            else:
                                docs.addnstr(6+4*n, 3, search_results[selected[1]].name, width-5, body_style | curses.A_BOLD | curses.A_STANDOUT)
                                docs.addnstr(7+4*n, 5, search_results[selected[1]].description[0], width-7, body_style | curses.A_STANDOUT)
                                docs.addnstr(8+4*n, 5, search_results[selected[1]].description[1], width-7, body_style | curses.A_STANDOUT)
                        elif search_results[selected[2]+n].unimplemented == True:
                            docs.addnstr(6+4*n, 3, search_results[selected[2]+n].name, width-5, unimplemented_style | curses.A_BOLD)
                            docs.addnstr(7+4*n, 5, search_results[selected[2]+n].description[0], width-7, unimplemented_style)
                            docs.addnstr(8+4*n, 5, search_results[selected[2]+n].description[1], width-7, unimplemented_style)
                        else:
                            docs.addnstr(6+4*n, 3, search_results[selected[2]+n].name, width-5, body_style | curses.A_BOLD)
                            docs.addnstr(7+4*n, 5, search_results[selected[2]+n].description[0], width-7, body_style)
                            docs.addnstr(8+4*n, 5, search_results[selected[2]+n].description[1], width-7, body_style)
                #key
                for n, line in enumerate(key):
                    docs.addstr(height-4, 1+len(''.join(key[0:n]))+2*n, line, key_style)
                # Search
                docs.addstr(height-2, 1, searchprompt, search_style)
                # Border
                docs.border(0)  # Border drawn later to reduce alterations to it
                # Cursor
                docs.move(height-2, len(searchprompt)+1)
                if searching == True:
                    curses.curs_set(2)
                elif searching == False:
                    curses.curs_set(0)
                # refresh
                docs.refresh()
                # wait for char
                if searching == False:
                    inchar = docs.getch()
                    if inchar == ord('q'):
                        curses.endwin()
                        sys.exit(0)
                    elif inchar == ord('h'):
                        draw()
                    elif inchar == ord('s'):
                        searching = True
                        continue
                    elif inchar == curses.KEY_UP:
                        if selected[1] == 0:
                            continue
                        elif selected[0] > 0:
                            selected[0] -= 1
                            selected[1] -= 1
                        elif selected[2] > 0:
                            selected[2] -= 1
                            selected[1] -= 1
                    elif inchar == curses.KEY_DOWN:
                        if selected[1] == len(search_results)-1:
                            continue
                        elif selected[0] < numberresults:
                            selected[0] += 1
                            selected[1] += 1
                        elif selected[0] == numberresults:
                            selected[1] += 1
                            selected[2] += 1
                    elif inchar == ord('\n'):
                        drawSelect(search_results[selected[1]])
                        curses.endwin()
                        sys.exit(0)
                    else:
                        pass
                elif searching == True:
                    instr = docs.getstr().decode(code)
                    drawSearch(instr)
                    searching = False
            except KeyboardInterrupt:
                break
    finally:
        curses.endwin()

def drawSelect(result, searchprompt = 'Search documentation: '):
    body = [result.name]
    if result.unimplemented == True:
        body.append("Warning! Unimplemented feature. Please wait until it's stable.")
    body.append('')
    for line in result.description:
        body.append("  "+line)
    draw(body,['home',result.name],searchprompt)



draw()
