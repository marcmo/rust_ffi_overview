#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

extern uint32_t
addition(uint32_t, uint32_t);

int main(void) {
    printf("C: calling rust\n");
    uint32_t sum = addition(21, 21);
    printf("C: sum was:%" PRIu32 "\n", sum);
}
