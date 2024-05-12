use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.on_cursor_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.on_cursor_down();
        }
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
            app.on_enter();
        }
        KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => {
            app.on_backspace();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            } else {
                app.on_toggle_coloring();
            }
        }
        KeyCode::Char('s') => {
            app.on_toggle_sorting();
        }
        KeyCode::Char('d') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.toggle_debug();
            } else {
                app.on_delete();
            }
        }
        KeyCode::Char('t') => {
            app.on_toggle_move_to_trash();
        }
        _ => {}
    }
    Ok(())
}
