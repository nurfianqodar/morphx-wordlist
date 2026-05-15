# morphx-wordlist

Fast **keyword-based wordlist generator** for combinatorics, password pattern
expansion, and text transformation.

Generate large-scale **wordlists from keywords** using transforms, permutations,
combinations, and custom combiners. Designed for **security research,
penetration testing, and automation pipelines**.


## Features

- Keyword-based wordlist generation
- Permutation, combination, and cartesian product sampling
- Transform pipeline (uppercase, lowercase, leetspeak, sponge case, titlecase, reverse)
- Prefix & suffix injection
- Custom combiners (concat, separator, random symbols)
- Randomized symbol insertion
- High-performance streaming output
- CLI-first design for scripting & automation


## Use Cases

- Password wordlist generation
- Penetration testing / security research
- Brute-force input generation
- Username / credential pattern generation
- Combinatoric text expansion
- Dataset augmentation for text pipelines


## Installation

### Build from Source
```bash
git clone https://github.com/nurfianqodar/morphx-wordlist
cd morphx-wordlist
cargo build --release
```

## LICENSE

MIT

Copyright (c) 2026 Nurifan Qodar.
