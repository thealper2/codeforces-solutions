n = int(input())
events = list(map(int, input().split()))

officers = 0
untreated = 0

for e in events:
	if e == -1:
		if officers > 0:
			officers -= 1
		else:
			untreated += 1
	else:
		officers += e

print(untreated)
