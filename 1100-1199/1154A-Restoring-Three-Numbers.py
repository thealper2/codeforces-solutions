digits = list(map(int, input().split()))
digits.sort()
total = digits[3]
a = total - digits[2]
b = total - digits[1]
c = total - digits[0]
print(a, b, c)
