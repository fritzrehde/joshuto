use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::textfield::JoshutoTextField;
use crate::ui;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct NewDirectory;

impl NewDirectory {
    pub fn new() -> Self {
        NewDirectory
    }
    pub const fn command() -> &'static str {
        "mkdir"
    }

    fn new_directory(
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), std::io::Error> {
        let (term_rows, term_cols) = ui::getmaxyx();
        const PROMPT: &str = ":mkdir ";

        let user_input: Option<String> = {
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
            );
            textfield.readline()
        };

        if let Some(user_input) = user_input {
            let path = path::PathBuf::from(user_input);

            std::fs::create_dir_all(&path)?;
            ReloadDirList::reload(context, view)?;
        }
        ncurses::doupdate();
        Ok(())
    }
}

impl JoshutoCommand for NewDirectory {}

impl std::fmt::Display for NewDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewDirectory {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        match Self::new_directory(context, view) {
            Ok(_) => Ok(()),
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}
