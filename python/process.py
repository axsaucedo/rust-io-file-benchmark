
import os, sys

in_path = sys.argv[1]
out_path = sys.argv[2]

with open(in_path) as in_f:
    with open(out_path, "w") as out_f:
        idx = 0
        row = []
        # skip first line
        in_f.readline()
        l = in_f.readline()
        while l:
            if idx < 3:
                row.append(l.replace("\n", "")[2:])
                row.append('\t')
                idx = idx + 1
            else:
                row.append("\n")
                out_f.write("".join(row))
                row = []
                idx = 0
            l = in_f.readline()



