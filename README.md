# Auriga

Auriga is a simple project that simulates a basic Redis server in Rust. It's a key-value store that supports `GET` and `SET` commands.

## Getting Started

### Prerequisites

- Rust: You need to have Rust installed on your machine. You can download it from the [official website](https://www.rust-lang.org/tools/install).

### Running the Server

1. Clone the repository: `git clone https://github.com/yourusername/auriga.git`
2. Navigate into the project directory: `cd auriga`
3. Run the server: `cargo run`

The server will start on `localhost` at port `6379`.

## Usage

You can interact with the server using any TCP client. Here's an example using `telnet`:

```shell
$ telnet localhost 6379
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
SET mykey myvalue
OK
GET mykey
myvalue
```