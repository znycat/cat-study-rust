#include <stdio.h>

struct Error {
  char expected;
  int line;
};

extern int return_err1(int t);
extern char *return_err2(int t);
extern struct Error return_err3();

int main() {
  int err_code = return_err1(1);
  printf("err_code: %d\n", err_code);

  char *err_str = return_err2(2);
  printf("err_str: %s\n", err_str);

  struct Error err = return_err3();
  printf("err.expected: %c\n", err.expected);
  printf("err.line: %d\n", err.line);

  return 0;
}
