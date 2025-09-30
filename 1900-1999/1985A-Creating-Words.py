n = int(input())
for _ in range(n):
	a, b = input().split()
	a, b = b[0] + a[1:], a[0] + b[1:]
	print(a, b)
