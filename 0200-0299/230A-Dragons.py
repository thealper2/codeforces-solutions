s, n = map(int, input().split())
result = 'YES'
dragons = []
for _ in range(n):
	ds, bonus = map(int, input().split())
	dragons.append([ds, bonus])

dragons.sort(key=lambda x: x[0])

for dragon in dragons:
	ds, bonus = dragon
	if s > ds:
		s += bonus
	else:
		result = 'NO'
		break

print(result)
