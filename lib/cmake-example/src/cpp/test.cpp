#include "test.h"
#include <stdio.h>

int test1(const char* name)
{
    fprintf(stdout, "my name is %s\n", name);
    return 0;
}

int adder(int lhs, int rhs)
{
    return lhs + rhs;
}