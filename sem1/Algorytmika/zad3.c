#include <stdio.h>
#include <string.h>

int longestSuffixPrefix(const char *x, const char *y)
{
  int n = strlen(x);
  int m = strlen(y);
  int k = 0;
  
  while (k < n && k < m && x[k] == y[m - k - 1])
  {
    k++;
  }
  return k;
}

int main()
{
  char x[] = "abcdef";
  char y[] = "xyzdef";
  printf("Max k: %d\n", longestSuffixPrefix(x, y));

  char z[] = "4gt6yu";
  char w[] = "uy4tg4";
  printf("Max k: %d\n", longestSuffixPrefix(z, w));
  return 0;
}
