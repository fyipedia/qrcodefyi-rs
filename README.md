# qrcodefyi

[![crates.io](https://img.shields.io/crates/v/qrcodefyi.svg)](https://crates.io/crates/qrcodefyi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust client for the [QRCodeFYI](https://qrcodefyi.com) REST API. QR code types. Uses `reqwest` for HTTP.

> **Explore at [qrcodefyi.com](https://qrcodefyi.com)** — interactive tools and comprehensive reference.

## Install

```toml
[dependencies]
qrcodefyi = "0.1"
```

## Quick Start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = qrcodefyi::Client::new();
    let result = client.search("query")?;
    println!("{:?}", result);
    Ok(())
}
```

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **Python** | `pip install qrcodefyi` | [PyPI](https://pypi.org/project/qrcodefyi/) |
| **npm** | `npm install qrcodefyi` | [npm](https://www.npmjs.com/package/qrcodefyi) |
| **Go** | `go get github.com/fyipedia/qrcodefyi-go` | [pkg.go.dev](https://pkg.go.dev/github.com/fyipedia/qrcodefyi-go) |
| **Rust** | `cargo add qrcodefyi` | [crates.io](https://crates.io/crates/qrcodefyi) |
| **Ruby** | `gem install qrcodefyi` | [rubygems](https://rubygems.org/gems/qrcodefyi) |


## Links

- **Site**: [qrcodefyi.com](https://qrcodefyi.com)
- **API**: [qrcodefyi.com/api/v1/](https://qrcodefyi.com/api/v1/)
- **OpenAPI**: [qrcodefyi.com/api/v1/schema/](https://qrcodefyi.com/api/v1/schema/)
- **Glossary**: [qrcodefyi.com/glossary/](https://qrcodefyi.com/glossary/)
- **Guides**: [qrcodefyi.com/guide/](https://qrcodefyi.com/guide/)
- **Tools**: [qrcodefyi.com/tools/](https://qrcodefyi.com/tools/)
Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## Tag FYI Family

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode formats, EAN, UPC, ISBN standards |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy, GATT, beacons |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips, tag types, NDEF, standards |
| **QRCodeFYI** | [qrcodefyi.com](https://qrcodefyi.com) | QR code types, versions, encoding modes |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags, frequency bands, standards |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart cards, EMV, APDU, Java Card |

## Embed Widget

Embed [QRCodeFYI](https://qrcodefyi.com) widgets on any website with [qrcodefyi-embed](https://widget.qrcodefyi.com):

```html
<script src="https://cdn.jsdelivr.net/npm/qrcodefyi-embed@1/dist/embed.min.js"></script>
<div data-qrcodefyi="entity" data-slug="example"></div>
```

Zero dependencies · Shadow DOM · 4 themes (light/dark/sepia/auto) · [Widget docs](https://widget.qrcodefyi.com)

## License

MIT
