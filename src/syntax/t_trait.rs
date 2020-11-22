pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Summary1 {
    fn summarize1(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Summary2 {
    fn summarize2(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary1 for NewsArticle {}

impl Summary2 for NewsArticle {
    fn summarize2(&self) -> String {
        unimplemented!()
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
