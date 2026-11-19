use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

/// Application state: the key/value pairs plus the current editing session.
#[derive(Debug, Default)]
pub struct App {
    key_input: String,
    value_input: String,
    pairs: BTreeMap<String, String>,
    current_screen: CurrentScreen,
    currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn screen(&self) -> CurrentScreen {
        self.current_screen
    }

    pub fn editing(&self) -> Option<CurrentlyEditing> {
        self.currently_editing
    }

    pub fn key_input(&self) -> &str {
        &self.key_input
    }

    pub fn value_input(&self) -> &str {
        &self.value_input
    }

    pub fn pairs(&self) -> &BTreeMap<String, String> {
        &self.pairs
    }

    /// Opens the editor on an empty key field.
    pub fn begin_editing(&mut self) {
        self.key_input.clear();
        self.value_input.clear();
        self.currently_editing = Some(CurrentlyEditing::Key);
        self.current_screen = CurrentScreen::Editing;
    }

    /// Moves focus between the key and value inputs.
    pub fn toggle_editing(&mut self) {
        self.currently_editing = match self.currently_editing {
            Some(CurrentlyEditing::Key) => Some(CurrentlyEditing::Value),
            Some(CurrentlyEditing::Value) => Some(CurrentlyEditing::Key),
            None => Some(CurrentlyEditing::Key),
        };
    }

    /// Enter on the key field advances to the value; Enter on the value saves the pair.
    pub fn submit_editing(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::Key) => {
                self.currently_editing = Some(CurrentlyEditing::Value);
            }
            Some(CurrentlyEditing::Value) => self.save_pair(),
            None => {}
        }
    }

    /// Stores the current pair and returns to the main screen. Empty keys are ignored.
    pub fn save_pair(&mut self) {
        if self.key_input.is_empty() {
            return;
        }

        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.cancel_editing();
    }

    /// Discards the current input and returns to the main screen.
    pub fn cancel_editing(&mut self) {
        self.key_input.clear();
        self.value_input.clear();
        self.currently_editing = None;
        self.current_screen = CurrentScreen::Main;
    }

    pub fn request_exit(&mut self) {
        self.current_screen = CurrentScreen::Exiting;
    }

    pub fn cancel_exit(&mut self) {
        self.current_screen = CurrentScreen::Main;
    }

    /// Appends a character to the focused input.
    pub fn push_char(&mut self, character: char) {
        match self.currently_editing {
            Some(CurrentlyEditing::Key) => self.key_input.push(character),
            Some(CurrentlyEditing::Value) => self.value_input.push(character),
            None => {}
        }
    }

    /// Removes the last character from the focused input.
    pub fn delete_char(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::Key) => {
                self.key_input.pop();
            }
            Some(CurrentlyEditing::Value) => {
                self.value_input.pop();
            }
            None => {}
        }
    }

    /// Prints the collected pairs to stdout as pretty-printed JSON.
    pub fn print_json(&self) -> serde_json::Result<()> {
        println!("{}", serde_json::to_string_pretty(&self.pairs)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with_pair(key: &str, value: &str) -> App {
        let mut app = App::new();
        app.begin_editing();
        app.key_input = key.to_string();
        app.value_input = value.to_string();
        app
    }

    #[test]
    fn saves_pair_on_enter_from_value_field() {
        let mut app = app_with_pair("name", "ratatui");

        app.submit_editing(); // key -> value
        app.submit_editing(); // saves

        assert_eq!(app.pairs().get("name"), Some(&"ratatui".to_string()));
        assert_eq!(app.screen(), CurrentScreen::Main);
    }

    #[test]
    fn ignores_pairs_with_an_empty_key() {
        let mut app = App::new();
        app.begin_editing();
        app.value_input = "orphan".to_string();
        app.toggle_editing(); // key -> value

        app.submit_editing();

        assert!(app.pairs().is_empty());
        assert_eq!(app.screen(), CurrentScreen::Editing);
    }

    #[test]
    fn cancelling_editing_discards_input() {
        let mut app = app_with_pair("key", "value");

        app.cancel_editing();

        assert_eq!(app.key_input(), "");
        assert_eq!(app.value_input(), "");
        assert_eq!(app.editing(), None);
        assert_eq!(app.screen(), CurrentScreen::Main);
    }
}
