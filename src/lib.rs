extern crate time;
extern crate crypto;
#[macro_use]
extern crate serde_derive;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Block {
    index: i32,
    prev_hash: String,
    timestamp: i64,
    data: String,
    hash: String,
}
type Chain = Vec<Block>;
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct BlockArgs {
    body: String,
}

// TODO
//instance Binary Block
//instance Out Block

impl Block {
    fn new() -> Self {
        let b = Block {
            index: 0,
            prev_hash: String::from("0"),
            timestamp: 0,
            data: String::from("initial data"),
            hash: String::new(),
        };
        b.add_hash()
    }
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

pub fn is_valid_chain(chain: &Chain) -> bool {
    if chain.len() == 0 {
        true
    } else if chain.len() == 1 {
        chain.iter().next().unwrap() == &Block::new()
    } else {
        let mut ps = chain.iter().zip(chain.iter().skip(1));
        chain.iter().next().unwrap() == &Block::new() &&
        ps.all(|p| {
                   let (pr, ne): (&Block, &Block) = p;
                   ne.is_next_of(pr)
               })
    }
}

pub fn newer_chain(current: &Chain, attempt: &Chain) -> Chain {
    if is_valid_chain(attempt) && attempt.len() > current.len() {
        attempt.to_vec()
    } else {
        current.to_vec()
    }
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
        let b = Block::new();
        let n = b.create_next("asdf");
        assert_eq!(is_valid_chain(&vec![b, n]), true);
        // TODO false at length more than 2
    }
}
