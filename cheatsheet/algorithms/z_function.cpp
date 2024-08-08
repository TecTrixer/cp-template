#include <algorithm>
#include <string>
#include <vector>
using namespace std;

//---START---
vector<int> z_func(string s) {
  int n = s.length();
  vector<int> z(n);
  for (int i = 0; i < n; i++)
    z[i] = 0;
  int l = 0, r = 0;
  for (int i = 1; i < n; i++) {
    if (i < r)
      z[i] = min(r - i, z[i - l]);
    while (i + z[i] < n && s[z[i]] == s[i + z[i]])
      z[i]++;
    if (i + z[i] > r) {
      l = i;
      r = i + z[i];
    }
  }
  return z;
}
//---END---
