# Whitewash

Whitewash is python binding for Ammonia.

Ammonia is a whitelist-based HTML sanitization library. It is designed to prevent cross-site scripting, layout breaking, and clickjacking caused by untrusted user-provided HTML being mixed into a larger web page.

Ammonia uses html5ever to parse and serialize document fragments the same way browsers do, so it is extremely resilient to syntactic obfuscation.

Ammonia parses its input exactly according to the HTML5 specification; it will not linkify bare URLs, insert line or paragraph breaks, or convert (C) into ©.

## Installation

```sh
python -m pip install maturin
maturin build
pythom -m pip install target/wheels/whitewash-0.1.0*whl 
```

## Usage

```python
import whitewash

input = "Pirate <script>alert(1)</script>Ahoy!"
assert whitewash.clean(input) == "Pirate Ahoy!"
assert whitewash.clean_text(input) == (
    "Pirate&#32;&lt;script&gt;alert(1)&lt;&#47;script&gt;Ahoy!"
)
assert whitewash.is_html(input)
```

## License

MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
