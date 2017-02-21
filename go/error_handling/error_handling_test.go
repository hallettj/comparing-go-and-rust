package error_handling

import (
	"testing"
)

func TestFetchAllBySameAuthor(t *testing.T) {
	posts, err := fetchAllBySameAuthor("3")
	if err != nil {
		t.Fatalf("unexpected error result")
	}
	if len(posts) != 3 {
		t.Fatalf("expected %i posts, but got %i", 3, len(posts))
	}
	if posts[0].ID != "4" {
		t.Fatalf("expected first post ID to be %s, but it was %s", "4", posts[0].ID)
	}
}
