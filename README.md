# rchidrun

A unified compiler CLI that runs scripts in various programming languages using WebAssembly (WASM). It uses **Wasmtime** to execute WASM runtimes and **Wasmer** to install runtimes for supported languages (Python, JavaScript, Ruby). Unsupported languages can be added by providing a WASM runtime URL.

## Features
- Run scripts in Python, JavaScript, Ruby, or any language with a WASM runtime.
- Automatically installs supported languages via Wasmer if not present.
- Fallback to URL-based installation for custom languages.
- Lists installed and supported SDKs.

## How It Works
- **WASM Execution**: Uses Wasmtime to load and run language runtimes as WASM modules.
- **Wasmer Integration**: Fetches runtimes for supported languages (e.g., `wasmer/python`) from the Wasmer registry.
- **Custom Runtimes**: Allows adding unsupported languages by downloading WASM files from URLs.

## Prerequisites
- **Rust**: Install from [rustup.rs](https://rustup.rs/).
- **Wasmer CLI**: Install with `cargo install wasmer-cli` or follow [Wasmer's instructions](https://wasmer.io/).
- A system with the `HOME` environment variable set (stores SDKs in `~/.rchidrun/plugins`).

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/RochdiFERjaoui1234/rchidrun
   cd rchidrun
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. (Optional) Install globally:
   ```bash
   cargo install --path .
   ```

## Project Structure
- `src/main.rs`: Core CLI logic.
- `Cargo.toml`: Rust dependencies and metadata.

## Detailed Usage Guide

`rchidrun` provides two main commands: `run` to execute scripts and `sdk list` to view installed and supported SDKs. Below are detailed instructions for each.

### 1. Running a Script (`run`)
The `run` command executes a script in the specified language using its WASM runtime.

**Syntax**:
```bash
rchidrun run <language> <script>
```
- `<language>`: The programming language (e.g., `python`, `javascript`, `ruby`).
- `<script>`: Path to the script file to execute.

#### Example 1: Running a Python Script
If the Python runtime is already installed:
```bash
rchidrun run python examples/hello.py
```
**Script (`examples/hello.py`)**:
```python
print("Hello from Python!")
```
**Output**:
```
Hello from Python!
```

#### Example 2: Running a JavaScript Script (Missing Runtime)
If the JavaScript runtime is not installed:
```bash
rchidrun run javascript examples/script.js
```
**Prompt**:
```
No runtime found for 'javascript'.
Install it via Wasmer? (y/n): 
```
- Type `y` to install the `wasmer/quickjs` runtime.
- **Output**:
  ```
  Installed 'javascript' via Wasmer
  Hello from JavaScript!
  ```
- **Script (`examples/script.js`)**:
  ```javascript
  console.log("Hello from JavaScript!");
  ```

#### Example 3: Running a Script in an Unsupported Language
For a language not predefined (e.g., `go`):
```bash
rchidrun run go my_script.go
```
**Prompt**:
```
No runtime found for 'go'.
Language not predefined. Provide a URL to the WASM runtime: 
```
- Enter a URL to a WASM file (e.g., `https://example.com/go.wasm`).
- **Output**:
  ```
  Installed 'go' from URL
  [Output depends on the script and runtime]
  ```
- Note: The WASM runtime must support WASI and expose a `_start` function.

#### Notes
- SDKs are stored in `~/.rchidrun/plugins/<language>/runtime.wasm`.
- If you decline installation (`n` at the Wasmer prompt), the program exits with an error:
  ```
  Fatal error: Installation aborted
  ```
- Ensure the script path is valid; otherwise, you’ll see:
  ```
  Fatal error: No such file or directory
  ```

### 2. Listing SDKs (`sdk list`)
The `sdk list` command shows installed SDKs and supported languages.

**Syntax**:
```bash
rchidrun sdk list
```
**Example Output** (after installing Python and JavaScript):
```
Installed SDKs:
- python
- javascript

Supported languages (via Wasmer):
- python (wasmer/python)
- javascript (wasmer/quickjs)
- ruby (wasmer/ruby)
```

#### Notes
- Installed SDKs are directories in `~/.rchidrun/plugins`.
- Supported languages are predefined and can be installed via Wasmer.

### Troubleshooting
- **Wasmer Not Found**:
  ```
  Fatal error: Wasmer not found: [...]. Please install Wasmer (https://wasmer.io/).
  ```
  Install Wasmer CLI with `cargo install wasmer-cli`.
- **Invalid URL**:
  ```
  Fatal error: Failed to download: [...]
  ```
  Ensure the provided URL points to a valid WASM file.
- **Missing `_start` Function**:
  ```
  Fatal error: _start function not found
  ```
  The WASM runtime must be WASI-compatible with a `_start` function.
- **No $HOME**:
  ```
  Fatal error: $HOME not set
  ```
  Ensure the `HOME` environment variable is set.

## Supported Languages
- Python (`wasmer/python`)
- JavaScript (`wasmer/quickjs`)
- Ruby (`wasmer/ruby`)

For other languages, provide a WASM runtime URL when prompted.

## Editor Integration
`rchidrun` can be integrated with editors like VS Code:
- Create a VS Code extension to run `rchidrun run <language> <file>`.
- Detect the language from file extensions (e.g., `.py` → `python`).
- Display output in the editor’s terminal or output panel.

## License
MIT




   ****************Made by Rochdi Ferjaoui****************
