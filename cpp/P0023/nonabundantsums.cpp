#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <numeric>

using namespace std;

enum NumberType {perfect, abundant, deficient};

int getProperDivisorsSum(int i)
{
  int sum = 0;

  for (int t = 2; t*t <= i; t++) {
    if (i%t == 0) {
      sum += t;
      if (t != i/t)
        sum += i/t;
    }
  }

  return sum;
}

NumberType getNumberType(int i)
{
  int propdivsum = getProperDivisorsSum(i);

  if (propdivsum < i)
    return deficient;
  else if (propdivsum == i)
    return perfect;
  else
    return abundant;
}

bool isSumOfTwo(int test, const vector<int> &numbers)
{
  // assumes that the numbers vector is sorted 
  // and has at least one element

  auto forward = numbers.cbegin();
  auto backward = numbers.crbegin();

  backward++;

  while (forward != numbers.cend()
         && backward != numbers.crend()
         && *forward <= *backward)
  {
    while (backward != numbers.crend()
           && *forward + *backward > test)
      backward++;

    if (backward != numbers.crend()
        && test == *forward + *backward)
        return true;

    forward++;
  }

  return false;
}

int main(void)
{
  vector<int> notsum;
  vector<int> abundantnumbers;

  // find all abundant numbers up to 28123
  for (int i = 12; i <= 28123; i++) {
    if (abundant == getNumberType(i))
      abundantnumbers.push_back(i);
  }

  // find all positive integers that cannot be written as the sum of two abundant numbers
  for (int i = 1; i <= 28123; i++) {
    if (!isSumOfTwo(i, abundantnumbers))
      notsum.push_back(i);
  }

  // compute sum
  int sum = accumulate(notsum.begin(), notsum.end(), 0);

  cout << "result = " << sum << endl;

  return 0;
}
