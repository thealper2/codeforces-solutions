n, m = map(int, input().split())
f = list(map(int, input().split()))
f.sort()
ans = float('inf')
for i in range(m - n + 1):
	ans = min(ans, f[i + n - 1] - f[i])

print(ans)
