use farid::*;
use std::fmt::Display;

#[test]
fn test_numbers() {
    let mut gen: Gen<usize> = Gen::init(0);

    println!("{:?}", gen.id());

    let id = gen.id();
    println!("{}", id);

    println!("{:?}", gen.id());

    gen.free(id);

    println!("{:?}", gen.id());
    println!("{:?}", gen.id());
    println!("{:?}", gen.id());
}

#[derive(Debug, Clone)]
struct RegNo {
    year: usize,
    branch: String,
    nonce: usize,
}

impl Advance for RegNo {
    fn advance(&mut self) {
        self.nonce += 1;
    }
}

impl Display for RegNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:0>4}", self.year % 100, self.branch, self.nonce)
    }
}

#[test]
fn test_custom_types() {
    let mut id: Gen<RegNo> = Gen::init(RegNo {
        year: 2017,
        branch: String::from("BCE"),
        nonce: 0,
    });

    println!("{}", id.id());
    println!("{}", id.id());
    println!("{}", id.id());
}
