#include <iostream>
#include <algorithm>
#include <vector>
#include <gmpxx.h>

using namespace std;

int main(void)
{
  vector<mpz_class> combinations;
  combinations.reserve(100*100);

  for (int a = 2; a <= 100; a++)
    for (int b = 2; b <= 100; b++) {
      mpz_class z;
      mpz_ui_pow_ui( z.get_mpz_t(), a, b );
      combinations.push_back(z);
    }

  sort(combinations.begin(), combinations.end());
    
  int distinct = distance(combinations.begin(),
                          unique(combinations.begin(),
                                 combinations.end()));
  cout << "result: " << distinct << endl;

  return 0;
}
