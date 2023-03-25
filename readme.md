# bropilot

bropilot is a CLI tool that lets you write terminal commands in plain English. It is inspired by GitHub Copilot X's CLI functionality, which is currently on a waitlist. This tool uses OpenAI's GPT-3.5 model to generate bash commands and their explanations based on user input.

![Demo Image Placeholder](/demo-img.svg)

## Installation

You can install bropilot through Cargo:

```sh
cargo install bropilot
```

## Build Instructions

Clone the repository:

```sh
git clone https://github.com/yourusername/bropilot.git
cd bropilot
```

In the backend folder, create a file called .dev.vars and add your OpenAI API key:

```sh
OPENAI_KEY=your_openai_api_key_here
```

Run the Cloudflare Worker in the `backend` folder:

```sh
cd backend
wrangler run dev
```

Update the .env file in the parent directory with the worker URL:

```sh
WORKER_URL=https://your_worker_url_here
```

Build and run the CLI tool:

```sh
cd ..
cargo build --release
./target/release/bropilot
```

## Usage

After installing, you can run it by simply typing bropilot followed by your query:

```sh
bropilot "print hello world"
```

The CLI will provide a shell command and a brief explanation. You can choose to run the command, revise your query, or cancel the operation.
