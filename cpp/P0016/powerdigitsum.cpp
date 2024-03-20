#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <gmpxx.h>

using namespace std;

int main(void)
{
  mpz_class a;
  mpz_class sumdigits = 0;

  mpz_ui_pow_ui( a.get_mpz_t(), 2, 1000 );

  cout << "2^1000 = " << a << endl;

  while (a != 0) {
    sumdigits += a%10;
    a /= 10;
    cout << "a = " << a << endl;
  }

  cout << "sum of the digits of 2^1000 = " << sumdigits << endl;

  return 0;
}
