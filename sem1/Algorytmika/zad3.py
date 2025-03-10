def longestSuffixPrefix(x: str, y: str) -> int:
  n, m = len(x), len(y)
  k = 0
  while k < n and k < m and x[k] == y[m - k - 1]:
    k += 1
  return k

x = "abcdef"
y = "cdef"
print("Max k:", longestSuffixPrefix(x, y))

x = "abcdef"
y = "56ccba"
print("Max k:", longestSuffixPrefix(x, y))