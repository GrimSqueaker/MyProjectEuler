#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <array>
#include <numeric>

using namespace std;

int main(void)
{
  int maxind = 0;
  int maxlen = 0;

  array<int,1000> visited;

  for (int i = 2; i < 1000; i++) {
    visited.fill(0);
    visited[1] = 1;

    int remainder = 1;
    int cyclelen = 0;
    int step = 1;

    while (remainder != 0 && cyclelen == 0) {
      step++;
      remainder *= 10;

      if (remainder >= i) {
        remainder = remainder%i;
        if (remainder) {
          //cout << remainder << "," << visited[remainder] << endl;
          if (visited[remainder])
            cyclelen = step - visited[remainder];
          else
            visited[remainder] = step;
        }
      }
    }

    cout << "d = " << i << ", cycle length = " << cyclelen << endl;

    if (cyclelen > maxlen) {
      maxlen = cyclelen;
      maxind = i;
    }
  }

  cout << "result: d = " << maxind << ", cycle length = " << maxlen << endl;

  return 0;
}
