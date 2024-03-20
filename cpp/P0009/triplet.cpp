#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <string>

using namespace std;

int main(void)
{
  for (int a = 1; a < 1000; a++)
    for (int b = a; b < 1000; b++) {
      int c = 1000 - a - b;
      if (a*a + b*b == c*c) {
        cout << "a = " << a << ", b = " << b << ", c = " << c << endl
             << "a*b*c = " << a*b*c << endl;
      }
    }

  return 0;
}
