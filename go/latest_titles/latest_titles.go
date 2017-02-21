package latest_titles

type Document struct {
	ID         int64
	IsArchived bool
	Title      string
}

// Take the titles of the first `count` docs that are not archived
func LatestTitles(docs []Document, count int) []string {
	var latest []string
	for _, doc := range docs {
		if len(latest) >= count {
			return latest
		}
		if !doc.IsArchived {
			latest = append(latest, doc.Title)
		}
	}
	return latest
}
