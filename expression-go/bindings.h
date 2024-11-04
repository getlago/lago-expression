#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * # Safety
 * Pass in a valid strings
 */
char *evaluate(const char *input, const char *event);

/**
 * # Safety
 * Only pass in pointers to strings that have been obtained through `evaluate`
 */
void free_evaluate(char *ptr);
