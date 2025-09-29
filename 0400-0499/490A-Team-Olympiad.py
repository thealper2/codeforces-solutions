n = int(input())
t = list(map(int, input().split()))

prog = []
maths = []
pe = []

for i, val in enumerate(t, 1):
    if val == 1:
        prog.append(i)
    elif val == 2:
        maths.append(i)
    else:
        pe.append(i)

teams = min(len(prog), len(maths), len(pe))
print(teams)
for i in range(teams):
    print(prog[i], maths[i], pe[i])
