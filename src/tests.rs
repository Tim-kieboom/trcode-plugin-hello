use trcode_plugin_api::{FromPlugin, Plugin, ToPlugin, WireView};

use crate::Hello;

/// The text of the view the plugin sent.
fn shown(messages: Vec<FromPlugin>) -> String {
    let Some(FromPlugin::SetView {
        view: WireView::Column { parts },
    }) = messages.into_iter().next()
    else {
        panic!("a column of buttons and lines");
    };
    let WireView::Lines { lines } = &parts[1] else {
        panic!("lines under the buttons");
    };
    lines
        .iter()
        .map(|line| line.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn action(hello: &mut Hello, id: &str) -> String {
    shown(hello.handle(ToPlugin::Action { action: id.into() }))
}

#[test]
fn it_starts_in_english() {
    let text = shown(Hello::default().handle(ToPlugin::Start));
    assert!(text.contains("Hello, TerminalCode!"), "{text}");
    assert!(text.contains("in English"), "{text}");
}

#[test]
fn next_goes_through_the_languages_and_around() {
    let mut hello = Hello::default();
    assert!(action(&mut hello, "next").contains("Hallo"));
    assert!(action(&mut hello, "next").contains("Bonjour"));
    for _ in 0..3 {
        action(&mut hello, "next");
    }
    assert!(action(&mut hello, "next").contains("Hallo"));
}

#[test]
fn reset_and_the_buttons_do_the_same_as_the_actions() {
    let mut hello = Hello::default();
    action(&mut hello, "next");
    let text = shown(hello.handle(ToPlugin::Press {
        button: "reset".into(),
    }));
    assert!(text.contains("Hello,"), "{text}");
    let text = shown(hello.handle(ToPlugin::Press {
        button: "next".into(),
    }));
    assert!(text.contains("Hallo"), "{text}");
}

#[test]
fn what_it_does_not_know_only_redraws() {
    let mut hello = Hello::default();
    assert!(action(&mut hello, "nothing").contains("Hello,"));
    assert!(shown(hello.handle(ToPlugin::Refresh)).contains("Hello,"));
}
