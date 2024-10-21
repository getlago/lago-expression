package main

// #cgo LDFLAGS: -L../target/debug -lexpression_go
// #include <stdio.h>
// #include <stdlib.h>
// #include "bindings.h"
import "C"

func main() {
	cs := C.CString("1+2")
	event := C.CString("{\"code\":\"123\",\"timestamp\":2,\"properties\":{\"a\": \"123\"}}")
	expr := C.parse(cs)

	result := C.GoString(C.evaluate(expr, event))
	println(result)

}
