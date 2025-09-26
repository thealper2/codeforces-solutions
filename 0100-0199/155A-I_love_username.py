n = int(input())
scores = list(map(int, input().split()))

amazing = 0
max_ = scores[0]
min_ = scores[0]

for score in scores[1:]:
	if score > max_:
		amazing += 1
		max_ = score
	elif score < min_:
		amazing += 1
		min_ = score

print(amazing)
