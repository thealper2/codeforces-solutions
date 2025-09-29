n = int(input())
a = list(map(int, input().split()))
max_val = max(a)
cnt = [0] * (max_val + 1)

for num in a:
	cnt[num] += 1

dp = [0] * (max_val + 1)
dp[1] = cnt[1] * 1

for i in range(2, max_val + 1):
	dp[i] = max(dp[i - 1], dp[i - 2] + cnt[i] * i)

print(dp[max_val])
