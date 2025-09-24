n = int(input())
person = 0
max_person = 0
for _ in range(n):
	a, b = map(int, input().split())
	person -= a
	person += b
	if person > max_person:
		max_person = person

print(max_person)
