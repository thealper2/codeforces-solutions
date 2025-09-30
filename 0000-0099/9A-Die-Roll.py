from math import gcd

y, w = map(int, input().split())
need = max(y, w)
num = 6 - need + 1
g = gcd(num, 6)
print(f"{num // g}/{6 // g}")
