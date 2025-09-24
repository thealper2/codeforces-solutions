s = input()
target = 'hello'
idx = 0

for c in s:
	if idx < len(target) and c == target[idx]:
		idx += 1

if idx == len(target):
	print('YES')
else:
	print('NO')
