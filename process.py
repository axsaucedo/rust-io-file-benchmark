
import os, sys

in_path = sys.argv[1]
out_path = sys.argv[2]

in_f = open(in_path)

with open(out_path, "w") as out_f: 
    idx = 0
    row = []
    for l in in_f.readlines()[1:]:
        if idx < 3:
            row.append(l.replace("\n", "")[2:])
            row.append('\t')
            idx = idx + 1
        else:
            row.append("\n")
            out_f.write("".join(row))
            row = []
            idx = 0



