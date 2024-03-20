#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

int main(void)
{
  int longest = 0;
  int longeststart = 0;

  for (int i = 2; i < 1000000; i++) {
    long long term = i;
    int len = 1;

    while (term != 1) {
      if (term%2 == 0)
        term /= 2;
      else
        term = 3*term + 1;

      len++;
    }

    if (longest < len) {
      longeststart = i;
      longest = len;
    }

    cout << "number " << i << " has " << len << " Collatz sequence terms" << endl;
  }

  cout << "Which starting number, under one million, produces the longest chain? " << longeststart << " (" << longest << " length)" << endl;

  return 0;
}
