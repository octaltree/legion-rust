extern crate legion;

use legion::*;

enum MySession {
    EmptySession,
}
struct MainArgs {
    http_port: String,
    p2p_port: String,
    seed_node: Option<String>,
}
struct BlockChainState {
    state: Vec<Block>,
    // TODO
}
enum BlockUpdate {
    UpdateData(Block),
    ReplaceData(Vec<Block>),
    RequestChain,
}
// TODO
//instance B.Binary BlockUpdate

const p2p_service_name: &str = "updateservice";

fn main() {
    println!("Hello, world!");
}
