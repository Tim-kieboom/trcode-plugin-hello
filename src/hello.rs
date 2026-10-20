use trcode_plugin_api::{FromPlugin, Plugin, ToPlugin, WireButton, WireLine, WireView};

const GREETINGS: [(&str, &str); 5] = [
    ("English", "Hello"),
    ("Nederlands", "Hallo"),
    ("Français", "Bonjour"),
    ("Español", "Hola"),
    ("Italiano", "Ciao"),
];

const TITLE_SLOT: &str = "explorer.dir";
const TEXT_SLOT: &str = "explorer.file";
const HINT_SLOT: &str = "explorer.hint";

/// The pane: which greeting it shows.
#[derive(Debug, Default)]
pub struct Hello {
    greeting: usize,
}

impl Hello {
    fn next(&mut self) {
        self.greeting = (self.greeting + 1) % GREETINGS.len();
    }

    fn view(&self) -> WireView {
        let (language, word) = GREETINGS[self.greeting];
        let line = |slot: &str, text: String| WireLine::new(text, slot);
        WireView::Column {
            parts: vec![
                WireView::Buttons {
                    buttons: vec![
                        WireButton::new("next", "Next"),
                        WireButton::new("reset", "Reset"),
                    ],
                },
                WireView::lines([
                    line(TITLE_SLOT, format!("{word}, TerminalCode!")),
                    line(TEXT_SLOT, format!("in {language}")),
                    line(HINT_SLOT, "n: next greeting, r: reset".to_owned()),
                ]),
            ],
        }
    }
}

impl Plugin for Hello {
    fn handle(&mut self, message: ToPlugin) -> Vec<FromPlugin> {
        match message {
            ToPlugin::Action { action } => match action.as_str() {
                "next" => self.next(),
                "reset" => self.greeting = 0,
                _ => {}
            },
            ToPlugin::Press { button } => match button.as_str() {
                "next" => self.next(),
                "reset" => self.greeting = 0,
                _ => {}
            },
            _ => {}
        }
        vec![FromPlugin::SetView { view: self.view() }]
    }
}
