package error_handling

type Post struct {
	ID       string
	AuthorID string
}

type User struct {
	ID    string
	Posts []string
}

func fetchPost(postId string) (*Post, error) {
	return &Post{
		ID:       postId,
		AuthorID: "3",
	}, nil
}

func fetchPosts(postIds []string) ([]Post, error) {
	posts := make([]Post, len(postIds))
	for i, id := range postIds {
		posts[i] = Post{ID: id, AuthorID: "3"}
	}
	return posts, nil
}

func fetchUser(userId string) (*User, error) {
	return &User{
		ID:    userId,
		Posts: []string{"4", "5", "6"},
	}, nil
}

func fetchAllBySameAuthor(postID string) ([]Post, error) {
	post, err := fetchPost(postID)
	if err != nil {
		return nil, err
	}

	author, err := fetchUser(post.AuthorID)
	if err != nil {
		return nil, err
	}

	return fetchPosts(author.Posts)
}
