n = int(input())
for _ in range(n):
	b = input()
	a = ''
	a += b[:2]
	for i in range(3, len(b), 2):
		a += b[i]

	print(a)
