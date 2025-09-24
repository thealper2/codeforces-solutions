n = int(input())
games = input().strip()
anton = games.count('A')
danik = games.count('D')
if anton < danik:
	print('Danik')
elif anton > danik:
	print('Anton')
else:
	print('Friendship')
