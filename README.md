# trcode-plugin-hello

A small example plugin for [TerminalCode](https://github.com/Tim-kieboom/TerminalCode): a pane
that says hello in a few languages, with two buttons. It asks for no capability, so it is the
smallest thing that is still a whole plugin; start from it to make your own.

## Installing

Add the plugin list it is in as a source, in `settings.json`:

```jsonc
"plugin_sources": ["https://raw.githubusercontent.com/Tim-kieboom/trcode-plugins/main/index.json"]
```

Open "Plugins: Manage", choose Hello and press `enter`. You are asked first (it is not an
official plugin), then where its view goes.

By hand: `cargo build --target wasm32-unknown-unknown --release`, then copy `manifest.toml` and
`target/wasm32-unknown-unknown/release/trcode_plugin_hello.wasm` (as `hello.wasm`) to
`~/.config/terminalcode/plugins/hello/`.

## Keys

| Key | Action |
| --- | --- |
| `n`, the Next button | the next greeting |
| `r`, the Reset button | back to English |

## How it is made

- `manifest.toml`: the id (it must be the name of the folder it is installed in), the plugin API
  it needs (`abi`), the module's file name, the capabilities it asks for (none) and its actions
  with their default keys.
- `src/hello.rs`: `Hello` implements `Plugin`: it is sent a message and answers with messages, here
  always a new view. It knows nothing of WebAssembly, so `src/tests.rs` tests it with plain
  `cargo test`.
- `src/lib.rs`: `export_plugin!(Hello)` makes the module.
- `.github/workflows/release.yml`: a `vX.Y.Z` tag builds the module and publishes it with the
  manifest, and prints the entry for the plugin list.

## Releasing

Set `version` in `manifest.toml`, tag `vX.Y.Z` and push the tag. Then add the printed entry to
`index.json` of [trcode-plugins](https://github.com/Tim-kieboom/trcode-plugins) (keep the older
versions listed: a published version must never change).

## License

MIT or Apache-2.0, at your option.
