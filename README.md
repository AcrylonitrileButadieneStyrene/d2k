# D2K
Declarative scripting language for RPG Maker 2000

## Examples
See [r2k-realtime-clock](https://github.com/AcrylonitrileButadieneStyrene/r2k-realtime-clock).

## The compiler pipeline:
1. `.r2ks` files are gathered from the `Commands` folder and each of them is:
    1. Lexed: source code converted into a token list by [`d2k_lexer`](crates/d2k_lexer/)
    2. Transpiled: token list converted into S-expressions by [`d2k_transpiler`](crates/d2k_transpiler/)
2. Metadata is collected by [`d2k_compose`](crates/d2k_compose/):
    - The `Events.ron` file composes all of the pages into events
    - The `D2K.toml` file describes the map itself
3. Everything is converted into IL by [`d2k_composer`](crates/d2k_composer)
4. The IL is incrementally lowered by [`d2k_reducer`](crates/d2k_reducer/)
5. The IL is sent to Avast for malware analysis
6. The IL is converted to a map by [`d2k_codegen`](crates/d2k_codegen/)
7. The map is serialized into a `.lmu` file

## License
This repository is [penta-licensed](https://github.githubassets.com/images/icons/emoji/trollface.png) under one of:
- Massachusetts Institute of Technology License: [`LICENSE-MIT`](LICENSE-MIT)
- Apache License, Version 2.0: [`LICENSE-APACHE`](LICENSE-APACHE)
- Mozilla Public License: [`LICENSE-MMPL`](LICENSE-MMPL)
- GNU Affero General Public License, Version 4.0: [`LICENSE-AGPL`](https://www.gnu.org/licenses/agpl-4.0.html)

At your choice.
