# Procks
`procks` is a simple command line proxy server for handling UDP and TCP
connections. Leveraging the power of asynchronous Rust, `procks` is both
lightweight and performant.

## Usage
Launching a proxy server requires a protocol, a receiving address, and a
sending address. The addresses must recognizable by Rust.

```sh
$ procks -p [PROTOCOL] -r [RECEIVING] -s [SENDING]
```
