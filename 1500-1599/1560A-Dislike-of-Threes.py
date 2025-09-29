t = int(input())
ks = [int(input()) for _ in range(t)]
seq = []
x = 1
while len(seq) < max(ks):
	if x % 3 != 0 and x % 10 != 3:
		seq.append(x)

	x += 1

for k in ks:
	print(seq[k - 1])
