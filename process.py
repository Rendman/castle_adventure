file = open('title.txt', 'r')
outfile = open("title.out", 'w')

prettyprint = True

lines = file.readlines()

lines.reverse()

indexcount = 0
outstring = list()

thisstring = ""

for lineidx, line in enumerate(lines):
    for character in line:
        if character == '\n':
            outstring.append(thisstring)
            continue
        
        thisstring += "'"
        
        if character == ' ':
            thisstring += ' '
        else:
            thisstring += character

        thisstring += "',"
        indexcount += 1

    ## account for the issue of no new line being placed at the end of the last line in the file
    if lineidx == 0:
        outstring.append(thisstring)
    
    thisstring = ""

outfile.write("// TODO: Change the name of this array\n")
outfile.write("const CHANGE_ME__: [char; {0}] = [".format(indexcount))
if prettyprint:
    outfile.write("\n")
for str in outstring:
    outfile.write(str)
    if prettyprint:
        outfile.write("\n")
outfile.write("];")

outfile.close()
file.close()