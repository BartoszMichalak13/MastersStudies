#include <stdio.h>
#include <string.h>

#define ALPHABET_SIZE 256
#define MAX_PATTERN_LEN 100

int transition[MAX_PATTERN_LEN + 1][ALPHABET_SIZE];

void buildPrefixAutomaton(const char *pattern)
{
  int m = strlen(pattern);
  memset(transition, 0, sizeof(transition));

  for (int state = 0; state <= m; state++)
  {
    for (int c = 0; c < ALPHABET_SIZE; c++)
    {
      char temp[MAX_PATTERN_LEN + 2];
      strncpy(temp, pattern, state);
      temp[state] = (char)c;
      temp[state + 1] = '\0';

      int nextState = 0;
      for (int i = state + 1; i > 0; i--)
      {
        if (strncmp(temp + state + 1 - i, pattern, i) == 0)
        {
          nextState = i;
          break;
        }
      }
      transition[state][c] = nextState;
    }
  }
}

void searchPattern(const char *text, const char *pattern)
{
  int n = strlen(text);
  int m = strlen(pattern);
  int state = 0;

  printf("Pattern \"%s\" found in text at: ", pattern);

  for (int i = 0; i < n; i++)
  {
    state = transition[state][(unsigned char)text[i]];
    if (state == m)
    {
      printf("%d ", i - m + 1);
    }
  }
  printf("\n");
}

int main()
{
  const char *pattern = "abc";
  const char *text = "abc456abcab7c4h645a";

  buildPrefixAutomaton(pattern);
  searchPattern(text, pattern);

  const char *pattern2 = "45";
  buildPrefixAutomaton(pattern2);
  searchPattern(text, pattern2);

  return 0;
}
