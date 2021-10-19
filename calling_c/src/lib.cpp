#include <stdint.h>
#include <stdio.h>

extern "C"
int triple_input(int input) {
    printf("C: triple_input for %d\n", input);
    return input * 3;
}

