n = int(input())
for _ in range(n):
	a, b = map(int, input().split())
	min_v = float('inf')
	for c in range(a, b + 1):
		v = (c - a) + (b - c)
		if v < min_v:
			min_v = v

	print(min_v)

