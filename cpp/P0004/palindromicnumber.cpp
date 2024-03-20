#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>

using namespace std;

bool checkPalindromic(int num)
{
  vector<int> digits;

  while (num != 0) {
    digits.push_back(num%10);
    num /= 10;
  }

  int s = digits.size();
  for (int i = 0; i < s/2; i++)
    if (digits[i] != digits[s-i-1])
      return false;

  return true;
}

int main(void)
{
  int largest = 0;

  for (int num1 = 999; num1 >= 100; num1--)
    for (int num2 = num1; num2 >= 100; num2--)
      if (checkPalindromic(num1*num2)) {
        cout << "found palindrome = "
             << num1*num2 << " = " << num1 << "x" << num2 << endl;
        largest = max(largest,num1*num2);
      }

  cout << "largest palindrome as product of two 3-digit numbers = " << largest << endl;

  return 0;
}
