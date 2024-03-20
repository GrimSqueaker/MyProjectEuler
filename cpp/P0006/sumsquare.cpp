#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

int main(void)
{
  long long sum = 0;
  long long squaresum = 0;

  for (int i = 1; i <= 100; i++) {
    sum += i;
    squaresum += i*i;
  }

  long long sumsquare = sum*sum;

  cout << " the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum = " << sumsquare - squaresum << endl;

  return 0;
}
