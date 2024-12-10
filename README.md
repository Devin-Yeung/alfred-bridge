# <a><img src="docs/images/alfred-bridge-logo.png" height="40" valign="baseline" /></a> Afred Bridge

[![Build Status][actions-badge]][actions-url]
[![Coverage Status][coveralls-badge]][coveralls-url]
[![MIT Licensed][mit-badge]][mit-url]
[![APACHE Licensed][apache-badge]][apache-url]

[actions-badge]: https://github.com/Devin-Yeung/alfred-bridge/actions/workflows/ci.yml/badge.svg?branch=master

[actions-url]: https://github.com/Devin-Yeung/alfred-bridge/actions/workflows/ci.yml

[coveralls-badge]: https://coveralls.io/repos/github/Devin-Yeung/alfred-bridge/badge.svg?branch=master

[coveralls-url]: https://coveralls.io/github/Devin-Yeung/alfred-bridge?branch=master

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[apache-badge]: https://img.shields.io/badge/license-APACHE_2.0-blue.svg

[mit-url]: https://github.com/Devin-Yeung/alfred-bridge/blob/master/LICENSE-MIT

[apache-url]: https://github.com/Devin-Yeung/alfred-bridge/blob/master/LICENSE-APACHE

# Usage

```toml
[dependencies]
alfred-bridge = { git = "https://github.com/Devin-Yeung/alfred-bridge.git", rev = "refs/heads/master" }
```

## Supported Features

| Field            |      Status       |       Comment       |
|------------------|:-----------------:|:-------------------:|
| uid              | Fully Supported ✅ |                     |
| title            | Fully Supported ✅ |                     |
| subtitle         | Fully Supported ✅ |                     |
| arg              | Fully Supported ✅ |                     |
| icon             | Fully Supported ✅ |                     |
| valid            | Fully Supported ✅ |                     |
| match            | Fully Supported ✅ |                     |
| autocomplete     | Fully Supported ✅ |                     |
| type             | Fully Supported ✅ |                     |
| mods             |    Planned  🗓    |                     |
| action           |    Planned  🗓    |                     |
| text             |    Planned  🗓    |                     |
| quicklookurl     |    Planned  🗓    |                     |
| session variable | Fully Supported ✅ |                     |
| item variable    |    Planned 🗓     |                     |
| rerun            |    Planned 🗓     |                     |
| cache            |    Planned 🗓     | Require Alfred v5.5 |
| result ordering  | Fully Supported ✅ |  Require Alfred v5  |

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
