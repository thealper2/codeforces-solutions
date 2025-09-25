n = int(input())
a = list(map(int, input().split()))
max_i = a.index(max(a))
min_i = n - 1 - a[::-1].index(min(a))
moves = max_i + (n - 1 - min_i)
if max_i > min_i:
    moves -= 1

print(moves)
