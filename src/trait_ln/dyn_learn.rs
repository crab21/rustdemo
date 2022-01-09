struct sheep {}

struct cow {}

trait animal {
    fn noise(&self) -> &'static str;
}

impl animal for sheep {
    fn noise(&self) -> &'static str {
        "baaaaah"
    }
}

impl animal for cow {
    fn noise(&self) -> &'static str {
        "moooo"
    }
}
fn random_animal(random_number: u32) -> Box<dyn animal> {
    match random_number {
        0 => Box::new(sheep {}),
        _ => Box::new(cow {}),
    }
}
#[test]
fn test_dyn() {
    let a = random_animal(1);
    assert_eq!(a.noise(), "moooo");
}
