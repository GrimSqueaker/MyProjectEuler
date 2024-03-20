#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <numeric>

using namespace std;

vector<int> getProperDivisors(int i)
{
  vector<int> divisors = {1};

  for (int t = 2; t*t < i; t++) {
    if (i%t == 0) {
      divisors.push_back(t);
      if (t != i/t)
        divisors.push_back(i/t);
    }
  }

  return divisors;
}

bool isAmicableNumber(int i) 
{
  vector<int> divi = getProperDivisors(i);
  int sumi = accumulate(divi.begin(), divi.end(), 0);

  vector<int> divsum = getProperDivisors(sumi);
  int sumsum = accumulate(divsum.begin(), divsum.end(), 0);

  if (i == sumsum && i != sumi) {
    cout << "d(" << i << ") = " << sumi << " = ";
    for (auto a: divi)
      cout << a << " + ";
    cout << endl;
    cout << "d(" << sumi << ") = " << sumsum << " = ";
    for (auto a: divsum)
      cout << a << " + ";
    cout << endl << endl;

    return true;
  }

  return false;
}

int main(void)
{
  long long sum = 0;

  for (int i = 1; i < 10000; i++) {
    if (isAmicableNumber(i))
      sum += i;
  }

  cout << "sum of all the amicable numbers under 10000 = " << sum << endl;

  return 0;
}
