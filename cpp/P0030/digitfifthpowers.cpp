#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

int digitFourthPowerSum(int num)
{
  int sum = 0;

  while (num > 0) {
    int digit = num%10;
    sum += digit*digit*digit*digit;
    num /= 10;
  }

  return sum;
}

int digitFifthPowerSum(int num)
{
  int sum = 0;

  while (num > 0) {
    int digit = num%10;
    sum += digit*digit*digit*digit*digit;
    num /= 10;
  }

  return sum;
}

int main(void)
{
  int sum = 0;

  for (int i = 2; i <= 1000000; i++)
    if (i == digitFifthPowerSum(i)) {
      cout << i << " can be written as sum of fourth powers of its digits" << endl;
      sum += i;
    }

  cout << "result: " << sum << endl;

  return 0;
}
