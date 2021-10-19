#include <stdint.h>
#include <stdio.h>

typedef void (*rust_callback)(uint8_t*, size_t, size_t);
rust_callback vec_free;

extern "C"
int32_t register_callback(rust_callback callback) {
  printf("C: register callback");
  vec_free = callback;
  return 1;
}

extern "C"
void fill_array(uint8_t* ptr, size_t len, size_t cap) {
    printf("C: fill array of size %ld (cap: %ld)\n", len, cap);
    for (uint16_t i = 0; i < len; i++) {
        ptr[i] = i;
    }
    printf("C: moving back array to rust\n");
    vec_free(ptr, len, cap);
}
