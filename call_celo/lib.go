package main

import (
	"fmt"
	"io"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
)

type command struct {
	body    []byte
	address uint8
}

type response struct {
	isErr bool
	gas   uint64
	body  []byte
}

func readSafe(reader io.Reader, desired uint) ([]byte, error) {
	buf := make([]byte, desired)
	var offset int
	for offset < int(desired) {
		size, err := reader.Read(buf[offset:])
		if err != nil {
			return nil, err
		}
		if size != 0 {
			offset += size
		}
	}
	return buf, nil
}

func (c *command) ReadFrom(reader io.Reader) (int64, error) {
	prefix, err := readSafe(reader, 3)

	if err != nil {
		return 0, err
	}

	var bodySize uint16
	bodySize |= uint16(prefix[0]) << 8
	bodySize |= uint16(prefix[1])

	body, err := readSafe(reader, uint(bodySize))

	if err != nil {
		return 0, err
	}

	c.address = prefix[2]
	c.body = body

	return int64(bodySize) + 3, nil
}

func (c *command) Run() *response {
	precompilesMap := vm.PrecompiledContractsIstanbul

	var res response

	if precompile, ok := precompilesMap[common.BytesToAddress([]byte{uint8(c.address)})]; ok {

		buf, gas, err := precompile.Run(c.body, common.Address{0}, nil, 10_000_000)
		if err != nil {
			res.isErr = true
			res.body = []byte(err.Error())
			res.gas = gas
		} else {
			res.isErr = false
			res.body = buf
			res.gas = gas
		}
	} else {
		res.isErr = true
		res.body = []byte(fmt.Errorf("Precompile %d does not exist", c.address).Error())
	}
	return &res
}

func (r *response) WriteTo(writer io.Writer) (int64, error) {
	l := len(r.body)
	_, err := writer.Write([]byte{uint8(l >> 8), uint8(l & 0xff)})

	if err != nil {
		return 0, err
	}

	var code uint8
	if r.isErr {
		code = 1
	}
	_, err = writer.Write([]byte{code})
	if err != nil {
		return 0, err
	}

	_, err = writer.Write(r.body)
	if err != nil {
		return 0, err
	}
	return 3 + int64(len(r.body)), nil
}

func main() {
	fi, _ := os.Stdin.Stat()
	if (fi.Mode() & os.ModeCharDevice) != 0 {
		fmt.Println("Piped only")
		return
	}

	for {
		var c command
		_, err := c.ReadFrom(os.Stdin)
		if err != nil {
			return
		}

		res := c.Run()
		res.WriteTo(os.Stdout)
	}
}
