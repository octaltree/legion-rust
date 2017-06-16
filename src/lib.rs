extern crate time;
extern crate crypto;
extern crate serde_json;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(PartialEq, Eq, Debug)]
pub struct Block {
    index: i32,
    prev_hash: String,
    timestamp: i64,
    data: String,
    hash: String,
}
#[derive(PartialEq, Eq, Debug)]
pub struct BlockArgs {
    body: String,
}

// TODO
//instance ToJSON BlockArgs
//instance FromJSON BlockArgs
//instance Binary Block
//instance Out Block

impl Block {
    fn calculate_hash(&self) -> String {
        let &Block {
            ref index,
            ref prev_hash,
            ref timestamp,
            ref data,
            ..
        } = self;
        let s = [index.to_string(),
                 prev_hash.to_string(),
                 timestamp.to_string(),
                 data.to_string()]
                .join("");
        {
            let mut digest = Sha256::new();
            digest.input_str(&s);
            digest.result_str()
        }
    }
    fn add_hash(self) -> Self {
        let h = self.calculate_hash();
        Block { hash: h, ..self }
    }
    fn is_next_of(&self, prev: &Self) -> bool {
        prev.index + 1 == self.index && prev.hash == self.prev_hash &&
        self.hash == self.calculate_hash()
    }
    fn create_next(&self, data: &str) -> Block {
        let time: i64 = time::now().to_timespec().sec;
        let &Block {
            ref index,
            ref hash,
            ..
        } = self;
        Block {
                index: index + 1,
                prev_hash: hash.to_string(),
                timestamp: time,
                data: data.to_string(),
                hash: String::new(),
            }
            .add_hash()
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
        assert_eq!(is_valid_chain(&vec![]), true);
        assert_eq!(is_valid_chain(&vec![Block {
                                           index: 0,
                                           prev_hash: String::from(""),
                                           timestamp: 0,
                                           data: String::from(""),
                                           hash: String::from(""),
                                       }]),
                   false);
        let b = initial_block();
        let n = b.create_next("asdf");
        assert_eq!(is_valid_chain(&vec![b, n]), true);
        // TODO false at length more than 2
    }
}
