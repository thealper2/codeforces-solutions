x = int(input())
a = x // 5
x -= a * 5
b = 1 if x % 5 else 0
print(a + b)
