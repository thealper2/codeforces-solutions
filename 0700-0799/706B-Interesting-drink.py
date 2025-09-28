import bisect


n = int(input())
prices = list(map(int, input().split()))
q = int(input())
m = [int(input()) for _ in range(q)]

prices.sort()
for coins in m:
	cnt = bisect.bisect_right(prices, coins)
	print(cnt)
