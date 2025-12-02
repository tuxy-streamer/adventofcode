#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include <stddef.h>

char *read_file(char path[]) {
  FILE *file;
  char *line = NULL;
  size_t len = 0;

  file = fopen(path, "rb");
  if (file == NULL) {
    perror("Failed to open file");
    return NULL;
  }

  getline(&line, &len, file);

  fclose(file);
  return line;
}

long long first_half(char input[]) {
  long long counter = 0;
  char *token;
  char *rest = malloc(strlen(input) + 1);
  strcpy(rest, input);
  char *rest_orig = rest;

  while ((token = strtok_r(rest, ",", &rest))) {
    long long start, end;

    sscanf(token, "%lld-%lld", &start, &end);

    for (long long id = start; id <= end; id++) {
      char id_str[20];
      sprintf(id_str, "%lld", id);

      int len = strlen(id_str);

      if (len % 2 == 1)
        continue;

      int half = len / 2;
      int is_invalid = 1;

      for (int i = 0; i < half; i++) {
        if (id_str[i] != id_str[half + i]) {
          is_invalid = 0;
          break;
        }
      }

      if (is_invalid) {
        counter += id;
      }
    }
  }

  free(rest_orig);
  return counter;
}

long long second_half(char input[]) {
  long long counter = 0;
  char *token;
  char *rest = malloc(strlen(input) + 1);
  strcpy(rest, input);
  char *rest_orig = rest;

  while ((token = strtok_r(rest, ",", &rest))) {
    long long start, end;

    sscanf(token, "%lld-%lld", &start, &end);

    for (long long id = start; id <= end; id++) {
      char id_str[20];
      sprintf(id_str, "%lld", id);

      int len = strlen(id_str);

      int is_invalid = 0;
      for (int pattern_len = 1; pattern_len <= len / 2; pattern_len++) {
        if (len % pattern_len != 0)
          continue;

        int repetitions = len / pattern_len;

        int is_valid_pattern = 1;
        for (int i = 0; i < len; i++) {
          if (id_str[i] != id_str[i % pattern_len]) {
            is_valid_pattern = 0;
            break;
          }
        }

        if (is_valid_pattern && repetitions >= 2) {
          is_invalid = 1;
          break;
        }
      }

      if (is_invalid) {
        counter += id;
      }
    }
  }

  free(rest_orig);
  return counter;
}

int main() {
  char test[] =
      "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-"
      "1698528,446443-446449,38593856-38593862,565653-565659,824824821-"
      "824824827,2121212118-2121212124";

  long long result = first_half(test);
  printf("%lld\n", result);

  if (result == 1227775554) {
    printf("Test: Pass\n");
  } else {
    printf("Test: Failed\n");
  }

  char *file_input = read_file("day2.txt");
  if (file_input != NULL) {
    long long file_result = first_half(file_input);
    printf("%lld\n", file_result);
    free(file_input);
  }

  result = second_half(test);
  printf("%lld\n", result);

  if (result == 4174379265) {
    printf("Test: Pass\n");
  } else {
    printf("Test: Failed\n");
  }

  file_input = read_file("day2.txt");
  if (file_input != NULL) {
    long long file_result = second_half(file_input);
    printf("%lld\n", file_result);
    free(file_input);
  }

  return 0;
}
