#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char* FILE_NAME = "../test_input.txt";
/* const char* FILE_NAME = "../input.txt"; */

u_int8_t hash(char* string) {
  char c;
  uint16_t hash = 0;

  while ((c = *string++)) {
    hash += c;
    hash *= 17;
    hash %= 256;
  }

  return hash;
}

int main() {
  FILE* fp = fopen(FILE_NAME, "r");
  if (fp == NULL) {
    printf("'%s' not found.\n", FILE_NAME);
    exit(1);
  }

  // Figure out how long the file is.
  fseek(fp, 0, SEEK_END);
  long input_size = ftell(fp);
  fseek(fp, 0, SEEK_SET);

  // Read the whole file into memory.
  char* input = (char*) malloc(input_size + 1);
  fread(input, input_size, 1, fp);
  assert(!fclose(fp));

  // Set the last character (a newline) to NUL.
  input[input_size - 1] = '\0';

  char* prev = input;
  char* token;
  int part1 = 0;

  while ((token = strsep(&prev, ","))) {
    /* printf("token: %s\n", token); */
    part1 += hash(token);
  }

  printf("part1 = %d\n", part1);
}
