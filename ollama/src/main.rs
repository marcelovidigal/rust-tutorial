use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// connects to localhost:11434 by default
	let ollama = Ollama::default();

	//let model = "gpt-oss:20b".to_string();
	let model = "qwen3.5:9b".to_string();
	let prompt = "Why is the sky blue?".to_string();

	println!("ollama example");
	println!("model: {}", model);
	println!("\n{}", prompt);

	let res = ollama.generate(GenerationRequest::new(model, prompt)).await?;
	println!("\n{}", res.response);

	Ok(())
}