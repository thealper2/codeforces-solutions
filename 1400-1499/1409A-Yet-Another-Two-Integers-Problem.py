n = int(input())
for _ in range(n):
	a, b = map(int, input().split())
	d = abs(a - b)
	print((d + 9) // 10)
