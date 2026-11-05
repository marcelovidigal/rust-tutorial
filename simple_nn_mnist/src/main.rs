/// mnist digit classification with candle and 
/// direct download from the hugging face hub
//use std::path::Path;
use candle_core::{DType, Device, Tensor};
use candle_nn::{linear, Linear, Module, Optimizer, VarBuilder, VarMap};
//use candle_datasets::vision::Dataset;
//use candle_datasets::vision::mnist;

const DEVICE: Device = Device::Cpu;

#[derive(Debug)]
struct SimpleNN {
	fc1: Linear,
	//ln1: LayerNorm, // added dropout, layer normalization
	fc2: Linear,
	//ln2: LayerNorm, // added dropout, layer normalization
	fc3: Linear,
	//dropout: Dropout, // added dropout, layer normalization
}

impl SimpleNN {
	/// Initializes the neural network for MNIST classification (784 inputs -> 10 classes)
	fn new(in_dim: usize, classes: usize, vb: VarBuilder) -> candle_core::Result<Self> {
		let fc1 = linear(in_dim, 128, vb.pp("fc1"))?;
		//let ln1 = layer_norm(128, LayerNormConfig::default(), vb.pp("ln1"))?; // new
		let fc2 = linear(128, 64, vb.pp("fc2"))?;
		//let ln2 = layer_norm(64, LayerNormConfig::default(), vb.pp("ln2"))?; // new
		let fc3 = linear(64, classes, vb.pp("fc3"))?;

		// dropout with a 10% to 20% drop probability
		//let dropout = Dropout::new(0.15); // new


		Ok(Self { fc1, fc2, fc3 })
		//Ok(Self { fc1, ln1, fc2, ln2, fc3, dropout })
	}
}

impl Module for SimpleNN {
	fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
		let x = self.fc1.forward(xs)?;
		//let x = self.ln1.forward(&x)?; // new
		let x = x.relu()?;
		//let x = self.dropout.forward(&x, true)?; // apply dropout during training/forward

		let x = self.fc2.forward(&x)?;
		//let x = self.ln2.forward(&x)?;
		let x = x.relu()?;
		//let x = self.dropout.forward(&x, true)?;
		
		let x = self.fc3.forward(&x)?; // no relu in the final logits layer

		Ok(x)
	}
}

/// Auxiliary function to calculate classification accuracy
fn compute_accuracy(logits: &Tensor, targets: &Tensor) -> candle_core::Result<f32> {
	let preds = logits.argmax(1)?;
	let preds_vec = preds.to_vec1::<u32>()?;
	let targets_vec = targets.to_vec1::<u32>()?;

	let mut correct = 0;
	let total = preds_vec.len();

	for (p, t) in preds_vec.iter().zip(targets_vec.iter()) {
		if p == t {
			correct += 1;
		}
	}

	Ok(correct as f32 / total as f32)
}

fn train_model(
	model: &SimpleNN, 
	x_train: &Tensor, 
	y_train: &Tensor, 
	optimizer: &mut candle_nn::AdamW, 
	epochs: usize
) -> anyhow::Result<()> {
	// track current learning rate locally
	let mut current_lr = 3e-3;

	for epoch in 0..epochs {
		// learning rate decay scheduler
		// example, cut the learning rate in half every 15,000 epochs
		if epoch > 0 && epoch % 100 == 0 {
			current_lr *= 0.5;

			// fetch current optimizer params, update lr, and apply them back
			let mut params = optimizer.params().clone();
			params.lr = current_lr;
			optimizer.set_params(params);

			println!("\n[scheduler] epoch {}: learning rate decayed to {:.6}", epoch, current_lr);
		}

		let logits = model.forward(x_train)?;
		let loss = candle_nn::loss::cross_entropy(&logits, y_train)?;

		optimizer.backward_step(&loss)?;

		if (epoch + 1) % 100 == 0 {
			let train_loss = loss.to_scalar::<f32>()?;
			let acc = compute_accuracy(&logits, y_train)?;

			println!(
				"epoch {:3} | train loss: {:.4} | accuracy: {:.2}%",
				epoch + 1, train_loss, acc * 100.0
			);
		}
	}

	Ok(())
}

fn evaluate_model(model: &SimpleNN, x_test: &Tensor, y_test: &Tensor) -> anyhow::Result<()> {
	let logits = model.forward(x_test)?;
	let loss = candle_nn::loss::cross_entropy(&logits, y_test)?;

	let test_loss = loss.to_scalar::<f32>()?;
	let acc = compute_accuracy(&logits, y_test)?;

	println!("--------------------------------------------------");
	println!("evaluation results:");
	println!("test loss (cross-entropy): {:.4}", test_loss);
	println!("test accuracy:             {:.2}%", acc * 100.0);
	println!("--------------------------------------------------");

	Ok(())
}

fn main() -> anyhow::Result<()> {
	println!("downloading/loading the mnist dataset from hugging face...");

	// downloads and loads the mnist dataset natively using the candle-datasets crate
	let mnist_data = candle_datasets::vision::mnist::load()?;

	// specify a custom local directory where the files should be saved/loaded
    //let dataset_path = Path::new("data/mnist");
    //let mnist_data: Dataset = candle_datasets::vision::mnist::load_dir(dataset_path)?;
	println!("dataset successfully loaded!");

	println!("training - shape images: {:?}", mnist_data.train_images.shape());
	println!("training - shape labels: {:?}", mnist_data.train_labels.shape());

	// tensors come pre-included in the default device, with pixels normalized/prepared by `candle-datasets`
	let x_train = mnist_data.train_images.to_device(&DEVICE)?;
	let y_train = mnist_data.train_labels.to_dtype(DType::U32)?.to_device(&DEVICE)?;

	let x_test = mnist_data.test_images.to_device(&DEVICE)?;
	let y_test = mnist_data.test_labels.to_dtype(DType::U32)?.to_device(&DEVICE)?;

	let train_d2 = x_train.dims()[1]; // 784 pixels (28x28)

	// Inicialização do modelo (784 entradas, 10 classes de saída)
	let varmap = VarMap::new();
	let vb = VarBuilder::from_varmap(&varmap, DType::F32, &DEVICE);
	let model = SimpleNN::new(train_d2, 10, vb)?;

	let optim_config = candle_nn::ParamsAdamW {
		//lr: 1e-3,
		lr: 3e-3,
		weight_decay: 1e-5,
		..Default::default()
	};
	
	let mut optimizer = candle_nn::AdamW::new(varmap.all_vars(), optim_config)?;

	println!("starting training...");
	train_model(&model, &x_train, &y_train, &mut optimizer, 1000)?;

	println!("evaluating the model...");
	evaluate_model(&model, &x_test, &y_test)?;

	// save the weights of the trained model
	println!("saving the model weights...");
	varmap.save("mnist_nn_v1.safetensors")?;
	println!("model successfully saved to 'mnist_nn_v1.safetensors'");

	// testing inferences on samples after saving
	println!("\nrunning test inferences on real samples");

	let num_samples_to_test = 5;

	// extracts the first n images and actual test labels for visual validation/printing
	let sample_images = x_test.narrow(0, 0, num_samples_to_test)?;
	let sample_labels = y_test.narrow(0, 0, num_samples_to_test)?.to_vec1::<u32>()?;

	// performs a forward pass on the selected samples
	let sample_logits = model.forward(&sample_images)?;
	let predicted_classes = sample_logits.argmax(1)?.to_vec1::<u32>()?;

	for i in 0..num_samples_to_test {
		println!(
			"sample {}: actual label = {} | model prediction = {}",
			i + 1,
			sample_labels[i],
			predicted_classes[i]
		);
	}

	Ok(())
}