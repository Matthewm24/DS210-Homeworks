use csv::ReaderBuilder;
use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use serde::Deserialize;
use std::{error::Error, fs::File};

struct NeuralNetwork {
    learning_rate: f32,
    weights_input_to_layer1: Array2<f32>,
    weights_layer1_to_layer2: Array2<f32>,
    weights_layer2_to_output: Array2<f32>,
}

#[derive(Debug, Deserialize)]
struct MnistData {
    label: u8,
    pixels: Vec<u8>,
}

fn load_mnist_data(file_path: &str) -> Result<(Array2<f32>, Array2<f32>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for result in rdr.deserialize() {
        let record: MnistData = result?;

        let mut one_hot = vec![0.0; 10];
        one_hot[record.label as usize] = 1.0;
        labels.push(one_hot);

        let normalized_pixels: Vec<f32> = record.pixels.iter().map(|&x| x as f32 / 255.0).collect();
        features.push(normalized_pixels);
    }

    let features_array = Array2::from_shape_vec((features.len(), 784), features.concat())?;
    let labels_array = Array2::from_shape_vec((labels.len(), 10), labels.concat())?;

    Ok((features_array, labels_array))
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: f32) -> f32 {
    x * (1.0 - x)
}

impl NeuralNetwork {
    fn new(
        input_size: usize,
        layer1_size: usize,
        layer2_size: usize,
        output_size: usize,
        learning_rate: f32,
    ) -> Self {
        let scale1 = (1.0 / (input_size + layer1_size) as f32).sqrt();
        let scale2 = (1.0 / (layer1_size + layer2_size) as f32).sqrt();
        let scale3 = (1.0 / (layer2_size + output_size) as f32).sqrt();

        let weights_input_to_layer1 =
            Array::random((input_size, layer1_size), Uniform::new(-scale1, scale1));
        let weights_layer1_to_layer2 =
            Array::random((layer1_size, layer2_size), Uniform::new(-scale2, scale2));
        let weights_layer2_to_output =
            Array::random((layer2_size, output_size), Uniform::new(-scale3, scale3));

        NeuralNetwork {
            learning_rate,
            weights_input_to_layer1,
            weights_layer1_to_layer2,
            weights_layer2_to_output,
        }
    }

    fn forward(&self, input: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
        let z1 = input.dot(&self.weights_input_to_layer1);
        let a1 = z1.mapv(sigmoid);

        let z2 = a1.dot(&self.weights_layer1_to_layer2);
        let a2 = z2.mapv(sigmoid);

        let z3 = a2.dot(&self.weights_layer2_to_output);
        let a3 = z3.mapv(sigmoid);

        (a1, a2, a3)
    }

    fn backward(
        &mut self,
        input: &Array2<f32>,
        layer1_output: &Array2<f32>,
        layer2_output: &Array2<f32>,
        final_output: &Array2<f32>,
        target: &Array2<f32>,
    ) {
        let error = target - final_output;

        let delta3 = error * final_output.mapv(sigmoid_derivative);

        let delta2 =
            delta3.dot(&self.weights_layer2_to_output.t()) * layer2_output.mapv(sigmoid_derivative);

        let delta1 =
            delta2.dot(&self.weights_layer1_to_layer2.t()) * layer1_output.mapv(sigmoid_derivative);

        self.weights_layer2_to_output += &(self.learning_rate * layer2_output.t().dot(&delta3));
        self.weights_layer1_to_layer2 += &(self.learning_rate * layer1_output.t().dot(&delta2));
        self.weights_input_to_layer1 += &(self.learning_rate * input.t().dot(&delta1));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (train_features, train_labels) = load_mnist_data("../MNIST_CSV/mnist_train.csv")?;
    let (test_features, test_labels) = load_mnist_data("../MNIST_CSV/mnist_test.csv")?;

    let mut network = NeuralNetwork::new(
        784, //input size
        512, //hiddenlayer1 size
        256, //hidden layer2 size
        10,  //output size
        0.2, //learning rate
    );

    for epoch in 0..3 {
        let mut correct = 0;
        let total = train_features.nrows();

        for i in 0..total {
            let input = train_features.row(i).into_owned().into_shape((1, 784))?;
            let target = train_labels.row(i).into_owned().into_shape((1, 10))?;

            let (layer1_output, layer2_output, final_output) = network.forward(&input);

            network.backward(
                &input,
                &layer1_output,
                &layer2_output,
                &final_output,
                &target,
            );

            let predicted = final_output
                .row(0)
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .unwrap();
            let actual = target
                .row(0)
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .unwrap();

            if predicted == actual {
                correct += 1;
            }
        }

        let epoch_accuracy = correct as f32 / total as f32 * 100.0;
        println!(
            "Epoch {} training accuracy: {:.2}%",
            epoch + 1,
            epoch_accuracy
        );
    }

    let mut correct = 0;
    for i in 0..test_features.nrows() {
        let input = test_features.row(i).into_owned().into_shape((1, 784))?;
        let target = test_labels.row(i).into_owned().into_shape((1, 10))?;

        let (_, _, final_output) = network.forward(&input);

        let predicted = final_output
            .row(0)
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        let actual = target
            .row(0)
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        if predicted == actual {
            correct += 1;
        }
    }

    let accuracy = (correct as f32) / (test_features.nrows() as f32) * 100.0;
    println!("Final test accuracy: {:.2}%", accuracy);

    Ok(())
}
