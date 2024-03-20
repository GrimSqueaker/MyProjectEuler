#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

/* too slow...
 
int gridpaths(int w, int h) {
  if (w == 1)
    return h+1;
  if (h == 1)
    return w+1;

  int paths = 1; // go down completely => 1 path

  for (int i = h; i > 0; i--)
    paths += gridpaths(w-1,i);

  return paths;
}
*/

int main(void)
{
  long long paths[21][21] {{0}};

  for (int i = 1; i < 21; i++) {
    paths[i][1] = i+1;
    paths[1][i] = i+1;
  }

  for (int i = 2; i < 21; i++)
    for (int j = i; j < 21; j++) {
      long long p = 1;
      for (int x = 1; x <= j; x++)
        p += paths[i-1][x];
      paths[i][j] = p;
      paths[j][i] = p;
    }

  cout << "How many such routes are there through a 1×2 grid? " << paths[1][2] << endl;
  cout << "How many such routes are there through a 2×1 grid? " << paths[2][1] << endl;
  cout << "How many such routes are there through a 2×2 grid? " << paths[2][2] << endl;
  cout << "How many such routes are there through a 2×3 grid? " << paths[2][3] << endl;
  cout << "How many such routes are there through a 3×2 grid? " << paths[3][2] << endl;
  cout << "How many such routes are there through a 3×3 grid? " << paths[3][3] << endl;
  cout << "How many such routes are there through a 20×20 grid? " << paths[20][20] << endl; // 407575348

  for (int i = 1; i < 21; i++) {
    for (int j = 1; j < 21; j++) {
      cout << paths[i][j] << ", ";
    }
    cout << endl;
  }


  return 0;
}
