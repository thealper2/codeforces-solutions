n, k, l, c, d, p, nl, np = map(int, input().split())

total_drink = k * l
drink_toasts = total_drink // nl
lime_toasts = c * d
salt_toasts = p // np

total_toasts = min(drink_toasts, lime_toasts, salt_toasts)
print(total_toasts // n)
