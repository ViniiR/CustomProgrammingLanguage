#ifndef STDLIB_H
#define STDLIB_H

#include <stdio.h>
#include <stdarg.h>

void println(const char *format, ...) {
    va_list args;
    va_start(args, format);

    vprintf(format, args);  

    va_end(args);
}

// int sqrt(int x)

// void println(char *str)

// char *concat(char *str1, char *str2)
//
// int compare(char *str1, char *str2)

#endif
