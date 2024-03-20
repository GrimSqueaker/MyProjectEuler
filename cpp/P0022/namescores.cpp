#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
#include <fstream>
#include <regex>

using namespace std;


int stringScore(const string &str)
{
  int score = 0;

  for (const char &c: str)
    score += c - 'A' + 1;

  return score;
}

int main(void)
{
  ifstream infile("p0022_names.txt");
  string line;

  infile >> line;

  vector<string> names;

  std::regex ws_re(","); // whitespace
  std::copy( std::sregex_token_iterator(line.begin(), line.end(), ws_re, -1),
             std::sregex_token_iterator(),
             back_inserter(names) );

  for (auto &s: names) {
    s.pop_back();
    s.erase(0,1);
  }

  sort(names.begin(), names.end());

  long long score = 0;
  for (size_t i = 0; i < names.size(); i++) {
    int sscore = stringScore(names[i]);
    score += sscore*(i+1);

    cout << names[i] << " -> " << sscore << endl;
  }

  cout << "What is the total of all the name scores in the file? " << score << endl;

  return 0;
}
