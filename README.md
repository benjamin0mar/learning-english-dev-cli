# LearningEnglishDev — English Writing Assistant for Developers

**LearningEnglishDev** (`led`) is a CLI tool that helps non-native software developers write better English for commits, PR descriptions, and AI prompts. It uses AI (Groq or Ollama) to provide corrections at 5 CEFR levels (A1–C1), including vocabulary definitions and explanations.

```text
$ led "fix the bug"

╭──────────────────────────────────────────────╮
│ LearningEnglishDev — English for devs         │
│  Provider: groq  |  Input: commit            │
╰──────────────────────────────────────────────╯

 ──  Level B1  ────────────────────────────────

Corrected Text: Fixed the bug

 Changes:
  ~  fix  →  Fixed

Explanation:
  Changed the verb tense to past tense to indicate
  the bug has been resolved.

Vocabulary:
+----------+---------------------------------+
| Word     | Definition                      |
+----------+---------------------------------+
| resolved | solved or fixed                 |
+----------+---------------------------------+
```

## Features

- **5 CEFR levels** — from A1 (basic fixes) to C1 (senior-level refinement)
- **Multiple input types** — commit messages, AI prompts, PR descriptions
- **Two AI backends** — Groq (cloud, fast) or Ollama (local, private)
- **History** — view past analyses with `history` subcommand
- **Flexible input** — argument, pipe (stdin), or file
- **JSON output** — for integration with other tools
- **Colored output** — clear visual diff of changes
- **Short alias** — use `led` instead of `learning-english-dev-cli`

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.75 or later)
- An API key for [Groq](https://console.groq.com/) **or** [Ollama](https://ollama.ai/) running locally

### From source

```bash
git clone https://github.com/omarbenjamin/learning-english-dev-cli.git
cd learning-english-dev-cli
cargo build --release
cp target/release/led ~/.local/bin/
```

Or run directly:

```bash
cargo run -- "your text here"
```

After installation, both `led` and `learning-english-dev-cli` are available.

## Configuration

Configuration is stored in `config/config.toml` inside the project directory. A default template is at `config/default.toml`.

### Quick setup

```bash
# Show current config
led config

# Set your Groq API key
led config groq_api_key "gsk_your_key_here"

# Or use Ollama (local)
led config provider ollama
led config ollama_model "llama3.2"
```

### Environment variables (override config file)

| Variable | Description |
|---|---|
| `GROQ_API_KEY` | Your Groq API key |
| `OLLAMA_BASE_URL` | Ollama server URL (default: `http://localhost:11434`) |
| `OLLAMA_MODEL` | Ollama model name |
| `DEVCOACH_PROVIDER` | Provider: `groq`, `ollama`, or `auto` |

You can also create a `config/.env` file in the project:

```env
GROQ_API_KEY=gsk_your_key_here
DEVCOACH_PROVIDER=groq
```

## Usage

### Basic

```bash
# Analyze a commit message
led "fix the bug"

# Specify input type
led "write a function to sort a list" -t prompt

# Filter by level (B1 only)
led "refactor auth module" -l b1

# Read from file
led --file pr.txt -t pr

# Pipe from stdin
echo "implement rate limiting" | led -t commit

# JSON output
led "fix the bug" --json
```

### Using Groq (cloud)

1. Get a free API key at [console.groq.com](https://console.groq.com/)
2. Set it:

```bash
led config groq_api_key "gsk_..."
led config provider groq
```

### Using Ollama (local, no API key needed)

1. Install Ollama from [ollama.ai](https://ollama.ai/)
2. Pull a model:

```bash
ollama pull llama3.2
```

3. Configure LearningEnglishDev:

```bash
led config provider ollama
led config ollama_model "llama3.2"
```

4. The tool will send requests to `http://localhost:11434` by default.

### Examples by input type

```bash
# Git commit message
led "fix the bug" -t commit

# AI/coding prompt
led "explain how to implement a binary search tree" -t prompt

# Pull request description
led "add user authentication with JWT" -t pr
```

### History

```bash
# View last 10 analyses
led history

# View last 5
led history -n 5

# As JSON
led history --json

# Clear history
led history --clear
```

## How it works

1. You provide text (commit, prompt, or PR description)
2. The tool sends it to an AI backend (Groq or Ollama)
3. The AI returns 5 corrected versions, one per CEFR level
4. For each level, you get: corrected text, a diff of changes, an explanation, and vocabulary

### CEFR Levels

| Level | Description |
|---|---|
| **A1** | Fix only critical grammar/spelling errors |
| **A2** | Fix grammar + improve clarity slightly |
| **B1** | Fix grammar + improve clarity + standard dev terminology |
| **B2** | Professional tone + precise technical vocabulary |
| **C1** | Refined, idiomatic, senior-level English |

## Contributing

Contributions are welcome! Here's how to help:

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

### Development

```bash
# Run in debug mode with the led alias
cargo run --bin led -- "test text"

# Run tests
cargo test

# Build release
cargo build --release
```

### Guidelines

- Keep the CLI interface simple and consistent
- Maintain support for both Groq and Ollama backends
- Add tests for new features
- Follow existing code style

## License

This project is licensed under **CC BY-NC 4.0** — see [LICENSE](LICENSE).

You are free to use, modify, and share this tool for **non-commercial purposes only**. Commercial use requires explicit permission.

## Acknowledgments

- [Groq](https://groq.com/) for fast inference API
- [Ollama](https://ollama.ai/) for local LLM serving
- [CEFR](https://www.coe.int/en/web/common-european-framework-reference-languages) language level framework
