#include <stdio.h>
#include <string.h>
#include <stdbool.h>

bool isPrefix(const char *s1, const char *s2) 
{
  return strncmp(s1, s2, strlen(s1)) == 0;
}

bool isPostfix(const char *s1, const char *s2) 
{
  size_t len1 = strlen(s1);
  size_t len2 = strlen(s2);
  if (len1 > len2) return false;
  return strcmp(s1, s2 + len2 - len1) == 0;
}

int main() 
{
  const char *word = "abcTESTcba";
  const char *prefix = "abc";
  const char *postfix = "T0ba";
  printf("Is '%s' a prefix of '%s'? %d\n", prefix, word, isPrefix(prefix, word));
  printf("Is '%s' a postfix of '%s'? %d\n", postfix, word, isPostfix(postfix, word));
  return 0;
} 
//python functions: https://www.w3schools.com/python/ref_string_startswith.asp
//                  https://www.w3schools.com/python/ref_string_endswith.asp
/*
 * txt = "Hello world"
 *
 * x = txt.endswith("world")
 *
 * print(x)
 */