trait Sibling {
    fn annoy(&mut self, other: &dyn Sibling);
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Sibling for Person {
    fn annoy(&mut self, other: &dyn Sibling) {
        println!("I am {} and I'm annoying you!", self.name);
    }
}

fn main() {
    let mut gabe = Person::new("gabriel".to_string());
    let nathan = Person::new("nathan".to_string());

    gabe.annoy(&nathan);
}
