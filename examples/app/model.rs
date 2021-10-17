//! ## Model
//!
//! app model

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{terminal, Application, Id, Msg};

use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{AttrValue, Attribute, NoUserEvent, Terminal, Update, View};

pub struct Model {
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: Terminal,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            quit: false,
            redraw: true,
            terminal: terminal::init(),
        }
    }
}

impl Model {
    pub fn view(&mut self, app: &mut Application<Id, Msg, NoUserEvent>) {
        assert!(self
            .terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Clock
                            Constraint::Length(3), // Letter Counter
                            Constraint::Length(3), // Digit Counter
                            Constraint::Length(1), // Label
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                app.view(&Id::Clock, f, chunks[0]);
                app.view(&Id::LetterCounter, f, chunks[1]);
                app.view(&Id::DigitCounter, f, chunks[2]);
                app.view(&Id::Label, f, chunks[3]);
            })
            .is_ok());
    }
}

// Let's implement Update for model

impl Update<Id, Msg, NoUserEvent> for Model {
    fn update(&mut self, view: &mut View<Id, Msg, NoUserEvent>, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::Clock => None,
                Msg::DigitCounterBlur => {
                    // Give focus to letter counter
                    assert!(view.active(&Id::LetterCounter).is_ok());
                    None
                }
                Msg::DigitCounterChanged(v) => {
                    // Update label
                    assert!(view
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("DigitCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
                Msg::LetterCounterBlur => {
                    // Give focus to digit counter
                    assert!(view.active(&Id::DigitCounter).is_ok());
                    None
                }
                Msg::LetterCounterChanged(v) => {
                    // Update label
                    assert!(view
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("LetterCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
            }
        } else {
            None
        }
    }
}
