n = int(input())
arr = list(map(int, input().split()))
evens = []
odds = []
for item in arr:
	if item % 2 == 0:
		evens.append(item)
	else:
		odds.append(item)

if len(evens) < len(odds):
	print(arr.index(evens[-1]) + 1)
else:
	print(arr.index(odds[-1]) + 1)

