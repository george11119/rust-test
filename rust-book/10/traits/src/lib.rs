pub mod aggregator {
    use std::fmt::format;

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // to use own defined summarize method
    // impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }

    // to use default summarize method
    // impl Summary for NewsArticle {}

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("{}", self.username)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
}
