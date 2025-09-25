n = int(input())
groups = list(map(int, input().split()))
cnt = [0] * 5
for g in groups:
	cnt[g] += 1

taxis = cnt[4] + cnt[3] + (cnt[2] * 2 + max(0, cnt[1] - cnt[3]) + 3) // 4
print(taxis)
