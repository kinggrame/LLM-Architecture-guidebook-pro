//! LLM Engine - 高性能 LLM 推理引擎
//!
//! 支持多种模型和量化格式的命令行推理工具

use clap::Parser;
use llm_engine::{
    tokenizer::{create_gpt2_tokenizer, create_llama_tokenizer, TokenizeResult, Tokenizer},
    InferenceConfig, ModelLoader, ModelType,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "llm_engine")]
#[clap(about = "High-performance LLM inference engine")]
struct Args {
    #[clap(short, long, default_value = "models/gpt2.bin")]
    model: PathBuf,

    #[clap(short, long, default_value = "gpt2")]
    model_type: String,

    #[clap(short, long, default_value = "gpt2")]
    tokenizer_type: String,

    #[clap(long)]
    quantization: Option<String>,

    #[clap(short, long)]
    prompt: Option<String>,

    #[clap(short, long)]
    interactive: bool,

    #[clap(short, long, default_value = "100")]
    max_length: usize,

    #[clap(long, default_value = "0.7")]
    temperature: f32,

    #[clap(long, default_value = "40")]
    top_k: usize,

    #[clap(long, default_value = "0.9")]
    top_p: f32,

    #[clap(long, default_value = "1.1")]
    repeat_penalty: f32,

    #[clap(long, default_value = "4")]
    threads: usize,

    #[clap(long, default_value = "1")]
    batch_size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("LLM Engine v{}", env!("CARGO_PKG_VERSION"));
    println!("========================\n");

    let tokenizer: Box<dyn Tokenizer> = match args.tokenizer_type.as_str() {
        "llama" => Box::new(create_llama_tokenizer()?),
        _ => Box::new(create_gpt2_tokenizer()?),
    };

    println!(
        "Tokenizer: {}, vocab_size: {}",
        args.tokenizer_type,
        tokenizer.vocab_size()
    );

    let config = InferenceConfig {
        max_length: args.max_length,
        temperature: args.temperature,
        top_k: args.top_k,
        top_p: args.top_p,
        repeat_penalty: args.repeat_penalty,
        use_kv_cache: true,
        batch_size: args.batch_size,
        num_threads: args.threads,
    };

    println!(
        "Config: max_length={}, temperature={}, top_k={}, top_p={}",
        config.max_length, config.temperature, config.top_k, config.top_p
    );

    let model_type = match args.model_type.as_str() {
        "llama" => ModelType::LLaMA,
        "gptj" => ModelType::GPTJ,
        "falcon" => ModelType::Falcon,
        _ => ModelType::GPT2,
    };

    let mut loader = ModelLoader::new(args.model.to_string_lossy().to_string(), model_type);

    println!("\nLoading model from {}...", args.model.display());

    let load_result = match loader.load() {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to load model: {}", e);
            println!("\nNote: This is a demo. Real model files needed for actual inference.");
            println!("Place model files in the path specified or use --help for options.\n");
            println!("Running in demo mode with simulated inference...\n");
            return run_demo_mode(&tokenizer, &config, args.prompt, args.interactive);
        }
    };

    println!("Model loaded successfully!");
    println!("  Model type: {:?}", load_result.config.model_type);
    println!("  Vocab size: {}", load_result.config.vocab_size);
    println!("  Embedding dim: {}", load_result.config.embedding_dim);
    println!("  Layers: {}", load_result.config.num_layers);
    println!("  Quantization: {:?}", load_result.quantization);

    println!("\n[Note: Full inference requires model-specific implementation]");

    if args.interactive {
        run_interactive(&tokenizer);
    } else if let Some(prompt) = args.prompt {
        run_inference(&tokenizer, &prompt);
    } else {
        println!("No prompt provided. Use --prompt or --interactive");
        println!("Use --help for more options");
    }

    Ok(())
}

fn run_demo_mode(
    tokenizer: &Box<dyn Tokenizer>,
    config: &InferenceConfig,
    prompt: Option<String>,
    interactive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if interactive {
        println!("Demo interactive mode (type 'quit' to exit):");
        loop {
            print!("> ");
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let input = input.trim();
            if input.is_empty() || input == "quit" {
                break;
            }

            println!("Input tokens: {:?}", tokenizer.encode(input)?);
            println!("[Demo: Would generate {} tokens]", config.max_length);
            println!();
        }
    } else if let Some(prompt) = prompt {
        println!("Prompt: {}", prompt);
        println!("Tokens: {:?}", tokenizer.encode(&prompt)?);
        println!("[Demo: Would generate {} tokens]", config.max_length);
    } else {
        let demo_prompt = "Hello, how are you?";
        println!("Demo prompt: {}", demo_prompt);
        println!("Tokens: {:?}", tokenizer.encode(demo_prompt)?);
        println!("[Demo: Would generate {} tokens]", config.max_length);
    }

    Ok(())
}

fn run_inference(tokenizer: &Box<dyn Tokenizer>, prompt: &str) {
    println!("\nTokenizing...");
    let tokens = match tokenizer.encode(prompt) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Tokenization error: {}", e);
            return;
        }
    };

    println!("Input tokens: {}", tokens.len());
    println!("Prompt: {}\n", prompt);
    println!("Generating...");
    println!("[Inference requires actual model implementation]");
}

fn run_interactive(tokenizer: &Box<dyn Tokenizer>) {
    println!("\nInteractive mode (type 'quit' to exit, 'clear' to clear context):");

    loop {
        print!("\n> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" {
            break;
        }

        if input == "clear" {
            println!("Context cleared");
            continue;
        }

        println!("Input: {}", input);
        println!("Tokens: {:?}", tokenizer.encode(input).unwrap_or_default());
        println!("[Inference requires actual model implementation]");
    }
}
