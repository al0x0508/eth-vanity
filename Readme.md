# ETH VANITY

A tool to generate Ethereum vanity addresses.

## Installation

```bash
$ cargo build --release
```

## CLI

```
A tool to generate Ethereum vanity addresses

Usage: eth-vanity [OPTIONS] -s <STARTS_WITH>

Options:
  -s <STARTS_WITH>      Name of the person to greet
  -c <COUNT>            Number of addresses to generate [default: 1]
  -h, --help            Print help
  -V, --version         Print version
```

## Usage

```
$ eth-vanity -s aaaa
Starting vanity address search...

Successfully found vanity address in 153.587542ms
Address: 0xaaaabb4e9627e96e47ff559f7a9883549cdcbd31
Private Key: 0x782443d1ec75dbe6726104b478a2e502637749fd5ae167eb460528f5e0862799
```

