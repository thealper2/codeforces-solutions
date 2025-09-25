n = int(input())
home = []
guest = []

for _ in range(n):
	h, a = map(int, input().split())
	home.append(h)
	guest.append(a)

freq = {}
for g in guest:
	if g not in freq:
		freq[g] = 1
	else:
		freq[g] += 1

result = 0
for h in home:
	result += freq.get(h, 0)

print(result)
