#include "stdio.h"

void hello() {
    printf("Hello world!\n");
}

int main() {
    char buf[1024];
    void (* p)() = &hello;
    (*p)();
    int *p1 = (int *) p;
    p1[1] = 0xdeadbeef;
}
