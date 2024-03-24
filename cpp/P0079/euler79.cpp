#include <iostream>
#include <vector>

using namespace std;


int main( void )
{
  vector<int> numbers = {
    319, 680, 180, 690, 129,
    620, 762, 689, 762, 318,
    368, 710, 720, 710, 629,
    168, 160, 689, 716, 731,
    736, 729, 316, 729, 729,
    710, 769, 290, 719, 680,
    318, 389, 162, 289, 162,
    718, 729, 319, 790, 680,
    890, 362, 319, 760, 316,
    729, 380, 319, 728, 716
  };

  for ( long long test = 10000000; true; test++ ) {
    bool solution = true;

    for (int num: numbers) {
      long long tmptest = test;
      int tmpnum = num;
      int n = tmpnum%10;

      while (tmptest > 0 && tmpnum > 0) {
        if (n == tmptest%10) {
          tmpnum /= 10;
          n = tmpnum % 10;
        }

        tmptest /= 10;
      }

      if (tmpnum != 0) {
        solution = false;
        break;
      }
    }

    if (solution) {
      cout << "solution = " << test << endl;
      return 0;
    }

    if (test % 10000000 == 0)
      cout << "current number = " << test << endl;
  }

  return 0;
}
