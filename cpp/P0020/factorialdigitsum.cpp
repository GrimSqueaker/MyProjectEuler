#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <gmpxx.h>

using namespace std;

int main(void)
{
  mpz_class a;
  mpz_class fact = 1;
  mpz_class sumdigits = 0;

  for (a = 1; a < 101; a = a + 1) {
    fact = fact*a;
  }

  cout << "100! = " << fact << endl;

  while (fact != 0) {
    sumdigits += fact%10;
    fact /= 10;
  }

  cout << "sum of the digits of 100! = " << sumdigits << endl;

  return 0;
}
