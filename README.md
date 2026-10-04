# AlgoFlow (`.algo`) 🚀

**AlgoFlow** is a custom compiled programming language designed to bridge the gap between human flowchart logic and machine execution. Built from the ground up in Rust, it provides an intuitive syntax for control flow, instantly renders dynamic node-based visual diagrams (`.flw`), and transpiles logic directly into native Windows executable binaries (`.exe`).

---

## ✨ Features

- **End-to-End Compiler Pipeline**: Built with a custom lexer, parser (AST generation), and dual code-generation backends in Rust.
- **Interactive Desktop GUI**: Visualizes `.algo` decision nodes, loops, and branching structures natively using `eframe` / `egui`.
- **Flowchart Visualizer Export (`.flw`)**: Generates standalone visual HTML/Mermaid documents for quick documentation and embedding.
- **Native `.exe` Transpilation**: Converts `.algo` logic into C code and compiles it directly into high-performance Windows binaries via GCC.

---

## 🛠️ Installation & Setup

### Prerequisites

1. **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
2. **GCC Compiler**: Ensure `gcc` (e.g., MinGW for Windows) is installed and added to your system `PATH`.

### Local Installation

Clone the repository and install the `algo` CLI binary globally:

```powershell
# Clone repository
git clone [https://github.com/YOUR_USERNAME/lang-algo.git](https://github.com/YOUR_USERNAME/lang-algo.git)
cd lang-algo

# Install binary to system PATH (~/.cargo/bin)
cargo install --path .
Verify the installation:PowerShellalgo --help
💻 CLI CommandsCommandDescriptionExamplealgo run <file.algo>Launches the native desktop GUI viewer to visualize the flowchart.algo run script.algoalgo compile <file.algo>Compiles .algo into an interactive .flw GUI document.algo compile script.algo -o diagram.flwalgo build <file.algo>Transpiles .algo source code directly into a native .exe binary.algo build script.algo -o output.exe📝 Syntax Guide.algo files use clear, structured keywords to represent logic operations, decision nodes, and loopback routes:Plaintextstart
Check sensor inputs
if1{ temp > 100 }
Trigger alarm sequence
return to "1"
end
Keyword Referencestart / end: Defines program boundaries.if<id>{ condition }: Creates a decision node with a unique ID for loopback targeting.return to "<id>": Constructs a backward loop arrow targeting an if decision block.📁 Repository StructurePlaintextlang-algo/
├── .gitignore
├── .gitattributes
├── Cargo.toml
├── README.md
├── build/
│   └── algo.exe            # Pre-compiled standalone executable
├── src/
│   ├── main.rs             # CLI entrypoint and argument routing
│   ├── lib.rs              # Module exports
│   ├── ast.rs              # Abstract Syntax Tree definition
│   ├── lexer.rs            # Tokenizer
│   ├── parser.rs           # Parser engine
│   ├── flowchart.rs        # GUI desktop canvas renderer & Mermaid exporter
│   └── codegen.rs          # C-transpilation backend
└── script.algo             # Sample source file
📄 LicenseDistributed under the MIT License. See LICENSE for more information.
