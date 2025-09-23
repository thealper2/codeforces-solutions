k, n, w = map(int, input().split())
total = sum(k * i for i in range(1, w + 1))
total = max(0, total - n)
print(total)
