extern crate legion;
extern crate getopts;
extern crate iron;

use legion::*;
use std::env;
use getopts::Options;
use iron::prelude::*;
use iron::status;

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

fn cliopts_parser() -> Options {
    let mut o = Options::new();
    o.optflag("h", "help", "print usage");
    o.optopt("s", "http-port", "http server's port", "8000");
    o.optopt("p", "p2p-port", "peer-to-peer connection port", "4662");
    o.optopt("n", "seed-node", "peer-to-peer seed node", "todo");
    o
}
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn serve_http(port: String) {
    Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, "Hello world!"))))
        .http(format!("localhost:{}", port))
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.iter().next().unwrap();
    let opts = cliopts_parser().parse(&args[1..]).unwrap(); // TODO err message
    print_usage(program, cliopts_parser());
    serve_http(opts.opt_str("s").unwrap_or("8000".to_owned()))
}
