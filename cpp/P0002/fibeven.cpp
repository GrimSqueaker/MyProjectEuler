#include <iostream>

using namespace std;

int main(void)
{
  int fib1 = 1;
  int fib2 = 2;
  int fib  = 0;

  int sum = 2;

  while (fib < 4000000) {
    fib = fib1 + fib2;

    if (fib%2 == 0)
      sum += fib;

    fib1 = fib2;
    fib2 = fib;
  }

  cout << "result = " << sum << endl;

  return 0;
}
