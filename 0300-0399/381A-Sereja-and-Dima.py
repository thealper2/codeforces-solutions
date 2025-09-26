n = int(input())
cards = list(map(int, input().split()))
l, r = 0, n - 1
sereja, dima = 0, 0
turn = 0

while l <= r:
	if cards[l] > cards[r]:
		choice = cards[l]
		l += 1
	else:
		choice = cards[r]
		r -= 1

	if not turn:
		sereja += choice
	else:
		dima += choice

	turn = not turn

print(sereja, dima)
