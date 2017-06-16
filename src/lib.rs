#[derive(PartialEq, Eq, Debug)]
pub struct Block {
    index: i32,
    prev_hash: String,
    timestamp: i32,
    data: String,
    hash: String,
}
#[derive(PartialEq, Eq, Debug)]
pub struct BlockArgs {
    body: String,
}

impl Block {
    fn calculate_hash(&self) -> String {
        // TODO
        [self.index.to_string()].join("")
    }
    fn add_hash(self) -> Self {
        let h = self.calculate_hash();
        Block { hash: h, ..self }
    }
    fn is_next_of(&self, prev: &Self) -> bool {
        prev.index + 1 == self.index && prev.hash == self.prev_hash &&
        self.hash == self.calculate_hash()
    }
}

pub fn is_valid_chain(chain: &Vec<Block>) -> bool {
    if chain.len() == 0 {
        true
    } else if chain.len() == 1 {
        chain.iter().next().unwrap() == &initial_block()
    } else {
        let mut ps = chain.iter().zip(chain.iter().skip(1));
        chain.iter().next().unwrap() == &initial_block() &&
        ps.all(|p| {
                   let (pr, ne): (&Block, &Block) = p;
                   ne.is_next_of(pr)
               })
    }
}

pub fn initial_block() -> Block {
    let b = Block {
        index: 0,
        prev_hash: String::from("0"),
        timestamp: 0,
        data: String::from("initial data"),
        hash: String::new(),
    };
    b.add_hash()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_is_valid_chain() {
        // TODO
        assert_eq!(is_valid_chain(&vec![]), true);
        assert_eq!(is_valid_chain(&vec![Block {
                                           index: 0,
                                           prev_hash: String::from(""),
                                           timestamp: 0,
                                           data: String::from(""),
                                           hash: String::from(""),
                                       }]),
                   false);
        assert_eq!(is_valid_chain(&vec![initial_block()]), true);
    }
}
