// struct  => class;

struct Bird {
    name: &str,
    attack: u64,
}

// methods for Bird
impl Bird {
    fn call() {
        println!("bla bla")
    }

    fn printName(&self) {
        println!("{}", self.name)
    }
}
