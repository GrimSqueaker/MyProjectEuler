#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

void extendPrimes(int p, vector<int> &primes)
{
  bool extended = false;
  for (int t = primes.back()+1; t <= p; t++) {
    bool isprime = true;
    for (int check = 0; primes[check]*primes[check]<=t; check++)
      if (t % primes[check] == 0) {
        isprime = false;
        break;
      }
    if (isprime) {
      primes.push_back(t);
      extended = true;
    }
  }

  if (extended)
    cout << "extending primes to " << p << endl;
}

bool checkPrime(int p, vector<int> &primes)
{
  if (p < 2)
    return false;

  if (p > primes.back())
    extendPrimes(p, primes);

  return find(primes.begin(), primes.end(), p) != primes.end();
}

int main(void)
{
  int a, b;
  int maxa = 0, maxb = 0;
  int maxprimes = 0;
  vector<int> primes {2};

  extendPrimes(1000, primes); // precompute some

  for (a = -999; a < 1000; a = a+1)
    for (b = 2; b < 1000; b = b+1) {
      bool isprime;
      int n = 0;
      do {
        int test = n*n + a*n + b;
        isprime = checkPrime(test,primes);
        n++;
      } while (isprime);
      n--;

      if (n > maxprimes) {
        maxa = a;
        maxb = b;
        maxprimes = n;
        cout << "a = " << a << " and b = " << b << " yields " << n << " primes" << endl;
      }
    }

  cout << "result: " << maxa*maxb
       << " for a = " << maxa << " and b = " << maxb
       << " with " << maxprimes << " primes" << endl;

  return 0;
}
