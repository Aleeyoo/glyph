# glyph

A lightweight, modern, Emacs-compatible text editor, rewritten from [mg](https://github.com/hboetes/mg) in Rust.

## Status

Pre-alpha. Skeleton only. Not usable yet.

## Design

See [openspec](https://github.com/Aleeyoo/mg/blob/master/openspec/changes/mg-rust-rewrite/) for full proposal, design, and task breakdown.

**Key goals:**
- 95%+ Emacs keybinding compatibility
- UTF-8 native
- tree-sitter syntax highlighting
- Embedded miniLisp extension language
- Binary < 2MB, startup < 20ms
- No LSP, no package manager, no org-mode — just a great editor

## Build

```bash
cargo build --release
```

## License

Public domain, same as mg.
