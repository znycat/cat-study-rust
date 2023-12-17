
#include <stdio.h>
int add(int a, int b) {
  printf("cat_custom.c: add(%d, %d) = %d\n", a, b, a + b);
  return a + b;
}

/*
将文件编译成目标文件并打包为静态库
gcc -c cat_custom.c -o cat_custom.o
ar rcs libcat_custom.a cat_custom.o
*/
