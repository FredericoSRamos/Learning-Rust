extern crate modules;

fn main() {
    modules::client::connect();
    modules::network::connect();
    modules::network::client::connect();
}