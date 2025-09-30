x = input().strip()
result = []

for i, digit in enumerate(x):
    d = int(digit)
    if i == 0:
        if d == 9:
            result.append('9')
        else:
            inverted = 9 - d
            result.append(str(min(d, inverted)))
    else:
        inverted = 9 - d
        result.append(str(min(d, inverted)))

print(''.join(result))
