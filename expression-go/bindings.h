#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Expression Expression;

/**
 * # Safety
 * Pass in a valid string
 */
Expression *parse(const char *input);

/**
 * # Safety
 * Pass in a valid string
 */
const char *evaluate(Expression *expr, const char *event);
