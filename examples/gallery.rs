//! ## Gallery
//!
//! `Gallery` is a demo which is used to test all the available components

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
extern crate tui;
extern crate tuirealm;

mod utils;

use utils::context::Context;
use utils::keymap::*;

use std::thread::sleep;
use std::time::{Duration, Instant};

// realm
use tuirealm::components::{
    checkbox, input, label, list, paragraph, progress_bar, radio, span, textarea,
};
use tuirealm::props::borders::{BorderType, Borders};
use tuirealm::props::{TableBuilder, TextSpan};
use tuirealm::{InputType, Msg, PropPayload, PropValue, PropsBuilder, Update, View};
// tui
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;

// -- components

const COMPONENT_CHECKBOX: &str = "CHECKBOX";
const COMPONENT_INPUT: &str = "INPUT";
const COMPONENT_LABEL: &str = "LABEL";
const COMPONENT_PARAGRAPH: &str = "PARAGRAPH";
const COMPONENT_PROGBAR: &str = "PROGBAR";
const COMPONENT_RADIO: &str = "RADIO";
const COMPONENT_SPAN: &str = "SPAN";
const COMPONENT_SCROLLIST: &str = "SCROLLTABLE";
const COMPONENT_TABLE: &str = "TABLE";
const COMPONENT_TEXTAREA: &str = "TEXTAREA";

// -- application model

struct Model {
    quit: bool,
    redraw: bool,
    last_redraw: Instant,
    view: View,
}

impl Model {
    fn new() -> Model {
        let view: View = init_view();
        Model {
            quit: false,
            redraw: true,
            last_redraw: Instant::now(),
            view,
        }
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn redraw(&mut self) {
        self.redraw = true;
    }

    fn reset(&mut self) {
        self.redraw = false;
        self.last_redraw = Instant::now();
    }
}

fn main() {
    // Create context
    let mut ctx: Context = Context::new();
    // Enter alternate screen
    ctx.enter_alternate_screen();
    // Clear screen
    ctx.clear_screen();
    // Initialize view
    let mut model = Model::new();
    // Poll input events
    while !model.quit {
        // read events
        if let Ok(Some(ev)) = ctx.input_hnd.read_event() {
            let msg = model.view.on(ev);
            model.redraw();
            model.update(msg);
        }
        // If redraw, draw interface
        if model.redraw || model.last_redraw.elapsed() > Duration::from_millis(50) {
            view(&mut ctx, &model.view);
            model.reset();
        }
        sleep(Duration::from_millis(10));
    }
    drop(ctx);
}

// -- View

fn init_view() -> View {
    let mut view: View = View::init();
    // Init components
    // Checkbox
    view.mount(
        COMPONENT_CHECKBOX,
        Box::new(checkbox::Checkbox::new(
            checkbox::CheckboxPropsBuilder::default()
                .with_color(Color::Cyan)
                .with_borders(Borders::ALL, BorderType::Rounded, Color::Magenta)
                .with_value(vec![1])
                .with_title("Select ice-cream flavours")
                .with_options(&[
                    "Vanilla",
                    "Chocolate",
                    "Coconut",
                    "Strawberry",
                    "Lemon",
                    "Unicorn 🦄",
                ])
                .build(),
        )),
    );
    // Input
    view.mount(
        COMPONENT_INPUT,
        Box::new(input::Input::new(
            input::InputPropsBuilder::default()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                .with_foreground(Color::LightYellow)
                .with_input(InputType::Text)
                .with_input_len(16)
                .with_label(String::from("Type in your username (max length 16)"))
                .build(),
        )),
    );
    // Label
    view.mount(
        COMPONENT_LABEL,
        Box::new(label::Label::new(
            label::LabelPropsBuilder::default()
                .bold()
                .italic()
                .rapid_blink()
                .reversed() // CAUSE COLORS TO BE INVERTED !!!
                .underlined()
                .with_foreground(Color::Red)
                .with_background(Color::Black)
                .with_text(String::from(
                    "Press <ESC> to QUIT!!! Change focus with <TAB>",
                ))
                .build(),
        )),
    );
    // Paragraph
    view.mount(
        COMPONENT_PARAGRAPH,
        Box::new(paragraph::Paragraph::new(
            paragraph::ParagraphPropsBuilder::default()
            .italic()
            .with_background(Color::White)
            .with_foreground(Color::Black)
            .with_borders(Borders::ALL, BorderType::Rounded, Color::Gray)
            .with_title("A poem for you")
            .with_texts(vec![
                TextSpan::new("Lorem ipsum dolor sit amet").underlined().fg(Color::Green),
                TextSpan::from(", consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est. Sed sed dui odio. Etiam nunc neque, sodales ut ex nec, tincidunt malesuada eros. Sed quis eros non felis sodales accumsan in ac risus"),
                TextSpan::from("Duis augue diam, tempor vitae posuere et, tempus mattis ligula.")
            ])
            .build()
        ))
    );
    // Progres bar
    view.mount(
        COMPONENT_PROGBAR,
        Box::new(progress_bar::ProgressBar::new(
            progress_bar::ProgressBarPropsBuilder::default()
                .with_background(Color::Black)
                .with_progbar_color(Color::Yellow)
                .with_borders(Borders::ALL, BorderType::Thick, Color::Yellow)
                .with_progress(0.64)
                .with_title("Downloading termscp 0.5.0")
                .with_label("64.2% - ETA 00:48")
                .build(),
        )),
    );
    // Radio
    view.mount(
        COMPONENT_RADIO,
        Box::new(radio::Radio::new(
            radio::RadioPropsBuilder::default()
                .with_color(Color::Magenta)
                .with_borders(
                    Borders::BOTTOM | Borders::TOP,
                    BorderType::Double,
                    Color::Magenta,
                )
                .with_inverted_color(Color::Black)
                .with_value(1)
                .with_title("Will you use tui-realm in your next project?")
                .with_options(&["Yes!", "No", "Maybe"])
                .build(),
        )),
    );
    // Span
    view.mount(
        COMPONENT_SPAN,
        Box::new(span::Span::new(
            span::SpanPropsBuilder::default()
                .bold()
                .with_foreground(Color::Green)
                .with_background(Color::Black)
                .with_spans(vec![
                    TextSpan::from("THIS IS A SPAN: "),
                    TextSpan::new("Hello ")
                        .italic()
                        .slow_blink()
                        .fg(Color::Black)
                        .bg(Color::White),
                    TextSpan::new("World!")
                        .bold()
                        .underlined()
                        .rapid_blink()
                        .fg(Color::Red),
                ])
                .build(),
        )),
    );
    // ScrollList
    view.mount(
        COMPONENT_SCROLLIST,
        Box::new(list::List::new(
            list::ListPropsBuilder::default()
                .with_borders(Borders::ALL, BorderType::Thick, Color::Blue)
                .with_highlighted_str(Some("🚀"))
                .with_max_scroll_step(4)
                .with_highlighted_color(Color::LightBlue)
                .scrollable(true)
                .with_title("My scrollable data")
                .with_rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("0"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("andreas").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("1"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("bohdan").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("2"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("charlie").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("3"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("denis").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("4"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("ector").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("5"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("frank").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("6"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("giulio").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("7"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("hermes").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("8"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("italo").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("9"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("lamar").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("10"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("mark").fg(Color::Cyan))
                        .add_row()
                        .add_col(TextSpan::from("11"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::new("napalm").fg(Color::Cyan))
                        .build(),
                )
                .build(),
        )),
    );
    // Table
    view.mount(
        COMPONENT_TABLE,
        Box::new(list::List::new(
            list::ListPropsBuilder::default()
                .with_foreground(Color::Green)
                .with_borders(Borders::ALL, BorderType::Thick, Color::LightGreen)
                .with_title("My data")
                .with_rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("2021-04-17T18:32:00"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("The cat has just left the house"))
                        .add_row()
                        .add_col(TextSpan::from("2021-04-17T18:36:00"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("The cat approached a duck"))
                        .add_row()
                        .add_col(TextSpan::from("2021-04-17T18:36:03"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("The duck has flown away"))
                        .add_row()
                        .add_col(TextSpan::from("2021-04-17T18:37:00"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("The cat has met his fiance mimì"))
                        .add_row()
                        .add_col(TextSpan::from("2021-04-17T18:37:10"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Mimì thinks the cat is harassing her"))
                        .add_row()
                        .add_col(TextSpan::from("2021-04-17T18:38:52"))
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("The cat is very sad and has come back home"))
                        .build(),
                )
                .build(),
        )),
    );
    // Textarea
    view.mount(
        COMPONENT_TEXTAREA,
        Box::new(textarea::Textarea::new(
            textarea::TextareaPropsBuilder::default()
                .with_foreground(Color::White)
                .italic()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightRed)
                .with_highlighted_str(Some("🎵"))
                .with_max_scroll_step(3)
                .with_title("Scrollable textarea")
                .with_texts(
                    vec![
                        TextSpan::new("About TermSCP").bold().underlined().fg(Color::Yellow),
                        TextSpan::from("TermSCP is basically a porting of WinSCP to terminal. So basically is a terminal utility with an TUI to connect to a remote server to retrieve and upload files and to interact with the local file system. It works both on Linux, MacOS, BSD and Windows and supports SFTP, SCP, FTP and FTPS."),
                        TextSpan::new("Why TermSCP 🤔").bold().underlined().fg(Color::Cyan),
                        TextSpan::from("It happens quite often to me, when using SCP at work to forget the path of a file on a remote machine, which forces me to connect through SSH, gather the file path and finally download it through SCP. I could use WinSCP, but I use Linux and I pratically use the terminal for everything, so I wanted something like WinSCP on my terminal. Yeah, I know there is midnight commander too, but actually I don't like it very much tbh (and hasn't a decent support for scp)."),
                        TextSpan::from("Lorde ~ Green Light"),
                        TextSpan::from("I do my makeup in somebody else's car We order different drinks at the same bars I know about what you did and I wanna scream the truth She thinks you love the beach, you're such a damn liar ")
                    ]
                )
                .build(),
        )),
    );
    // Focus
    view.active(COMPONENT_CHECKBOX);
    view
}

fn view(ctx: &mut Context, view: &View) {
    let _ = ctx.terminal.draw(|f| {
        // Prepare chunks
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        // Make columns
        let lcol = Layout::default()
            .constraints(
                [
                    Constraint::Length(3), // Checkbox
                    Constraint::Length(3), // Input
                    Constraint::Length(1), // Label
                    Constraint::Length(3), // Progress bar
                    Constraint::Length(3), // Radio
                    Constraint::Length(6), // Paragraph
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(chunks[0]);
        let rcol = Layout::default()
            .constraints(
                [
                    Constraint::Length(6), // ScrollList
                    Constraint::Length(1), // Span
                    Constraint::Length(8), // Table
                    Constraint::Length(8), // Textarea
                    Constraint::Length(1), // End textarea
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(chunks[1]);
        // Render
        // left
        view.render(COMPONENT_CHECKBOX, f, lcol[0]);
        view.render(COMPONENT_INPUT, f, lcol[1]);
        view.render(COMPONENT_LABEL, f, lcol[2]);
        view.render(COMPONENT_PROGBAR, f, lcol[3]);
        view.render(COMPONENT_RADIO, f, lcol[4]);
        view.render(COMPONENT_PARAGRAPH, f, lcol[5]);
        // right
        view.render(COMPONENT_SCROLLIST, f, rcol[0]);
        view.render(COMPONENT_SPAN, f, rcol[1]);
        view.render(COMPONENT_TABLE, f, rcol[2]);
        view.render(COMPONENT_TEXTAREA, f, rcol[3]);
    });
}

impl Update for Model {
    fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                (COMPONENT_CHECKBOX, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_INPUT);
                    // Update progress
                    let msg = update_progress(&mut self.view);
                    self.update(msg)
                }
                (COMPONENT_INPUT, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_RADIO);
                    // Update progress
                    let msg = update_progress(&mut self.view);
                    self.update(msg)
                }
                (COMPONENT_RADIO, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_SCROLLIST);
                    // Update progress
                    let msg = update_progress(&mut self.view);
                    self.update(msg)
                }
                (COMPONENT_SCROLLIST, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_TEXTAREA);
                    // Update progress
                    let msg = update_progress(&mut self.view);
                    self.update(msg)
                }
                (COMPONENT_TEXTAREA, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_CHECKBOX);
                    // Update progress
                    let msg = update_progress(&mut self.view);
                    self.update(msg)
                }
                (comp, Msg::OnSubmit(payload)) => {
                    let props = label::LabelPropsBuilder::from(
                        self.view.get_props(COMPONENT_LABEL).unwrap(),
                    )
                    .with_text(format!("GOT SUBMIT EVENT FROM '{}': {:?}", comp, payload))
                    .build();
                    // Report submit
                    self.view.update(COMPONENT_LABEL, props);
                    // Update progress
                    let msg = update_progress(&mut self.view);
                    self.update(msg)
                }
                (_, &MSG_KEY_ESC) => {
                    // Quit
                    self.quit();
                    None
                }
                _ => None,
            },
        }
    }
}

// -- misc

fn update_progress(view: &mut View) -> Option<(String, Msg)> {
    let props = view.get_props(COMPONENT_PROGBAR).unwrap();
    let new_prog: f64 = match props.own.get("progress") {
        Some(PropPayload::One(PropValue::F64(val))) => match val + 0.05 > 1.0 {
            true => 0.0,
            false => val + 0.05,
        },
        _ => 0.0,
    };
    view.update(
        COMPONENT_PROGBAR,
        progress_bar::ProgressBarPropsBuilder::from(props)
            .with_progress(new_prog)
            .with_label(format!("{:.2}% - ETA 00:30", new_prog * 100.0))
            .build(),
    )
}
