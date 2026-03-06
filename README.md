# qrcodefyi

[![crates.io](https://img.shields.io/crates/v/qrcodefyi)](https://crates.io/crates/qrcodefyi)
[![docs.rs](https://docs.rs/qrcodefyi/badge.svg)](https://docs.rs/qrcodefyi)

Async Rust client for the [QRCodeFYI](https://qrcodefyi.com) API. Look up QR code types, versions, encoding modes, error correction levels, and standards.

## Install

```toml
[dependencies]
qrcodefyi = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use qrcodefyi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let results = client.search("micro qr").await?;
    println!("Found {} results", results.total);
    Ok(())
}
```

## API Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Search QR types, encodings, and glossary |
| `qr_type(slug)` | Get QR code type details |
| `version(version)` | Get QR version details (1-40) |
| `component(slug)` | Get component details |
| `encoding(slug)` | Get encoding mode details |
| `standard(slug)` | Get standard details |
| `use_case(slug)` | Get use case details |
| `glossary_term(slug)` | Get glossary term definition |
| `compare(slug_a, slug_b)` | Compare two QR code types |
| `random()` | Get a random QR code type |

All methods are async and return `Result<T, QrCodeFyiError>`.

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| Python | [qrcodefyi](https://pypi.org/project/qrcodefyi/) | `pip install qrcodefyi` |
| TypeScript | [qrcodefyi](https://www.npmjs.com/package/qrcodefyi) | `npm install qrcodefyi` |
| Go | [qrcodefyi-go](https://pkg.go.dev/github.com/fyipedia/qrcodefyi-go) | `go get github.com/fyipedia/qrcodefyi-go` |
| Rust | [qrcodefyi](https://crates.io/crates/qrcodefyi) | `cargo add qrcodefyi` |
| Ruby | [qrcodefyi](https://rubygems.org/gems/qrcodefyi) | `gem install qrcodefyi` |

## Code FYI Family

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode symbologies & standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types & encoding |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips & protocols |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags & readers |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart card platforms |

## License

MIT
