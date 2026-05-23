package types

type Number interface {
	uint8 | uint16 | uint32 | uint64 | int8 | int16 | int32 | int64 | uint | int
}

type Solution[D1 Number, D2 Number] interface {
	LoadInput() error
	RunPart1() D1
	RunPart2() D2
}
