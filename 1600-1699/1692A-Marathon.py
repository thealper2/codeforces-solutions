n = int(input())
for _ in range(n):
	a, b, c, d = map(int, input().split())
	print(sum(1 for e in [b, c, d] if e > a))
