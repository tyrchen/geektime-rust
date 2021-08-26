#include <stdio.h>

int add_numbers(int a, int b) {
    int result = a + b;
    return result;
}

int main() {
    char c = "42";
    int n = 42;

    // 为什么说 C 是 weakly typed
    int result = add_numbers(c, n);
    printf("%d\n", result);

    return 0;
}