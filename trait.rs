pub struct NewsArticle {
  pub author: String,
  pub headline: String,
  pub content: String
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {}", self.headline, self.author);
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}, by {}", self.content, self.username);
  }
}

pub trait Summary{
  fn summarize(&self) -> String;
}




