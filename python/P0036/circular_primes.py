import math
import numpy as np

max_num = 1000000

# helper function to check if list is palindrome
def is_list_palindrome(l: list):
    for i in range(len(l)//2):
        if l[i] != l[-1-i]:
            return False
    return True

def digits_as_list(number: int, base: int):
    list_of_digits = []
    num = number
    while num > 0:
        list_of_digits.append(num%base)
        num //= base
    list_of_digits.reverse()
    return list_of_digits

sum = 0
for number in range(1,max_num):
    decimal_list = digits_as_list(number, 10)
    binary_list = digits_as_list(number, 2)
    if is_list_palindrome(decimal_list) and is_list_palindrome(binary_list):
        print(f"Decimal and binary palindrome: {number} = {binary_list}")
        sum += number

print(f"Solution: {sum}")
# 872187

