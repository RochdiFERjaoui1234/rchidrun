use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use reqwest::blocking::get;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, copy, Write};
use std::path::PathBuf;
use std::process::Command;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

#[derive(Parser)]
#[command(name = "rchidrun", version = "0.1.0", about = "Unified compiler for running scripts with WASM")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run a script with a language")]
    Run {
        #[arg(help = "Programming language (e.g., python, javascript)")]
        language: String,
        #[arg(help = "Path to the script")]
        script: String,
    },
    #[command(about = "List installed SDKs and supported languages")]
    SdkList,
}

fn sdk_dir() -> Result<PathBuf> {
    let home = env::var("HOME").map_err(|_| anyhow!("$HOME not set"))?;
    let mut dir = PathBuf::from(home);
    dir.push(".rchidrun/plugins");
    Ok(dir)
}

fn get_language_packages() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("python", "wasmer/python");
    map.insert("javascript", "wasmer/quickjs");
    map.insert("ruby", "wasmer/ruby");
    map
}

fn is_supported_language(language: &str) -> bool {
    get_language_packages().contains_key(language)
}

fn get_wasmer_package(language: &str) -> Option<&'static str> {
    get_language_packages().get(language).copied()
}

fn read_line() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn install_via_wasmer(language: &str) -> Result<()> {
    let package = get_wasmer_package(language).ok_or(anyhow!("Language not supported"))?;
    let mut sdk_path = sdk_dir()?;
    sdk_path.push(language);
    fs::create_dir_all(&sdk_path)?;
    let status = Command::new("wasmer")
        .args(["install", package, "--to", &sdk_path.to_string_lossy()])
        .status()
        .map_err(|e| anyhow!("Wasmer not found: {}. Please install Wasmer[](https://wasmer.io/).", e))?;
    if status.success() {
        println!("Installed '{}' via Wasmer", language);
        Ok(())
    } else {
        Err(anyhow!("Wasmer installation failed"))
    }
}

fn install_via_url(language: &str, url: &str) -> Result<()> {
    let mut sdk_path = sdk_dir()?;
    sdk_path.push(language);
    fs::create_dir_all(&sdk_path)?;
    sdk_path.push("runtime.wasm");
    let mut file = File::create(&sdk_path)?;
    let mut resp = get(url).map_err(|e| anyhow!("Failed to download: {}", e))?;
    copy(&mut resp, &mut file)?;
    println!("Installed '{}' from URL", language);
    Ok(())
}

fn run_sdk(language: &str, script: &str) -> Result<()> {
    let mut wasm_path = sdk_dir()?;
    wasm_path.push(language);
    wasm_path.push("runtime.wasm");
    let engine = Engine::default();
    let module = Module::from_file(&engine, &wasm_path)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .args(&[script])?
        .build();
    let mut store = Store::new(&engine, wasi);
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut _| ctx.clone())?;
    let instance = linker.instantiate(&mut store, &module)?;
    let start = instance
        .get_func(&mut store, "_start")
        .ok_or(anyhow!("_start function not found"))?;
    start.call(&mut store, &[], &mut [])?;
    Ok(())
}

fn run_language(language: &str, script: &str) -> Result<()> {
    let sdk_path = sdk_dir()?.join(language).join("runtime.wasm");
    if sdk_path.exists() {
        run_sdk(language, script)
    } else {
        println!("No runtime found for '{}'.", language);
        if is_supported_language(language) {
            print!("Install it via Wasmer? (y/n): ");
            io::stdout().flush()?;
            let choice = read_line()?;
            if choice.to_lowercase() == "y" {
                install_via_wasmer(language)?;
                run_sdk(language, script)
            } else {
                Err(anyhow!("Installation aborted"))
            }
        } else {
            print!("Language not predefined. Provide a URL to the WASM runtime: ");
            io::stdout().flush()?;
            let url = read_line()?;
            install_via_url(language, &url)?;
            run_sdk(language, script)
        }
    }
}

fn sdk_list() -> Result<()> {
    let dir = sdk_dir()?;
    println!("Installed SDKs:");
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(n) = entry.file_name().to_str() {
                    println!("- {}", n);
                }
            }
        }
    }
    println!("\nSupported languages (via Wasmer):");
    for (lang, pkg) in get_language_packages() {
        println!("- {} ({})", lang, pkg);
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { language, script } => run_language(&language, &script)?,
        Commands::SdkList => sdk_list()?,
    }
    Ok(())
}