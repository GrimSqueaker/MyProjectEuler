import math

# We only need to look at most at 7 digit numbers, because 8*9! = 2903040
# This means that the largest digit factorial number of an 8 digit number is a 7 digit number
# and therefore no >7 digit number can be the sum of the factorials of its digits

all_sum = 0
for number in range(10, 10000000):
    digits = [int(d) for d in str(number)]
    factorials = [math.factorial(d) for d in digits]
    current_sum = sum(factorials)
    if (current_sum == number):
        print(f"{number} is the sum of the factorials of its digits")
        all_sum = all_sum + number

print(f"Solution: {all_sum}")
# Solution: 40730
