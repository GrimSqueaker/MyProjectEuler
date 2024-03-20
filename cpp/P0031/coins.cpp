#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

// Coins 200, 100, 50, 20, 10, 5, 2, 1
vector<int> types = { 200, 100, 50, 20, 10, 5, 2, 1 };

int value(const vector<int> &counter)
{
  int sum = 0;

  for (size_t i = 0; i < types.size(); i++)
    sum += counter[i]*types[i];

  return sum;
}

int count(vector<int> &counter, size_t pos)
{
  if (pos == types.size()) {
    if (value(counter) == 200) {
      for (auto n: counter)
        cout << n << ", ";
      cout << endl;
      return 1;
    }
    else
      return 0;
  }

  int sum = 0;
  for (int c = 0; (c <= 200/types[pos]) && value(counter) <= 200; c++) {
    counter[pos] = c;
    sum += count(counter, pos+1);
  }
  counter[pos] = 0;

  return sum;
}

int main(void)
{
  vector<int> counter(8,0);

  cout << types.size() << endl;
  cout << counter.size() << endl;
  cout << "result: " << count(counter,0) << endl;

  return 0;
}
