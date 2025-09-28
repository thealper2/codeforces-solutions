n = int(input())
arr = list(map(int, input().split()))

max_welfare = max(arr)
total_spent = sum(max_welfare - i for i in arr)
print(total_spent)
