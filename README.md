# Delay VST Plugin(Rust + eframe)
**By Tayo**

A web-based replica of a Delay VST plugin built entirely in Rust using the [`egui`](https://github.com/emilk/egui) / `eframe` framework and compiled to WebAssembly (Wasm).

## 🚀 Tech Stack
* **Language:** Rust
* **GUI Framework:** `eframe` (egui)
* **Web Build Tool:** Trunk
* **Target:** WebAssembly (`wasm32-unknown-unknown`)

## 🛠️ Prerequisites

To run this project locally, you need to have Rust and Trunk installed on your machine.

**1. Install Rust:**
If you don't have Rust installed, run the following command in your terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Add the WebAssembly target: Tell Rust how to compile code for the web:

rustup target add wasm32-unknown-unknown

3. Install Trunk: Trunk is the tool we use to bundle the Rust code and the
index.html file together.

cargo install trunk

💻 How to Run Locally

1.  Clone this repository and open the folder in your terminal.
2.  Start the local development server by running:
    trunk serve
3.  Open your web browser and navigate to: http://127.0.0.1:8080

(Note: Do not open the index.html file directly in your browser, as modern
browsers block WebAssembly via the file:// protocol. Always use trunk serve!)

☁️ Running in GitHub Codespaces

If you are developing this project inside a GitHub Codespace, follow these
steps:

1.  Open a new terminal in VS Code.
2.  Ensure Rust and Trunk are installed (see Prerequisites above).
3.  Run trunk serve.
4.  Look for the PORTS tab (usually next to the TERMINAL tab at the bottom).
5.  Find Port 8080 in the list.
6.  Click the Globe icon (Open in Browser) next to the port to view the app.

📁 Project Structure

  - src/ - Contains the Rust source code for the UI and logic.
  - index.html - The entry point and template for the web app. Trunk injects the
    compiled Wasm here.
  - Trunk.toml - Configuration file for the Trunk bundler.
  - Cargo.toml - Rust dependencies and project metadata.

📄 License

MIT License


***

### How to add this to your project:
1. In your GitHub Codespace, click the **New File** button in your file explorer (on the left side).
2. Name the file exactly **`README.md`** (all caps for README).
3. Paste the code above into the file and save it.
4. Commit and push 💪 
