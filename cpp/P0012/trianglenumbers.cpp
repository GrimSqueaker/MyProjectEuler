#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

long long divisorscount(long long num)
{
  long long divisors = 2;

  for (long long i = 2; i*i <= num; i++)
    if (num % i == 0)
      divisors += 2;

  long long sqrtnum = sqrt(num);
  if (sqrtnum*sqrtnum == num)
    divisors--;

  return divisors;
}

int main(void)
{
  long long trianglenumber = 0;
  long long i = 1;
  long long divisors;

  while (true) {
    trianglenumber += i++;

    divisors = divisorscount(trianglenumber);
    if (divisors > 500)
      break;

    cout << trianglenumber << " has " << divisors << " divisors" << endl;
  }

  cout << "the value of the first triangle number to have over five hundred divisors = " << trianglenumber << " (" << divisors << " divisors)" << endl;

  return 0;
}
