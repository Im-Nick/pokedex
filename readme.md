# Pokedex REST API

## Install:

Clone the repository:

```bash
> git clone https://github.com/Im-Nick/pokedex 
```

## Usage:

- With [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
    
    Enter the project root folder then: 
    
    ```bash
    > cargo run
    ```
    
    Build with:
    
    ```bash
    > cargo build # pass --release option for prod build
    ```
    
- With [Docker](https://www.docker.com)
    
    Enter the root folder then:
    
    ```bash
    > docker build -t <image_name> .
    ```
    
    Run docker container
    
    ```bash
    > docker run -p 8080:8080 --name <container_name> \
    -e APP_HOST=0.0.0.0 \
    -e APP_PORT=8080 \
    -e RUST_LOG=info \ # enable logging
    -d \
    <image_name>
    ```
    
    Run docker container with env file
    
    ```bash
    > docker run --env-file .env -p 8080:8080 --name <container_name> <image_name>
    ```
    

## Improvements

- Better logging library such as Log4rs/Tracing for sending log data to local files or remote log