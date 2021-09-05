#include <stdio.h>

int * lcal_ref()
{
    int a = 42;
    return &a;
}

int main()
{
    int* p = lcal_ref();
    printf("%i\n", *p);
    *p = 256;
    printf("%i\n", *p);
}
