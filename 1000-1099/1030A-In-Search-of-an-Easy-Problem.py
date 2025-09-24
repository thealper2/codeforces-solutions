n = int(input())
arr = map(int, input().split())

if all(item == 0 for item in arr):
	print('EASY')
else:
	print('HARD')
