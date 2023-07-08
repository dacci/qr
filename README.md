# QR Code decoder / encoder

## Usage

```console
$ qr --help
QR Code decoder / encoder

Usage: qr <COMMAND>

Commands:
  decode  Decodes QR Code from an image file
  encode  Encodes QR Code from a string
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```console
$ qr decode --help
Decodes QR Code from an image file

Usage: qr decode [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Path to the image to decode

Options:
  -e, --encoding <ENCODING>  Character encoding to use [default: UTF-8]
  -h, --help                 Print help
```

```console
$ qr encode --help
Encodes QR Code from a string

Usage: qr encode [OPTIONS] [DATA]

Arguments:
  [DATA]  Data to be encoded

Options:
  -m, --micro              Generates Micro QR Code. (requires --version)
  -v, --version <VERSION>  The version of the generated image. (1 to 40 for normal, 1 to 4 for micro)
  -l, --level <LEVEL>      The error correction level. (L/M/Q/H) [default: L]
  -f, --file <FILE>        Path to a file contains data to be encoded
  -h, --help               Print help
```

## Example

```console
$ qr encode -v 2 https://github.com/dacci/qr
█████████████████████████████████
█████████████████████████████████
████ ▄▄▄▄▄ ██▄▀▀    ▀█ ▄▄▄▄▄ ████
████ █   █ █▄  ▀▄██▄██ █   █ ████
████ █▄▄▄█ ██▀▄██▄  ▀█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█ ▀ █ ▀ █▄█▄▄▄▄▄▄▄████
████▄ ▄▀▄ ▄ ▄█  ▄ ██   ███▄█▀████
████▀██ ▀ ▄█▀▄██▄▀██ ██▄██▄ ▄████
████▄▀█▀▀█▄▄▀▀██▀ █▄▄▄ ██ █▄ ████
████▄▀▀ ▀ ▄▄▀▀█▀█▀ █▄ ▄▄▀█▄ ▄████
████▄█▄██▄▄▄▀ ▄█▀ █▀ ▄▄▄  █▀▀████
████ ▄▄▄▄▄ █ ███▀  ▄ █▄█  █▄ ████
████ █   █ █▄▀ ▄▄ ██▄▄▄   ▀██████
████ █▄▄▄█ █ ▀█▀█  ▄ ▀▀█ ▀█▀▄████
████▄▄▄▄▄▄▄█▄▄▄▄▄▄█▄▄█▄▄███▄▄████
█████████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```
