#include <stdint.h>
#include <stdio.h>

extern "C"
void fill_array(uint8_t* ptr, size_t len, size_t cap) {
    printf("C: fill array of size %ld (cap: %ld)\n", len, cap);
    for (uint16_t i = 0; i < len; i++) {
        ptr[i] = i;
    }
    // vec_free(ptr, len, cap);
}