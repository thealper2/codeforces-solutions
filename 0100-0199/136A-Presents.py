n = int(input())
gifts = map(int, input().split())
arr = []
for i, gift in enumerate(gifts, start=1):
	arr.append((i, gift))

arr.sort(key=lambda x: x[1])
result = ' '.join(str(item[0]) for item in arr)
print(result)
