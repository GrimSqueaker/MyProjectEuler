#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

vector<int> primefactors(int num)
{
  vector<int> factors(21,0);

  int i = 2;
  while (num > 1) {
    if (num % i == 0) {
      factors[i]++;
      num /= i;
    }
    else
      i++;
  }

  return factors;
}

int main(void)
{
  vector<int> factors(21,0);

  for (int test = 2; test <= 20; test++) {
    auto fact = primefactors(test);
    for (int i=0; i <= 20; i++)
      factors[i] = max(factors[i],fact[i]);
  }
  
  int prod = 1;
  for (int i = 2; i <= 20; i++)
    for (int j = 0; j < factors[i]; j++)
      prod *= i;

  cout << "smallest positive number that is evenly divisible by all of the numbers from 1 to 20 = " << prod << endl;

  return 0;
}
