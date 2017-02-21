package option

type Option struct {
	isSome bool
	value  interface{}
}

func Some(value interface{}) Option {
	return Option{
		isSome: true,
		value:  value,
	}
}

var None = Option{
	isSome: false,
}

// Match method uses visitor pattern to ensure that patterns are provided for
// `Some` and for `None`.
func (opt Option) Match(
	onSome func(value interface{}) interface{},
	onNone func() interface{},
) interface{} {
	if opt.isSome {
		return onSome(opt.value)
	} else {
		return onNone()
	}
}
