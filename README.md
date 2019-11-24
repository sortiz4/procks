# Procks
Procks is a simple proxy server that supports both TCP and UDP. It's written in
asynchronous Rust and supports a high degree of concurrency.

## Usage
Launching a proxy server requires a protocol, a receiving address, and a
sending address. The addresses must recognizable by Rust.

```sh
$ procks -p [PROTOCOL] -r [RECEIVING] -s [SENDING]
```
