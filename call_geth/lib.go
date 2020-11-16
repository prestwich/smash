package main

import (
	"fmt"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
)

const (
	maxOutputLen = 4096
	maxErrLen    = 256
)

// //export CRunGethPrecompile
// func CRunGethPrecompile(op C.char, i *C.char, iLen uint32, o *C.char, oLen *uint32, e *C.char, eLen *uint32) C.int {

// 	iBuff := C.GoBytes(unsafe.Pointer(i), C.int(iLen))
// 	oBuff := (*[maxOutputLen]byte)(unsafe.Pointer(o))
// 	eBuff := (*[maxErrLen]byte)(unsafe.Pointer(e))

// 	var res []byte
// 	var err error

// 	precompilesMap := vm.PrecompiledContractsIstanbul

// 	if precompile, ok := precompilesMap[common.BytesToAddress([]byte{uint8(op)})]; ok {
// 		res, err = precompile.Run(iBuff)

// 		if err != nil {
// 			errDescr := err.Error()
// 			if len(errDescr) == 0 {
// 				*eLen = uint32(0)
// 				return 1
// 			}
// 			errDescrBytes := []byte(errDescr)
// 			errDescrByteLen := len(errDescrBytes)
// 			*eLen = uint32(errDescrByteLen)
// 			copied := copy(eBuff[0:], errDescrBytes)
// 			if copied != errDescrByteLen {
// 				println("Invalid number of bytes copied for an error")
// 			}
// 			return 1
// 		}
// 		oBytes := res
// 		resLen := len(oBytes)
// 		*oLen = uint32(len(oBytes))
// 		copied := copy(oBuff[0:], oBytes)
// 		if copied != resLen {
// 			println("Invalid number of bytes copied for result")
// 		}
// 		return 0
// 	}
// 	return 1
// }

func readStdin(size uint16) ([]byte, error) {
	buf := make([]byte, size)
	var offset int

	for offset < int(size) {
		size, err := os.Stdin.Read(buf[offset:])
		if err != nil {
			return nil, err
		}
		if size != 0 {
			offset += size
		}
	}
	return buf, nil
}

func inputFromStdin() (uint8, []byte, error) {
	prefix, err := readStdin(3)
	if err != nil {
		return 0, nil, err
	}

	var bodySize uint16
	bodySize |= uint16(prefix[0]) << 8
	bodySize |= uint16(prefix[1])

	address := prefix[2]

	body, err := readStdin(bodySize)
	if err != nil {
		return 0, nil, err
	}
	return address, body, nil
}

func retToStdOut(buf []byte, isErr bool) error {
	l := len(buf)
	_, err := os.Stdout.Write([]byte{uint8(l >> 8), uint8(l & 0xff)})
	if err != nil {
		return err
	}

	var code uint8
	if isErr {
		code = 1
	}
	os.Stdout.Write([]byte{code})

	_, err = os.Stdout.Write(buf)
	if err != nil {
		return err
	}
	return nil
}

func main() {
	fi, _ := os.Stdin.Stat()
	if (fi.Mode() & os.ModeCharDevice) != 0 {
		fmt.Println("Piped only")
		return
	}

	precompilesMap := vm.PrecompiledContractsIstanbul

	for {
		address, body, err := inputFromStdin()
		if err != nil {
			return
		}

		var outBody []byte
		var isErr bool
		if precompile, ok := precompilesMap[common.BytesToAddress([]byte{uint8(address)})]; ok {
			buf, err := precompile.Run(body)
			if err != nil {
				outBody = []byte(err.Error())
			} else {
				outBody = buf
			}
		} else {
			outBody = []byte("Precompile does not exist")
		}
		retToStdOut(outBody, isErr)
	}
}
