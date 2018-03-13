# cryptoaudit
Catallaxy Cryptocurrencies Auditing suite

## Building

### Build Prerequisites

In order to compile and run cryptoaudit on your machine, you should have installed:

* <b>Git</b> - to clone the repository
* <b>Rust</b> - 1.21.0 or greater via [Rustup](https://www.rustup.rs/) - Can be installed via your package manager or manually via the following commands:
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

### Build Instructions (Linux/Unix)


#### Clone Cryptoaudit

```
git clone https://github.com/quentinlesceller/cryptoaudit.git
```

#### Build Cryptoaudit
```sh
cd cryptoaudit
git checkout 0.1
cargo build
```

## Usage

You can use the ```private-assets.json``` to perform tests.

For example sign with the message "Catallaxy":

```
$ ./cryptoaudit sign -f private-assets.json -m "Catallaxy"

Signing message: Catallaxy
Effectively signing hash of the message which is: "d16a832ecd509c3807eea4e0c52dbf87688d96924270e4faf1047738732bebe6"
Starting Bitcoin Signing Process
Done Bitcoin Signing Process
Starting BitcoinCash Signing Process
Done BitcoinCash Signing Process
Starting Ethereum Signing Process
Done Ethereum Signing Process
Starting Counterparty Signing Process
Done Counterparty Signing Process
Successfully signed message with keys
```

This will create a signed-assets.json.

Then, to verify a signature file you need to write:

```
$ ./cryptoaudit verify -f signed-assets.json

Started verification for file "signed-assets.json"
Signature verification passed.
```
