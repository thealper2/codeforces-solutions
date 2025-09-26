n, l = map(int, input().split())
a = list(map(int, input().split()))

a.sort()

max_gap = max(a[0], l - a[-1])
for i in range(n - 1):
	gap = a[i + 1] - a[i]
	max_gap = max(max_gap, gap / 2.0)

print(round(max_gap, 10))
