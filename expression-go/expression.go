package expression

// #cgo LDFLAGS: -lexpression_go
// #include <stdio.h>
// #include <stdlib.h>
// #include "bindings.h"
import "C"
import "unsafe"

func Evaluate(expression string, event_json string) *string {
	cs := C.CString(expression)
	event := C.CString(event_json)

	// Evaluate the expression
	ptr := C.evaluate(cs, event)

	C.free(unsafe.Pointer(cs))
	C.free(unsafe.Pointer(event))

	if ptr != nil {
		result := C.GoString(ptr)
		C.free_evaluate(ptr)
		return &result
	} else {
		return nil
	}
}
