use super::trait_learn::Summary;

pub struct NewArticles {
    pub head_line: String,
    pub content: String,
}

impl Summary for NewArticles {
    fn summarize(&self) -> String {
        println!("{}", self.head_line);
        String::from("ok")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
