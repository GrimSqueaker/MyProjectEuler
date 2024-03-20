#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

int main(void)
{
  vector<int> primes;
  primes.reserve(2000000);
  primes.push_back(2);
  int next = 3;
  long long sum = 2;

  while (*(primes.end()-1) < 2000000) {

    bool isprime = true;

    for (int test: primes)
      if (next%test == 0) {
        isprime = false;
        break;
      }

    if (isprime) {
      primes.push_back(next);
      sum += next;
      cout << ".";
    }

    next += 2;
  }

  sum -= *(primes.end()-1);

  cout << " the sum of all the primes below two million = " << sum << endl;

  return 0;
}
