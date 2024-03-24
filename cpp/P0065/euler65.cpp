
#include <iostream>
#include <algorithm>
#include <vector>
#include <gmpxx.h>

using namespace std;

int main(void)
{
  vector<mpz_class> p(110);

  p[0] = 0;
  p[1] = 1;
  p[2] = 2;

  vector<int> a = {
    2,
    1, 2, 1,
    1, 4, 1,
    1, 6, 1,
    1, 8, 1,
    1, 10, 1,
    1, 12, 1,
    1, 14, 1,
    1, 16, 1,
    1, 18, 1,
    1, 20, 1,
    1, 22, 1,
    1, 24, 1,
    1, 26, 1,
    1, 28, 1,
    1, 30, 1,
    1, 32, 1,
    1, 34, 1,
    1, 36, 1,
    1, 38, 1,
    1, 40, 1,
    1, 42, 1,
    1, 44, 1,
    1, 46, 1,
    1, 48, 1,
    1, 50, 1,
    1, 52, 1,
    1, 54, 1,
    1, 56, 1,
    1, 58, 1,
    1, 60, 1,
    1, 62, 1,
    1, 64, 1,
    1, 66, 1
  };

  for (int n = 0; n < 100; n++) {
    p[n+2] = a[n] * p[n-1+2] + p[n-2+2];
    cout << "n = " << n+1 << " p(n) = " << p[n+2] << endl;
  }

  //    mpz_class z;
  //    mpz_ui_pow_ui( z.get_mpz_t(), a, b );

  mpz_class quer = 0;
  mpz_class zahl = p[101];
  while (zahl != 0) {
    quer = quer + zahl % 10;
    zahl = zahl / 10;
  }
  cout << "result: " << quer << endl;

  return 0;
}

