#include <stdio.h>
#include <stdlib.h>
#include <time.h>

typedef unsigned int u32;

int main() {
  const u32 lim = 1000000;
  const u32 size = 16384;
  u32* array = (u32*)malloc(size * sizeof(u32));
  u32 tot = 0;
  u32 sum = 0;

  clock_t start = clock();

  for (u32 j = 0; j < lim; ++j) {
    for (u32 i = 0; i < size; ++i) {
      array[i] = i;
    }
  }
  //for (u32 i = 0; i < size; ++i) {
  //  sum += array[i];
  //}

  clock_t end = clock();
  double cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;

  fprintf(stderr, "");
  printf("%lf\n", cpu_time_used);

  fprintf(stderr, "done %u %u\n", array[7], tot);
}
