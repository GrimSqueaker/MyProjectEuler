import math
import numpy as np

max_num = 1000000

# use a sieve to obtain all primes < 1000000
sieve = np.ones(max_num, dtype=bool)
sieve[0] = False
sieve[1] = False

for check in range(2, int(math.sqrt(max_num))):
    remove = check + check
    while remove < max_num:
        sieve[remove] = False
        remove = remove + check

# check: print all primes below 100
print(f"Primes below 100")
for num in range(100):
    if sieve[num]:
        print(num, end=' ')
print("")

# helper function to get circular numbers
def get_circular_numbers(number):
    list_of_digits = []
    num = number
    while num > 0:
        list_of_digits.append(num%10)
        num //= 10
    list_of_digits.reverse()

    circulars = []

    for circ in range(len(list_of_digits)):
        shifted_list = [*list_of_digits[circ:], *list_of_digits[:circ]]
        number = shifted_list[0]
        for num in shifted_list[1:]:
            number *= 10
            number += num
        circulars.append(number)
    
    return circulars

# check for all primes if the circular numbers are also prime and count
num_circular_primes = 0

print(f"Circular primes below {max_num}")
for prime in range(2, max_num):
    if sieve[prime]:
        circulars = get_circular_numbers(prime)
        primes = [sieve[p] for p in circulars]
        if all(primes):
            num_circular_primes += 1
            print(prime, end=" ")

print("")
print(f"Solution: # Circular primes below {max_num}")
print(num_circular_primes)
# Solution: 55