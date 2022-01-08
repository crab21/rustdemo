use std::string;

trait enum_trait_v4 {
    fn enum_trait_method(&self);
}

struct v4 {
    username: String,
    password: String,
}

impl enum_trait_v4 for v4 {
    fn enum_trait_method(&self) {
        println!("{}", self.username);
        println!("{}", self.password);
    }
}

trait enum_trait_v6 {
    fn enum_trait_method(&self);
}

struct v6 {
    username: String,
    password: String,
}

impl enum_trait_v6 for v6 {
    fn enum_trait_method(&self) {
        println!("{}", self.username);
        println!("{}", self.password);
    }
}
enum GoEnum {
    ve(v4),
    v6(v6),
}

fn enum_learn() {}
