n = int(input())
coins = list(map(int, input().split()))

coins.sort(reverse=True)
total = sum(coins)
curr = 0
cnt = 0

for c in coins:
    curr += c
    cnt += 1
    if curr > total - curr:
        break

print(cnt)
