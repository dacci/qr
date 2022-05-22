# QR Code decoder / encoder

## Usage

```console
$ qr --help
qr 0.3.0
QR Code decoder / encoder

USAGE:
    qr <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decodes QR Code from an image file
    encode    Encodes QR Code from a string
    help      Print this message or the help of the given subcommand(s)
```

```console
$ qr decode --help
qr-decode
Decodes QR Code from an image file

USAGE:
    qr decode [OPTIONS] <IMAGE>

ARGS:
    <IMAGE>    Path to the image to decode

OPTIONS:
    -e, --encoding <ENCODING>    Character encoding to use [default: UTF-8]
    -h, --help                   Print help information
```

```console
$ qr encode --help
qr-encode
Encodes QR Code from a string

USAGE:
    qr encode [OPTIONS] <DATA>

ARGS:
    <DATA>    Data to be encoded

OPTIONS:
    -h, --help                 Print help information
    -l, --level <LEVEL>        The error correction level. (L/M/Q/H) [default: L]
    -m, --micro                Generates Micro QR Code. (requires --version)
    -v, --version <VERSION>    The version of the generated image. (1 to 40 for normal, 1 to 4 for
                               micro)
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
