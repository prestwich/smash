package main

import (
	"C"
	"unsafe"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
)

const (
	maxOutputLen = 4096
	maxErrLen    = 256
)

//export CRunCeloPrecompile
func CRunCeloPrecompile(op C.char, i *C.char, iLen uint32, o *C.char, oLen *uint32, e *C.char, eLen *uint32) C.int {

	iBuff := C.GoBytes(unsafe.Pointer(i), C.int(iLen))
	oBuff := (*[maxOutputLen]byte)(unsafe.Pointer(o))
	eBuff := (*[maxErrLen]byte)(unsafe.Pointer(e))

	var res []byte
	var err error

	precompilesMap := vm.PrecompiledContractsIstanbul

	if precompile, ok := precompilesMap[common.BytesToAddress([]byte{uint8(op)})]; ok {
		res, _, err = precompile.Run(iBuff, common.Address{0}, nil, 1_000_000)

		if err != nil {
			errDescr := err.Error()
			if len(errDescr) == 0 {
				*eLen = uint32(0)
				return 1
			}
			errDescrBytes := []byte(errDescr)
			errDescrByteLen := len(errDescrBytes)
			*eLen = uint32(errDescrByteLen)
			copied := copy(eBuff[0:], errDescrBytes)
			if copied != errDescrByteLen {
				println("Invalid number of bytes copied for an error")
			}
			return 1
		}
		oBytes := res
		resLen := len(oBytes)
		*oLen = uint32(len(oBytes))
		copied := copy(oBuff[0:], oBytes)
		if copied != resLen {
			println("Invalid number of bytes copied for result")
		}
		return 0
	}
	return 1
}

func main() {}
