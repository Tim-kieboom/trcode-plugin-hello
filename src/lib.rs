//! A small example plugin for TerminalCode: a pane that says hello, in a few
//! languages, with two buttons. It asks for no capability.
//!
//! The logic is in [`Hello`], which answers messages with messages and knows
//! nothing of WebAssembly, so it is tested on the machine it is written on; the
//! last line makes it the module.

pub use hello::Hello;

mod hello;
#[cfg(test)]
mod tests;

trcode_plugin_api::export_plugin!(Hello);
