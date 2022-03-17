


pub trait Summary {
    fn summarize(&self) ->String;
}


pub fn start_info() {
    let tw = super::summarize_learn::Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let s = tw.summarize();
    println!("{}", s);
}

