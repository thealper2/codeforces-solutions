n, h = map(int, input().split())
person = list(map(int, input().split()))
result = 0
for p in person:
	if p <= h:
		result += 1
	else:
		result += 2

print(result)
