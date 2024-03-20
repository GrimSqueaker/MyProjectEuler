#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

int main(void)
{
  vector<int> primes;
  primes.push_back(2);
  int next = 3;

  while (primes.size() < 10001) {

    bool isprime = true;

    for (int test: primes)
      if (next%test == 0) {
        isprime = false;
        break;
      }

    if (isprime) {
      primes.push_back(next);
      cout << "prime: " << next << endl;
    }

    next++;
  }

  cout << "10001st prime number = " << primes[10000] << endl;

  return 0;
}
