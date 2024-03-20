from fractions import Fraction

# enumerate all fractions with
# - one digit in numerator and denominator
# - value < 1
single_digit_fractions = []
for numer in range(1,9):
    for denom in range(numer+1, 10):
        single_digit_fractions.append((numer, denom))

print(single_digit_fractions)

cancelling_fractions = []
# expand to all possible 2 digit fractions
for frac in single_digit_fractions:
    for digit in range(1,10):
        double_digits = [
            (10*digit+frac[0], 10*digit+frac[1]),
            (10*digit+frac[0], 10*frac[1]+digit),
            (10*frac[0]+digit, 10*digit+frac[1]),
            (10*frac[0]+digit, 10*frac[1]+digit)
            ]
        # check if expanded and original fraction have the same value
        for test in double_digits:
            if (Fraction(*frac) == Fraction(*test)):
                print(f"{frac} == {test}")
                cancelling_fractions.append(Fraction(*test))

solution = Fraction(1,1)
for cf in cancelling_fractions:
    solution = solution*cf

print(solution.denominator)
