# Argum Server

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Usage

```bash
cargo run
```

## Environment Variables

For a list of environment variables, see the [`.env.example`](.env.example) file.

| Variable | Description       | Default |
| -------- | ----------------- | ------- |
| `PORT`   | Port to listen on | `3000`  |

## Docker

### Development

Build the Docker image:
```bash
docker build -t argum-server:local-dev -f dev.Dockerfile .
```

Run Dev Docker image
```bash
docker run --rm -it -p 3000:3000 -v $PWD:/app argum-server
```

Start the server
```bash
cargo ran
```

### Release

Build Docker image:
```bash
docker build -t argum-server:latest .
```

Run Argum Server:
```bash
docker run --name argum-server -d -p 3000:3000 argum-server
```