n, a, b, c = map(int, input().split())
NEG = -10**9
dp = [NEG] * (n + 1)
dp[0] = 0

for i in range(1, n + 1):
	for x in (a, b, c):
		if i >= x and dp[i - x] != NEG:
			dp[i] = max(dp[i], dp[i - x] + 1)

print(dp[n])
