use lib::{neural_network::Network, activation::SIGMOID};

pub mod lib;

// Sample Case: XOR
// 0, 0 -> 0
// 0, 1 -> 1
// 1, 0 -> 1
// 1, 1 -> 0

fn main() {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];
    
    let mut network = Network::new(
        vec![2,3,1],
        0.5,
        SIGMOID
    );

    network.train(inputs, targets, 10000);

    println!("0, 0 -> {:?}", network.feed_forward(vec![0.0, 0.0]));
    println!("0, 1 -> {:?}", network.feed_forward(vec![0.0, 1.0]));
    println!("1, 0 -> {:?}", network.feed_forward(vec![1.0, 0.0]));
    println!("1, 1 -> {:?}", network.feed_forward(vec![1.0, 1.0]));
}
