package map_impl

func Map(f func(x interface{}) interface{}, xs []interface{}) []interface{} {
	ys := make([]interface{}, len(xs))
	for i, x := range xs {
		ys[i] = f(x)
	}
	return ys
}
