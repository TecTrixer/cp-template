#include <vector>
using namespace std;
//---START---
const int N = 20;
int lp[N + 1];
void create_lp_sieve() {
  vector<int> pr;
  for (int i = 2; i <= N; i++) {
    if (lp[i] == 0) {
      lp[i] = i;
      pr.push_back(i);
    }
    for (int j = 0; i * pr[j] <= N; j++) {
      lp[i * pr[j]] = pr[j];
      if (pr[j] == lp[i])
        break;
    }
  }
}
//---END---
