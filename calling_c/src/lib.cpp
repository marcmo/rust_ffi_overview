#include <stdint.h>
#include <stddef.h>
#include <stdio.h>

extern "C"
int triple_input(int input) {
  return input * 3;
}

extern "C"
void fill_array(uint8_t* ptr, size_t len, size_t cap) {
    printf("fill array of size %d\n", len);
    for (uint16_t i = 0; i < len; i++) {
        ptr[i] = i;
    }
    // vec_free(ptr, len, cap);
}