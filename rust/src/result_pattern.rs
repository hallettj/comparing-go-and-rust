use std::io;

#[derive(Debug)]
pub struct Post {
    id: String,
    author_id: String,
}

#[derive(Debug)]
pub struct User {
    id: String,
    posts: Vec<String>,
}

fn fetch_post(post_id: &str) -> Result<Post, io::Error> {
    Ok(Post { id: post_id.to_string(), author_id: "3".to_string(), })
}

fn fetch_posts(post_ids: &[String]) -> Result<Vec<Post>, io::Error> {
    post_ids.iter()
        .map(|id| Ok(Post { id: id.to_string(), author_id: "3".to_string() }))
        .collect()
}

fn fetch_user(user_id: &str) -> Result<User, io::Error> {
    Ok(User { id: user_id.to_string(), posts: vec!["4".to_string()] })
}

pub fn fetch_all_by_same_author_w_pattern_matching(post_id: &str) -> Result<Vec<Post>, io::Error> {
    let post = match fetch_post(post_id) {
        Ok(p)    => p,
        Err(err) => return Err(err),
    };

    let author = match fetch_user(&post.author_id) {
        Ok(a)    => a,
        Err(err) => return Err(err),
    };

    fetch_posts(&author.posts)
}

pub fn fetch_all_by_same_author_w_try_macro(post_id: &str) -> Result<Vec<Post>, io::Error> {
    let post   = try!(fetch_post(post_id));
    let author = try!(fetch_user(&post.author_id));
    fetch_posts(&author.posts)
}

pub fn fetch_all_by_same_author_w_combinators(post_id: &str) -> Result<Vec<Post>, io::Error> {
    let post   = fetch_post(post_id);
    let author = post.and_then(|p| fetch_user(&p.author_id));
    author.and_then(|a| fetch_posts(&a.posts))
}

pub fn fetch_all_by_same_author_w_error_context(post_id: &str) -> Result<Vec<Post>, io::Error> {
    let post = fetch_post(post_id)
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e));
    let author = post.and_then(|p| fetch_user(&p.author_id));
    author.and_then(|a| fetch_posts(&a.posts))
}

#[cfg(test)]
mod tests {
    use super::*; // import the implementations from above
    use std::io;

    fn fetches_posts(result: Result<Vec<Post>, io::Error>) {
        let posts = result.expect("fetch was successful");
        assert_eq!("4", posts[0].id);
    }

    #[test]
    fn fetches_with_pattern_matching() {
        fetches_posts(fetch_all_by_same_author_w_pattern_matching("3"));
    }

    #[test]
    fn fetches_with_try_macro() {
        fetches_posts(fetch_all_by_same_author_w_try_macro("3"));
    }

    #[test]
    fn fetches_with_combinators() {
        fetches_posts(fetch_all_by_same_author_w_combinators("3"));
    }

    #[test]
    fn fetches_with_error_context() {
        fetches_posts(fetch_all_by_same_author_w_error_context("3"));
    }
}
