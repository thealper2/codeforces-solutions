n = int(input())
prev = None
group = 1

for _ in range(n):
	magnet = str(input())
	a, b = magnet[0], magnet[1]
	if prev and prev == a:
		group += 1

	prev = b

print(group)
