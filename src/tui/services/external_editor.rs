use std::io::stdout;
use std::path::Path;

use crossterm::ExecutableCommand;
use crossterm::cursor;
use crossterm::terminal::{
    self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use crate::error::{JanusError, Result};
use crate::utils::open_in_editor;

pub struct ExternalEditor;

impl ExternalEditor {
    /// Open a ticket file in the user's $EDITOR, suspending the TUI.
    ///
    /// This temporarily exits the alternate screen and raw mode so the
    /// editor can take full control of the terminal. After the editor
    /// exits, terminal state is restored for iocraft to resume.
    pub fn open_ticket_file(path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(JanusError::FileNotFound(path.to_string_lossy().to_string()));
        }

        // Suspend TUI terminal state
        Self::suspend_terminal()
            .map_err(|e| JanusError::TuiError(format!("Failed to suspend terminal: {e}")))?;

        // Launch editor (blocks until editor closes)
        let editor_result = open_in_editor(path);

        // Always restore terminal state, even if editor failed
        let restore_result = Self::restore_terminal();

        // Return the first error encountered (editor error takes priority)
        editor_result?;
        restore_result.map_err(|e| JanusError::TuiError(format!("Failed to restore terminal: {e}")))
    }

    fn suspend_terminal() -> std::io::Result<()> {
        let mut out = stdout();
        out.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        out.execute(cursor::Show)?;
        Ok(())
    }

    fn restore_terminal() -> std::io::Result<()> {
        let mut out = stdout();
        out.execute(cursor::Hide)?;
        out.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        out.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }
}
