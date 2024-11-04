package main

// #cgo LDFLAGS: -L../target/release -lexpression_go
// #include <stdio.h>
// #include <stdlib.h>
// #include "bindings.h"
import "C"
import "unsafe"

func main() {
	cs := C.CString("concat(event.properties.a, 'test')")
	event := C.CString("{\"code\":\"13\",\"timestamp\":2,\"properties\":{\"a\": 123.12}}")

	ptr := C.evaluate(cs, event)
	if ptr != nil {

		result := C.GoString(ptr)
		println(result)

		C.free_evaluate(ptr)
	}

	C.free(unsafe.Pointer(cs))
	C.free(unsafe.Pointer(event))

}
