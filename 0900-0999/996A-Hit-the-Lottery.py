n = int(input())
bills = [100, 20, 10, 5, 1]
cnt = 0
for b in bills:
    cnt += n // b
    n %= b

print(cnt)
