/// Simple neural network with candle framework,
/// Includes Feature Scaling, Weight Decay, LR Scheduler, and Dropout.
use candle_core::{DType, Device, Tensor};
use candle_nn::{linear, Linear, Module, Optimizer, VarBuilder, VarMap};
//use candle_nn::{linear, Linear, Module, Optimizer, VarBuilder, VarMap, Dropout, layer_norm, LayerNorm, LayerNormConfig};

const DEVICE: Device = Device::Cpu;

#[derive(Debug)]
struct SimpleNN {
	fc1: Linear, // fully connected 1
	//ln1: LayerNorm, // added dropout, layer normalization
	fc2: Linear, // fully connected 2
	//ln2: LayerNorm, // added dropout, layer normalization
	fc3: Linear, // added third layer
	//dropout: Dropout, // added dropout, layer normalization
}

// creates a new `SimpleNN` model instance with two linear layers
impl SimpleNN{
	/// Initializes a new 2-layer fully connected neural network.
	///
	/// # Arguments
	/// * `in_dim` - The number of input features (channels).
	/// * `vb` - The variable builder used to allocate and initialize layer weights.
	///
	/// # Errors
	/// Returns a `candle_core::Result::Err` if layer initialization or weight loading fails.

	// this function instantiates a new model
	fn new(in_dim:usize, vb:VarBuilder)->candle_core::Result<Self>{
		//let fc1 = linear(in_dim, 64, vb.pp("fc1"))?;
		//let fc2 = linear(64, 1, vb.pp("fc2"))?;

		let fc1 = linear(in_dim, 128, vb.pp("fc1"))?; // widened first layer
		//let ln1 = layer_norm(128, LayerNormConfig::default(), vb.pp("ln1"))?; // new
		let fc2 = linear(128, 64, vb.pp("fc2"))?; // added second hidden layer
		//let ln2 = layer_norm(64, LayerNormConfig::default(), vb.pp("ln2"))?; // new
		let fc3 = linear(64, 1, vb.pp("fc3"))?; // output layer

		// dropout with a 10% to 20% drop probability
		//let dropout = Dropout::new(0.15); // new

		//Ok(Self { fc1, fc2 })
		Ok(Self { fc1, fc2, fc3 })
		//Ok(Self { fc1, ln1, fc2, ln2, fc3, dropout })
	}
}

// forward pass of our model using Module trait 
impl Module for SimpleNN {
	fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
		let x = self.fc1.forward(xs)?;
		//let x = self.ln1.forward(&x)?; // new
		let x = x.relu()?;
		//let x = self.dropout.forward(&x, true)?; // apply dropout during training/forward
		
		let x = self.fc2.forward(&x)?;
		//let x = self.ln2.forward(&x)?;
		let x = x.relu()?; // added
		//let x = self.dropout.forward(&x, true)?;
		
		// no relu on final output layer for regression
		// no activation/dropout on final regression layer
		let x = self.fc3.forward(&x)?;

		Ok(x)
	}
}

/// helper function to compute mean absolute error (mae) and 
/// threshold accuracy (fraction of predictions within 20% of the true value)
fn compute_metrics(preds: &Tensor, targets: &Tensor) -> candle_core::Result<(f32, f32)> {
	let preds_vec = preds.to_vec1::<f32>()?;
	let targets_vec = targets.to_vec1::<f32>()?;
	let n = preds_vec.len() as f32;

	let mut total_abs_error = 0.0;
	let mut within_threshold_count = 0;

	for (p, t) in preds_vec.iter().zip(targets_vec.iter()) {
		let abs_error = (p - t).abs();
		total_abs_error += abs_error;
		
		// check if prediction is within 20% tolerance of target value
		let tolerance = 0.20 * t.abs();
		
		if abs_error <= tolerance {
			within_threshold_count += 1;
		}
	}

	let mae = total_abs_error / n;
	let accuracy = within_threshold_count as f32 / n;

	Ok((mae, accuracy))
}

/*
fn train_model(model: &SimpleNN, x_train: &Tensor, y_train: &Tensor, optimizer: &mut candle_nn::AdamW, epochs: usize) -> anyhow::Result<()> {
	// track current learning rate locally
	let mut current_lr = 1e-2;
	//let mut current_lr = 2e-3;
	//let mut current_lr = 3e-3;

	for epoch in 0..epochs {
		// learning rate decay scheduler
		// example, cut the learning rate in half every 15,000 epochs
		if epoch > 0 && epoch % 5000 == 0 {
			current_lr *= 0.75;

			// fetch current optimizer params, update lr, and apply them back
			let mut params = optimizer.params().clone();
			params.lr = current_lr;
			optimizer.set_params(params);

			println!("\n[scheduler] epoch {}: learning rate decayed to {:.6}\n", epoch, current_lr);
		}

		// forward pass 
		let output = model.forward(x_train)?;
		let preds = output.squeeze(1)?;
		//let loss = candle_nn::loss::mse(&output.squeeze(1)?, y_train)?;
		let loss = candle_nn::loss::mse(&preds, y_train)?;

		// backward pass and optimization
		optimizer.backward_step(&loss)?;

		if (epoch+1) % 50 == 0 {
			//println!("({}) train Loss: {}", (epoch+1), loss.to_scalar::<f32>()?);

			// compute metrics for logging
			let train_loss = loss.to_scalar::<f32>()?;
			let (mae, acc) = compute_metrics(&preds, y_train)?;
			
			println!(
				"epoch {:3} | train loss: {:.4} | mae: {:.4} | accuracy (±20%): {:.2}%",
				epoch + 1, train_loss, mae, acc * 100.0
			);
		}
	}

	Ok(())
}
*/

// added training mini-batches 
fn train_model(
	model: &SimpleNN, 
	x_train_vec: &[f32], 
	y_train_vec: &[f32], 
	num_samples: usize, 
	in_dim: usize, 
	optimizer: &mut candle_nn::AdamW, 
	epochs: usize
) -> anyhow::Result<()> {
	let mut current_lr = 3e-3;
	//let mut current_lr = 2e-3;
	let batch_size = 64;

	for epoch in 0..epochs {
		// optional learning rate scheduler every 100 epochs, since we now have few epochs
		if epoch > 0 && epoch % 250 == 0 {
			current_lr *= 0.75;
			let mut params = optimizer.params().clone();
			params.lr = current_lr;
			optimizer.set_params(params);
			println!("\n[scheduler] epoch {}: learning rate decayed to {:.6}\n", epoch, current_lr);
		}

		// mini-batch loop per epoch
		for chunk in (0..num_samples).step_by(batch_size) {
			let end = std::cmp::min(chunk + batch_size, num_samples);
			let current_batch_size = end - chunk;

			// slice corresponding portions from the flattened X and Y arrays
			let x_batch_slice: Vec<f32> = x_train_vec[chunk * in_dim .. end * in_dim].to_vec();
			let y_batch_slice: Vec<f32> = y_train_vec[chunk..end].to_vec();

			let x_batch = Tensor::from_vec(x_batch_slice, (current_batch_size, in_dim), &DEVICE)?;
			let y_batch = Tensor::from_vec(y_batch_slice, current_batch_size, &DEVICE)?;

			let output = model.forward(&x_batch)?;
			let preds = output.squeeze(1)?;
			let loss = candle_nn::loss::mse(&preds, &y_batch)?;

			optimizer.backward_step(&loss)?;
		}

		// log evaluation every 50 epochs
		if (epoch + 1) % 50 == 0 {
			// evaluates loss/metrics using the entire batch for quick logging
			let x_train_full = Tensor::from_vec(x_train_vec.to_vec(), (num_samples, in_dim), &DEVICE)?;
			let y_train_full = Tensor::from_vec(y_train_vec.to_vec(), num_samples, &DEVICE)?;
			let output_full = model.forward(&x_train_full)?;
			let preds_full = output_full.squeeze(1)?;
			let train_loss = candle_nn::loss::mse(&preds_full, &y_train_full)?.to_scalar::<f32>()?;
			let (mae, acc) = compute_metrics(&preds_full, &y_train_full)?;

			println!(
				"epoch {:3} | train loss: {:.4} | mae: {:.4} | accuracy (±20%): {:.2}%",
				epoch + 1, train_loss, mae, acc * 100.0
			);
		}
	}

	Ok(())
}

fn evaluate_model(model: &SimpleNN, x_test: &Tensor, y_test: &Tensor, scaler: &Scaler) -> anyhow::Result<()> {
	let output = model.forward(x_test)?;
	let preds_scaled = output.squeeze(1)?.flatten_all()?.to_vec1::<f32>()?;

	// inverse transform predictions back to real values for accurate mae/metrics
	let preds_real: Vec<f32> = preds_scaled.iter().map(|&p| scaler.inverse_transform_prediction(p)).collect();
	let preds_tensor = Tensor::from_vec(preds_real, preds_scaled.len(), x_test.device())?;
	
	// compute test loss (mse) and metrics on the real scale
    let test_loss = candle_nn::loss::mse(&preds_tensor, y_test)?.to_scalar::<f32>()?;	
	let (mae, acc) = compute_metrics(&preds_tensor, &y_test)?;

	println!("--------------------------------------------------");
	println!("evaluation results (real scale):");
	println!("test loss (mse): {:.4}", test_loss);
	println!("test mae:        {:.4}", mae);
	println!("test accuracy:   {:.2}% (within 20% of target)", acc * 100.0);
	println!("--------------------------------------------------");

	Ok(())
}

// to load our training data 
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
	#[serde(rename = "X_train")]
	x_train: Vec<Vec<f32>>,
	#[serde(rename = "X_test")]
	x_test: Vec<Vec<f32>>,
	y_train: Vec<f32>,
	y_test: Vec<f32>,
}

/// Helper struct to store training means and standard deviations 
/// and reuse them for inference scaling.
#[derive(Debug)]
struct Scaler {
	means: Vec<f32>,
	std_devs: Vec<f32>,
	y_mean: f32, // target scaling
	y_std: f32, // target scaling
}

impl Scaler {
	/// Computes mean and std from training data, scales train & test in place,
	/// and returns the fitted Scaler for future inference.
	fn fit_transform(x_train: &mut Vec<Vec<f32>>, x_test: &mut Vec<Vec<f32>>, y_train: &mut Vec<f32>) -> Self {
		let num_features = x_train[0].len();
		let num_train_rows = x_train.len() as f32;

		let mut means = vec![0.0; num_features];
		let mut std_devs = vec![1.0; num_features];

		for col in 0..num_features {
			// calculate mean
			let mut sum = 0.0;
			
			for row in 0..x_train.len() {
				sum += x_train[row][col];
			}
			
			let mean = sum / num_train_rows;
			means[col] = mean;

			// calculate standard deviation
			let mut variance_sum = 0.0;

			for row in 0..x_train.len() {
				let diff = x_train[row][col] - mean;
				variance_sum += diff * diff;
			}

			let mut std_dev = (variance_sum / num_train_rows).sqrt();
            
			if std_dev == 0.0 {
				std_dev = 1.0; // prevent division by zero
			}
			
			std_devs[col] = std_dev;

			// scale training data
			for row in 0..x_train.len() {
				x_train[row][col] = (x_train[row][col] - mean) / std_dev;
			}

			// scale test data using the *same* training mean and std_dev
			for row in 0..x_test.len() {
				x_test[row][col] = (x_test[row][col] - mean) / std_dev;
			}
		}
		
		// target scaling
		// fit and scale target values (y_train)
		let y_mean = y_train.iter().sum::<f32>() / y_train.len() as f32;
		let mut y_var_sum = 0.0;
		
		for val in y_train.iter() {
			let diff = val - y_mean;
			y_var_sum += diff * diff;
		}
		
		let mut y_std = (y_var_sum / y_train.len() as f32).sqrt();
		if y_std == 0.0 { y_std = 1.0; }
		
		for val in y_train.iter_mut() {
			*val = (*val - y_mean) / y_std;
		}

		//Self { means, std_devs }
		Self { means, std_devs, y_mean, y_std }
	}

	/// Transforms a single new unknown data sample using the saved training parameters.
	fn transform_inference_sample(&self, sample: &mut Vec<f32>) {
		for (col, val) in sample.iter_mut().enumerate() {
			*val = (*val - self.means[col]) / self.std_devs[col];
		}
	}

	// target scaling
	// convert scaled prediction back to real house value units
	fn inverse_transform_prediction(&self, scaled_pred: f32) -> f32 {
		(scaled_pred * self.y_std) + self.y_mean
	}
}

fn main() -> anyhow::Result<()> {
	//let device = Device::new_cuda(0)?;
	//let device = Device::new_metal(0)?;
	//let DEVICE = Device::new_metal(0)?;

	let file = std::fs::File::open("fetch_california_housing.json")?;
	let reader = std::io::BufReader::new(file);

	// deserialize the json into the struct
	//let data: Data = serde_json::from_reader(reader)?;
	let mut data: Data = serde_json::from_reader(reader)?;
	
	// standardize training and test data, returns saved training means and stds
	//let scaler = Scaler::fit_transform(&mut data.x_train, &mut data.x_test);
	// target scaler
	let scaler = Scaler::fit_transform(&mut data.x_train, &mut data.x_test, &mut data.y_train);

	// pretty-print to an indented json string
	//let pretty_json = serde_json::to_string_pretty(&data)?;
	//println!("{}", pretty_json);

	// note down the dimensions of the data 
	// to reshape the flatten vectors back into original shapes
	let train_d1 = data.x_train.len();
	let train_d2 = data.x_train[0].len();
	let test_d1 = data.x_test.len();
	let test_d2 = data.x_test[0].len();

    // flatten the vectors to make tensors from them 
    // we can not make tensors from multi dimensional vectors
    let x_train_vec = data.x_train.into_iter().flatten().collect::<Vec<_>>();
    let x_test_vec = data.x_test.into_iter().flatten().collect::<Vec<_>>();
    let y_train_vec = data.y_train;
    let y_test_vec = data.y_test;

	// data for training
	// removed to mini-batches optimization
    //let x_train = Tensor::from_vec(x_train_vec.clone(), (train_d1, train_d2), &DEVICE)?;
	//let y_train = Tensor::from_vec(y_train_vec, train_d1, &DEVICE)?;

    // data for testing
	let x_test = Tensor::from_vec(x_test_vec.clone(), (test_d1, test_d2), &DEVICE)?;
	let y_test = Tensor::from_vec(y_test_vec, test_d1, &DEVICE)?;

	// make model to train 
	// varmap is necessary to train/finetune the model 
	// as it keeps the track of gradients
	let varmap = VarMap::new();
	let vb = VarBuilder::from_varmap(&varmap, DType::F32, &DEVICE);
	let model = SimpleNN::new(train_d2, vb)?;
	//varmap.load("pytorch_model.safetensors")?;
	let optim_config = candle_nn::ParamsAdamW {
		lr: 1e-2,
		//lr: 3e-3, // lowered slightly for stability with deeper networks
		weight_decay: 1e-5,
		..Default::default()
	};
	let mut optimizer = candle_nn::AdamW::new(varmap.all_vars(), optim_config)?;

	// traning and evaluate model
	//train_model(&model, &x_train, &y_train, &mut optimizer, 500)?;
	println!("starting training...");
	//train_model(&model, &x_train, &y_train, &mut optimizer, 5000)?;
	//train_model(&model, &x_train, &y_train, &mut optimizer, 10000)?;
	//train_model(&model, &x_train, &y_train, &mut optimizer, 25000)?;
	//train_model(&model, &x_train, &y_train, &mut optimizer, 50000)?;
	
	// added to mini-batches optimization
	train_model(&model, &x_train_vec, &y_train_vec, train_d1, train_d2, &mut optimizer, 5000)?; // 500, 1000, 5000

	println!("evaluating model...");
	evaluate_model(&model, &x_test, &y_test, &scaler)?;

	// save the trained model weights
	println!("saving model weights...");
	varmap.save("simple_nn_v1.safetensors")?;
	println!("model successfully saved to 'simple_nn_v1.safetensors'");
	
	// example, running inference on an unknown new data point
	println!("\nrunning inference on a new sample...");

	// raw feature vector for a single new house
	let mut new_house_features = vec![8.3252, 41.0, 6.9841, 1.0238, 322.0, 2.5556, 37.88, -122.23];

	// standardize the inference sample using the fitted training scaler parameters
	scaler.transform_inference_sample(&mut new_house_features);

	// convert to a 1-row tensor
	let x_infer = Tensor::from_vec(new_house_features, (1, train_d2), &DEVICE)?;

	// forward pass
	//let prediction = model.forward(&x_infer)?;
	let prediction_scaled = model.forward(&x_infer)?.flatten_all()?.to_vec1::<f32>()?[0];

	//let predicted_value = prediction.to_scalar::<f32>()?;
	//let predicted_value = prediction.to_vec1::<f32>()?[0];
	//let predicted_value = prediction.flatten_all()?.to_vec1::<f32>()?[0];
	// convert back to real dollar value
	let predicted_value = scaler.inverse_transform_prediction(prediction_scaled);
	//println!("predicted house value: {:.4}", predicted_value);
	println!("predicted house value: ${:.2}", predicted_value * 100000.0); // or direct value depending on your dataset scale

	Ok(())
}