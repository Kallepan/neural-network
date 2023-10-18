use super::{matrix::Matrix, activation::Activation};

pub struct Network<'a> {
    layers: Vec<usize>, // list of sizes of matrices
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learning_rate: f64,
    activation: Activation<'a>,
}

impl Network<'_> {
    pub fn new<'a>(
        layers: Vec<usize>, 
        learning_rate: f64, 
        activation: Activation<'a>,
    ) -> Network<'a> {
        let mut weights = Vec::new();
        let mut biases = Vec::new();

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            learning_rate,
            activation,
        }
    }


    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Input size does not match network size");
        }

        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1 {
            current = self.weights[i]
            .multiply(&current)
            .add(&self.biases[i])
            .map(self.activation.function);

            self.data.push(current.clone());
        }

        current.transpose().data[0].to_owned()
    }

    pub fn back_propogate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layers[self.layers.len() - 1] {
            panic!("Target size does not match network size");
        }

        // get difference between outputs and targets as errors
        let parsed = Matrix::from(vec![outputs]).transpose();
        let mut errors = Matrix::from(vec![targets]).transpose().subtract(&parsed);
        // Get gradients of outputs by applying derivative of activation function
        let mut gradients = parsed.map(self.activation.derivative);

        for i in (0..self.layers.len() - 1).rev() {
            // Adjust weights and biases by multiplying gradients by errors and learning rate
            gradients = gradients
                .dot_multiply(&errors)
                .map(&|x| x * self.learning_rate);
            
            // Adjust weights and biases by adding gradients
            self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            // Adjust errors so that it can be used to adjust the previous layer.
            // Do the same for gradients
            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: usize) {
        if inputs.len() != targets.len() {
            panic!("Input and target sizes do not match");
        }

        // train for the specified number of epochs
        for i in 0..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                println!("Epoch: {}%", i * 100 / epochs);
            }

            // feed forward and back propagate for each input
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(inputs[j].clone());
                self.back_propogate(outputs, targets[j].clone());
            }
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::lib::activation;

    use super::*;

    #[test]
    fn test_new() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation = activation::SIGMOID;
        let network = Network::new(layers, learning_rate, activation);

        assert_eq!(network.layers, vec![2, 3, 1]);
        assert_eq!(network.learning_rate, 0.1);
    }

    #[test]
    fn test_feed_forward() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation = activation::SIGMOID;
        let mut network = Network::new(layers, learning_rate, activation);

        let inputs = vec![0.1, 0.2];
        let outputs = network.feed_forward(inputs);

        assert_eq!(outputs.len(), 1);
    }

    #[test]
    fn test_back_propogate() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation =  activation::SIGMOID;
        let mut network = Network::new(layers, learning_rate, activation);

        let inputs = vec![0.1, 0.2];
        let outputs = network.feed_forward(inputs);
        let targets = vec![0.3];
        network.back_propogate(outputs, targets);

        assert_eq!(network.weights[0].data.len(), 3);
        assert_eq!(network.weights[0].data[0].len(), 2);
        assert_eq!(network.weights[0].data[1].len(), 2);
        assert_eq!(network.weights[0].data[2].len(), 2);
        assert_eq!(network.biases[0].data.len(), 3);
        assert_eq!(network.biases[0].data[0].len(), 1);
        assert_eq!(network.biases[0].data[1].len(), 1);
        assert_eq!(network.biases[0].data[2].len(), 1);
    }

    #[test]
    fn test_train() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation = activation::SIGMOID;
        let mut network = Network::new(layers, learning_rate, activation);

        let inputs = vec![vec![0.1, 0.2], vec![0.3, 0.4]];
        let targets = vec![vec![0.3], vec![0.5]];
        network.train(inputs, targets, 100);

        assert_eq!(network.weights[0].data.len(), 3);
        assert_eq!(network.weights[0].data[0].len(), 2);
        assert_eq!(network.weights[0].data[1].len(), 2);
        assert_eq!(network.weights[0].data[2].len(), 2);
        assert_eq!(network.biases[0].data.len(), 3);
        assert_eq!(network.biases[0].data[0].len(), 1);
        assert_eq!(network.biases[0].data[1].len(), 1);
        assert_eq!(network.biases[0].data[2].len(), 1);
    }

    #[test]	
    #[should_panic]
    fn test_train_panic() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation = activation::SIGMOID;
        let mut network = Network::new(layers, learning_rate, activation);

        let inputs = vec![vec![0.1, 0.2], vec![0.3, 0.4]];
        let targets = vec![vec![0.3]];
        network.train(inputs, targets, 100);
    }

    #[test]
    #[should_panic]
    fn test_feed_forward_panic() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation = activation::SIGMOID;
        let mut network = Network::new(layers, learning_rate, activation);

        let inputs = vec![0.1];
        network.feed_forward(inputs);
    }

    #[test]
    #[should_panic]
    fn test_back_propogate_panic() {
        let layers = vec![2, 3, 1];
        let learning_rate = 0.1;
        let activation = activation::SIGMOID;
        let mut network = Network::new(layers, learning_rate, activation);

        let inputs = vec![0.1, 0.2];
        let outputs = network.feed_forward(inputs);
        let targets = vec![0.3, 0.4];
        network.back_propogate(outputs, targets);
    }
}