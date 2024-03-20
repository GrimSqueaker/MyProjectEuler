#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <gmpxx.h>

using namespace std;

int main(void)
{
  mpz_class f1 = 1;
  mpz_class f2 = 1;
  mpz_class fn = 2;
  long long count = 3;

  mpz_class stop;
  mpz_ui_pow_ui( stop.get_mpz_t(), 10, 999 );
  
  while (fn < stop) {
    count = count + 1;
    f1 = f2;
    f2 = fn;
    fn = f1+f2;

    if (count % 100 == 0)
      cout << "F" << count << " = " << fn << endl;
  }

  cout << "result: F(" << count << ") = " << fn << endl;

  return 0;
}
