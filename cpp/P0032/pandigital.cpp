#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <array>

// 99 * 99 < 100 * 100 = 10000
// => largest product to check is less than 10000


using namespace std;

// returns false if one of the digits is already checked in the digits array
bool checkDigits( array<bool, 9> &digits, int number )
{
  while (number) {
    if (number % 10 == 0)
      return false;
    else if (digits[number%10-1])
      return false;
    else {
      digits[number%10-1] = true;
      number /= 10;
    }
  }

  return true;
}

bool isPandigital( int multiplicand, int multiplier, int product )
{
  array<bool, 9> digits = {{false}};

  bool good = checkDigits( digits, multiplicand )
      && checkDigits( digits, multiplier )
      && checkDigits( digits, product );

  for (bool d: digits)
    good = good && d;

  return good;
}

int main(void)
{
  int sumofprods = 0;
  int countpandi = 0;

  for (int prod = 1; prod < 10000; prod++) {
    bool isPandi = false;
    for (int mult = 2; (mult*mult <= prod) && !isPandi; mult++) {
      if (prod % mult == 0) {
        if (isPandigital(mult, prod/mult, prod)) {
          cout << mult << " x " << prod/mult << " = " << prod << endl;
          isPandi = true;
          sumofprods += prod;
          countpandi++;
        }
      }
    }
  }

  cout << "number of pandigitals: " << countpandi << endl;
  cout << "sum of products of pandigitals: " << sumofprods << endl;

  return 0;
}
