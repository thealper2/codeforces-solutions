shoes = list(map(int, input().split()))
shoes.sort()
n = len(shoes)
cnt = 0

for i in range(n - 1):
	if shoes[i] == shoes[i + 1]:
		cnt += 1

print(cnt)
