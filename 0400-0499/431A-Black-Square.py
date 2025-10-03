calories = list(map(int, input().split()))
s = input()
result = 0
for c in s:
	d = int(c)
	result += calories[d - 1]

print(result)
