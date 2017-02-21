package map_impl

import (
	"testing"
)

var xs = []int{1, 2, 3, 4}

func TestMap(t *testing.T) {
	// Process `xs` so that the data can be passed to `Map`
	input := make([]interface{}, len(xs))
	for i, x := range xs {
		input[i] = x
	}

	output := Map(func(value interface{}) interface{} {
		x := value.(int)
		return x * x
	}, input)

	result := make([]int, len(output))
	for i, x := range output {
		result[i] = x.(int)
	}

	if len(result) != 4 {
		t.Fatalf("expected %i elements in returned slice, but found %i", 4, len(result))
	}

	if result[1] != 4 {
		t.Fatalf("expected 2 * 2 to equal 4, but found %i", result[1])
	}
}
