#include <iostream>
#include <cmath>

using namespace std;

int main(void)
{
  long long num = 600851475143;
  long long stop = sqrt(num);

  int test = 2;

  while (test <= stop) {
    if (num % test == 0) {
      cout << "found prime factor " << test << endl;
      num /= test;
      stop = sqrt(num);
    }
    else {
      test++;
    }
  }

  cout << "largest prime factor of 600851475143 = " << num << endl;

  return 0;
}
