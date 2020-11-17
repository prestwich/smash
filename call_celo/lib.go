package main

import (
	"fmt"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
)

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
			buf, _, err := precompile.Run(body, common.Address{0}, nil, 1_000_000)

			if err != nil {
				outBody = []byte(err.Error())
			} else {
				outBody = buf
			}
		} else {
			outBody = []byte("Precompile does not exist")
			isErr = true
		}
		retToStdOut(outBody, isErr)
	}
}
