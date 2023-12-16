#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* const char* FILE_NAME = "../test_input.txt"; */
const char* FILE_NAME = "../input.txt";

typedef struct BoxItem BoxItem;

struct BoxItem {
  BoxItem* next;
  char* label;
  int focal_length;
};

void remove_box(BoxItem* list, char* label);
void insert_or_replace(BoxItem* list, BoxItem* item);

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

  BoxItem* boxes[256];
  for (int i = 0; i < 256; i++) {
    BoxItem* item = (BoxItem*) malloc(sizeof(BoxItem));
    item->next = NULL;
    item->label = NULL;
    item->focal_length = 0;

    boxes[i] = item;
  }

  while ((token = strsep(&prev, ","))) {
    part1 += hash(token);

    int length = strlen(token);

    if (*(token + length - 1) == '-') {
      // Trim the '-' so token == label.
      *(token + length - 1) = '\0';

      // Find the box where this item should go.
      uint8_t box_number = hash(token);

      BoxItem* a_box = boxes[box_number];
      remove_box(a_box, token);
    } else {
      // Trim the '=' so token == label.
      *(token + length - 2) = '\0';

      int focal_length = *(token + length - 1) - '0';

      // Find the box where this item should go.
      uint8_t box_number = hash(token);
      BoxItem* a_box = boxes[box_number];

      // Create the BoxItem to put in `boxes`.
      BoxItem* item = (BoxItem*) malloc(sizeof(BoxItem));
      item->next = NULL;
      item->label = token;
      item->focal_length = focal_length;

      insert_or_replace(a_box, item);
    }
  }

  printf("part1 = %d\n", part1);

  // Calculate focusing power:
  int part2 = 0;
  for (int box_number = 0; box_number < 256; box_number++) {
    BoxItem* box = boxes[box_number];

    int slot = 1;
    while (box && box->focal_length) {
      int diff = (box_number + 1) * slot * box->focal_length;
      part2 += diff;
      slot += 1;
      box = box->next;
    }
  }

  printf("part2 = %d\n", part2);

  free(input);
  for (int i = 0; i < 256; i++) {
    free(boxes[i]);
  }
}

// Iterate over a list until a `BoxItem` with a specific `label` is found.
// Then remove it from the linked list.
void remove_box(BoxItem* list, char* label) {
  BoxItem* prev = NULL;

  // Return early if the box is empty.
  if (list->label == NULL) {
    return;
  }

  for (;;) {
    if (!strcmp(list->label, label)) {
      // We found a matching label in the list.

      if (prev) {
        // If it's not the first item, just splice it out.
        prev->next = list->next;
        free(list);
      } else {
        if (list->next) {
          // If there are more items, splice out the head of the list.
          BoxItem* next = list->next;
          list->next = next->next;
          list->label = next->label;
          list->focal_length = next->focal_length;
          free(next);
        } else {
          // There's only one item so just clear it.
          list->next = NULL;
          list->label = NULL;
          list->focal_length = 0;
        }
      }

      return;
    }

    if (list->next) {
      // If there are more items, keep iterating over the list.
      prev = list;
      list = list->next;
    } else {
      // Otherwise the item isn't in the list.
      return;
    }
  }
}

// Update the focal length for an item in a box. If an item with that label
// doesn't already exist, insert it at the end of the box.
void insert_or_replace(BoxItem* list, BoxItem* item) {
  if (list->label == NULL) {
    // If the box is empty, just insert it at the start/end.
    *list = *item;
    return;
  }

  for (;;) {
    if (!strcmp(list->label, item->label)) {
      // Found a matching label so replace the focal length.
      list->focal_length = item->focal_length;
      return;
    }

    if (list->next) {
      // If there are more items, keep iterating over the list.
      list = list->next;
    } else {
      // Otherwise we're at the end so insert the item.
      list->next = item;
      return;
    }
  }
}
