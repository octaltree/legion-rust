extern crate legion;

use legion::*;

enum MySession {
    EmptySession,
}
enum BlockUpdate {
    UpdateData(Block),
    ReplaceData(Vec<Block>),
    RequestChain,
}

fn main() {
    println!("Hello, world!");
}
