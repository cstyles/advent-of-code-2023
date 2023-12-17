#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <unordered_map>

void debug(char* grid);
void tilt_north(char* grid);
void tilt_south(char* grid);
void tilt_east(char* grid);
void tilt_west(char* grid);
int load(char* grid);

/* const size_t DIMENSION = 10; */
/* const char* FILE_NAME = "../test_input.txt"; */
const size_t DIMENSION = 100;
const char* FILE_NAME = "../input.txt";

#define lookup(grid, y, x) (grid + y * DIMENSION + x)

int main() {
  FILE* fp = fopen(FILE_NAME, "r");
  if (fp == NULL) {
    printf("'%s' not found.\n", FILE_NAME);
    exit(1);
  }

  char* grid = (char*) malloc(sizeof(char) * DIMENSION * DIMENSION);
  char* grid_ptr = grid;

  char* line = NULL;
  size_t len = 0;
  ssize_t chars_read = 0;

  while ((chars_read = getline(&line, &len, fp)) != -1) {
    strncpy(grid_ptr, line, DIMENSION);
    grid_ptr += DIMENSION;
  }

  free(line);

  char* part1_grid = (char*) malloc(sizeof(char) * DIMENSION * DIMENSION);
  memcpy(part1_grid, grid, DIMENSION * DIMENSION);

  tilt_north(part1_grid);

  printf("part1 = %d\n", load(part1_grid));
  free(part1_grid);

  std::unordered_map<std::string, int> seen;
  seen[grid] = 0;

  int first_cycle_of_loop;
  int current_cycle = 0;

  while (true) {
    current_cycle += 1;

    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);

    if (seen.count(grid) > 0) {
      first_cycle_of_loop = seen[grid];
      break;
    } else {
      seen[grid] = current_cycle;
    }
  }

  int loop_length = current_cycle - first_cycle_of_loop;
  int how_many_loops = (1000000000 - current_cycle) / loop_length;
  int leftover_cycles = 1000000000 - current_cycle - loop_length * how_many_loops;

  for (int x = 0; x < leftover_cycles; x++) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
  }

  printf("part2 = %d\n", load(grid));
  free(grid);
}

void debug(char* grid) {
    for (int y = 0; y < DIMENSION; y++) {
      for (int x = 0; x < DIMENSION; x++) {
        printf("%c", grid[y * DIMENSION + x]);
      }

      printf("\n");
    }
}

void tilt_north(char* grid) {
  for (int y = 0; y < DIMENSION; y++) {
    for (int x = 0; x < DIMENSION; x++) {
      char* tile = lookup(grid, y, x);

      if (*tile != 'O') {
        continue;
      }

      int final_y = y;
      while (final_y > 0 && *lookup(grid, (final_y - 1), x) == '.') {
        final_y -= 1;
      }

      *lookup(grid, y, x) = '.';
      *lookup(grid, final_y, x) = 'O';
    }
  }
}

void tilt_south(char* grid) {
  for (int y = DIMENSION - 1; y >= 0; y--) {
    for (int x = 0; x < DIMENSION; x++) {
      char* tile = lookup(grid, y, x);

      if (*tile != 'O') {
        continue;
      }

      int final_y = y;
      while (final_y < DIMENSION - 1 && *lookup(grid, (final_y + 1), x) == '.') {
        final_y += 1;
      }

      *lookup(grid, y, x) = '.';
      *lookup(grid, final_y, x) = 'O';
    }
  }
}

void tilt_east(char* grid) {
  for (int y = 0; y < DIMENSION; y++) {
    for (int x = DIMENSION - 1; x >= 0; x--) {
      char* tile = lookup(grid, y, x);

      if (*tile != 'O') {
        continue;
      }

      int final_x = x;
      while (final_x < DIMENSION - 1 && *lookup(grid, y, final_x + 1) == '.') {
        final_x += 1;
      }

      *lookup(grid, y, x) = '.';
      *lookup(grid, y, final_x) = 'O';
    }
  }
}

void tilt_west(char* grid) {
  for (int y = 0; y < DIMENSION; y++) {
    for (int x = 0; x < DIMENSION; x++) {
      char* tile = lookup(grid, y, x);

      if (*tile != 'O') {
        continue;
      }

      int final_x = x;
      while (final_x > 0 && *lookup(grid, y, final_x - 1) == '.') {
        final_x -= 1;
      }

      *lookup(grid, y, x) = '.';
      *lookup(grid, y, final_x) = 'O';
    }
  }
}

int load(char* grid) {
  int load = 0;

  for (int y = 0; y < DIMENSION; y ++) {
    int flipped_y = DIMENSION - y - 1;

    for (int x = 0; x < DIMENSION; x++) {
      if (*lookup(grid, flipped_y, x) == 'O') {
        load += (y + 1);
      }
    }
  }

  return load;
}
