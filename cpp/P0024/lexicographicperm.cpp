#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <array>
#include <numeric>

using namespace std;

array<long long,10> pos = {{1000000000, 100000000, 10000000, 1000000, 100000,
                           10000, 1000, 100, 10, 1}};

void permute(array<bool,10> &a, int depth, long long permutation, int &counter)
{
  if (depth == 10) {
    counter++;
    if (counter == 1000000) {
      cout << "result = " << permutation << endl;
      exit(0);
    }
    else if (counter % 10000 == 0)
      cout << counter << "-th permutation = " << permutation << endl;
  }
  else {
    for (int i = 0; i < 10; i++) {
      if (a[i] == false) {
        a[i] = true;
        permute(a, depth+1, permutation+i*pos[depth], counter);
        a[i] = false;
      }
    }
  }
}

int main(void)
{
  int counter = 0;
  array<bool,10> numbers = {{false}};

  // enumerate all permutations of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
  permute(numbers, 0, 0, counter);

  return 0;
}
