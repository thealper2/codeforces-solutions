n = int(input())
result = 0
drinks = map(int, input().split())
for drink in drinks:
	result += drink / (100 * n)

result *= 100
print(result)
