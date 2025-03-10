#include <stdio.h>
#include <string.h>

void naivePatternSearch(const char *text, const char *pattern)
{
  int textLen = strlen(text);
  int patternLen = strlen(pattern);

  for (int i = 0; i <= textLen - patternLen; i++)
  {
    if (memcmp(&text[i], pattern, patternLen) == 0)
    {
      printf("Pattern found at position %d\n", i);
    }
  }
}

int main()
{
  const char *text = "abcabc56777aba22abc323abcd";
  const char *pattern = "abc";

  naivePatternSearch(text, pattern);

  return 0;
}
