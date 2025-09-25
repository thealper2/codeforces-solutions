n = int(input())
result = []
for _ in range(n):
	a, b = map(int, input().split())
	if a % b == 0:
		result.append(0)
	else:
		t = a // b
		result.append((t + 1) * b - a)

for item in result:
	print(item)
