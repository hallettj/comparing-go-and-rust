package option

import (
	"testing"
)

func checkedDivision(dividend, divisor int) Option {
	if divisor == 0 {
		return None
	} else {
		return Some(dividend / divisor)
	}
}

func TestDivideByNonzero(t *testing.T) {
	result := checkedDivision(12, 4)
	num := result.Match(
		/* Some */ func(value interface{}) interface{} { return value },
		/* None */ func() interface{} { return 0 },
	).(int)
	if num != 3 {
		t.Fatalf("expected %i, but got %i", 3, num)
	}
}

func TestDivideByZero(t *testing.T) {
	result := checkedDivision(12, 0)
	isNone := result.Match(
		/* Some */ func(value interface{}) interface{} { return false },
		/* None */ func() interface{} { return true },
	).(bool)
	if !isNone {
		t.Fatalf("expected `None` result")
	}
}
