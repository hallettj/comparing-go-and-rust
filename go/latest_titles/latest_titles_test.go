package latest_titles

import (
	"testing"
)

func TestGetsTitles(t *testing.T) {
	docs := []Document{
		Document{ID: 1, IsArchived: false, Title: "1st doc"},
		Document{ID: 2, IsArchived: true, Title: "2nd doc"},
		Document{ID: 3, IsArchived: false, Title: "3rd doc"},
		Document{ID: 4, IsArchived: false, Title: "4th doc"},
	}

	latest := LatestTitles(docs, 2)

	if len(latest) != 2 {
		t.Fatalf("expected %i titles, but found %i", 2, len(latest))
	}

	if latest[0] != "1st doc" {
		t.Errorf("expected first title to be %s, but was %s", "1st doc", latest[0])
	}

	if latest[1] != "3rd doc" {
		t.Errorf("expected first title to be %s, but was %s", "3rd doc", latest[1])
	}
}
