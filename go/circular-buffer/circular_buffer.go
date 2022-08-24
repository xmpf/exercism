package circular

import (
	"errors"
)

type Buffer struct {
	data 	[]byte
	size	int
	start 	int
	end		int
}

func NewBuffer(size int) *Buffer {
	return &Buffer{data: make([]byte, 1 + size), size: 1 + size, start: 0, end: 0}
}

// helper function
func (b *Buffer) next() (int, int) {
	return (1 + b.start) % b.size, (1 + b.end) % b.size
}

// helper function
func (b *Buffer) isFull() bool {
	_, end := b.next()
	return end == b.start
}

func (b *Buffer) isEmpty() bool {
	return b.start == b.end
}

func (b *Buffer) ReadByte() (byte, error) {
	if b.isEmpty() {
		return 0, errors.New("Buffer is empty :(")
	}
	
	datum := b.data[b.start]
	b.start, _ = b.next()
	
	return datum, nil
}

func (b *Buffer) WriteByte(c byte) error {
	if b.isFull() {
		return errors.New("Buffer is full :/")
	}
	
	b.data[b.end] = c
	_, b.end = b.next()

	return nil
}

func (b *Buffer) Overwrite(c byte) error {
	if b.isFull() {
		b.start, _ = b.next()
	}
	
	b.data[b.end] = c
	_, b.end = b.next()
	
	return nil
}

func (b *Buffer) Reset() {
	b.start, b.end = 0, 0
}
