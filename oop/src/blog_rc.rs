mod post {
    pub struct Post {
        content: String
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new()
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    pub struct DraftPost {
        content: String
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content
            }
        }
    }

    pub struct PendingReviewPost {
        content: String
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blog_rc::post::Post;

    #[test]
    fn test_post_lifecycle() {
        let mut post = Post::new();

        post.add_text("Some text");
        let post = post.request_review();
        let post = post.approve();

        assert_eq!(post.content(), "Some text");
    }
}