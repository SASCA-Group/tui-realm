//! ## List
//!
//! `List` represents a read-only textual list component which can be scrollable through arrows or inactive

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
use crate::event::KeyCode;
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, Table as TextTable};
use crate::tui::{
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List as TuiList, ListItem, ListState},
};
use crate::{Component, Event, Frame, Msg, Payload};

// -- Props

const COLOR_HIGHLIGHTED: &str = "highlighted-color";
const PROP_SCROLLABLE: &str = "scrollable";
const PROP_HIGHLIGHTED_TXT: &str = "highlighted-txt";
const PROP_MAX_STEP: &str = "max-step";
const PROP_TABLE: &str = "table";
const PROP_TITLE: &str = "title";

pub struct ListPropsBuilder {
    props: Option<Props>,
}

impl Default for ListPropsBuilder {
    fn default() -> Self {
        ListPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for ListPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for ListPropsBuilder {
    fn from(props: Props) -> Self {
        ListPropsBuilder { props: Some(props) }
    }
}

impl ListPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for area
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for area
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### with_highlighted_color
    ///
    /// Set color for highlighted entry
    pub fn with_highlighted_color(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.palette.insert(COLOR_HIGHLIGHTED, color);
        }
        self
    }

    /// ### with_borders
    ///
    /// Set component borders style
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    /// ### bold
    ///
    /// Set bold property for component
    pub fn bold(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::BOLD;
        }
        self
    }

    /// ### italic
    ///
    /// Set italic property for component
    pub fn italic(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::ITALIC;
        }
        self
    }

    /// ### underlined
    ///
    /// Set underlined property for component
    pub fn underlined(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::UNDERLINED;
        }
        self
    }

    /// ### slow_blink
    ///
    /// Set slow_blink property for component
    pub fn slow_blink(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::SLOW_BLINK;
        }
        self
    }

    /// ### rapid_blink
    ///
    /// Set rapid_blink property for component
    pub fn rapid_blink(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::RAPID_BLINK;
        }
        self
    }

    /// ### reversed
    ///
    /// Set reversed property for component
    pub fn reversed(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::REVERSED;
        }
        self
    }

    /// ### strikethrough
    ///
    /// Set strikethrough property for component
    pub fn strikethrough(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::CROSSED_OUT;
        }
        self
    }

    /// ### with_rows
    ///
    /// Set rows
    /// You can define a title if you want. The title will be displayed on the upper border of the box
    pub fn with_rows(&mut self, table: TextTable) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_TABLE, PropPayload::One(PropValue::Table(table)));
        }
        self
    }

    /// ### with_title
    ///
    /// Set title
    pub fn with_title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_TITLE,
                PropPayload::One(PropValue::Str(title.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_highlighted_str
    ///
    /// Display a symbol to highlighted line in scroll table
    pub fn with_highlighted_str(&mut self, s: Option<&str>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match s {
                None => {
                    props.own.remove(PROP_HIGHLIGHTED_TXT);
                }
                Some(s) => {
                    props.own.insert(
                        PROP_HIGHLIGHTED_TXT,
                        PropPayload::One(PropValue::Str(s.to_string())),
                    );
                }
            }
        }
        self
    }

    /// ### with_max_scroll_step
    ///
    /// Defines the max step for PAGEUP/PAGEDOWN keys
    pub fn with_max_scroll_step(&mut self, step: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_MAX_STEP, PropPayload::One(PropValue::Usize(step)));
        }
        self
    }

    /// ### scrollable
    ///
    /// Sets whether the list is scrollable
    pub fn scrollable(&mut self, scrollable: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_SCROLLABLE,
                PropPayload::One(PropValue::Bool(scrollable)),
            );
        }
        self
    }
}

// -- States

struct OwnStates {
    focus: bool,
    list_index: usize, // Index of selected item in textarea
    list_len: usize,   // Lines in text area
}

impl OwnStates {
    /// ### set_list_len
    ///
    /// Set list length
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// ### incr_list_index
    ///
    /// Incremenet list index
    pub fn incr_list_index(&mut self) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        }
    }

    /// ### decr_list_index
    ///
    /// Decrement list index
    pub fn decr_list_index(&mut self) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        }
    }

    /// ### fix_list_index
    ///
    /// Keep index if possible, otherwise set to lenght - 1
    pub fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else if self.list_len == 0 {
            self.list_index = 0;
        }
    }

    /// ### list_index_at_first
    ///
    /// Set list index to the first item in the list
    pub fn list_index_at_first(&mut self) {
        self.list_index = 0;
    }

    /// ### list_index_at_last
    ///
    /// Set list index at the last item of the list
    pub fn list_index_at_last(&mut self) {
        if self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else {
            self.list_index = 0;
        }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
    pub fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max {
            max
        } else {
            remaining
        }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
    pub fn calc_max_step_behind(&self, max: usize) -> usize {
        if self.list_index > max {
            max
        } else {
            self.list_index
        }
    }
}

// -- Component

/// ## List
///
/// represents a read-only text component without any container.
pub struct List {
    props: Props,
    states: OwnStates,
}

impl List {
    /// ### new
    ///
    /// Instantiates a new `Table` component.
    pub fn new(props: Props) -> Self {
        let len: usize = match props.own.get(PROP_TABLE).as_ref() {
            Some(PropPayload::One(PropValue::Table(t))) => t.len(),
            _ => 0,
        };
        List {
            props,
            states: OwnStates {
                focus: false,
                list_index: 0,
                list_len: len,
            },
        }
    }

    /// ### scrollable
    ///
    /// returns the value of the scrollable flag; by default is false
    fn scrollable(&self) -> bool {
        match self.props.own.get(PROP_SCROLLABLE) {
            Some(PropPayload::One(PropValue::Bool(scrollable))) => *scrollable,
            _ => false,
        }
    }
}

impl Component for List {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        if self.props.visible {
            let active: bool = match self.scrollable() {
                true => self.states.focus,
                false => true,
            };
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let div: Block = super::utils::get_block(&self.props.borders, title, active);
            // Make list entries
            let list_items: Vec<ListItem> = match self.props.own.get(PROP_TABLE).as_ref() {
                Some(PropPayload::One(PropValue::Table(table))) => table
                    .iter()
                    .map(|row| {
                        let columns: Vec<Span> = row
                            .iter()
                            .map(|col| {
                                let (fg, bg, modifiers) =
                                    super::utils::use_or_default_styles(&self.props, col);
                                Span::styled(
                                    col.content.clone(),
                                    Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                                )
                            })
                            .collect();
                        ListItem::new(Spans::from(columns))
                    })
                    .collect(), // Make List item from TextSpan
                _ => Vec::new(),
            };
            let highlighted_color: Color = match self.props.palette.get(COLOR_HIGHLIGHTED) {
                None => match self.states.focus {
                    true => self.props.background,
                    false => self.props.foreground,
                },
                Some(color) => *color,
            };
            let (fg, bg): (Color, Color) = match active {
                true => (self.props.background, highlighted_color),
                false => (highlighted_color, self.props.background),
            };
            // Make list
            let mut list = TuiList::new(list_items)
                .block(div)
                .start_corner(Corner::TopLeft)
                .highlight_style(
                    Style::default()
                        .fg(fg)
                        .bg(bg)
                        .add_modifier(self.props.modifiers),
                );
            // Highlighted symbol
            if let Some(PropPayload::One(PropValue::Str(highlight))) =
                self.props.own.get(PROP_HIGHLIGHTED_TXT)
            {
                list = list.highlight_symbol(highlight);
            }
            if self.scrollable() {
                let mut state: ListState = ListState::default();
                state.select(Some(self.states.list_index));
                render.render_stateful_widget(list, area, &mut state);
            } else {
                render.render_widget(list, area);
            }
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // re-Set list length
        self.states
            .set_list_len(match self.props.own.get(PROP_TABLE).as_ref() {
                Some(PropPayload::One(PropValue::Table(t))) => t.len(),
                _ => 0,
            });
        // Fix list index
        self.states.fix_list_index();
        // disable if scrollable
        if self.scrollable() {
            self.blur();
        }
        // Return None
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a copy of the component properties.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    fn on(&mut self, ev: Event) -> Msg {
        // Return key
        if let Event::Key(key) = ev {
            if self.scrollable() {
                match key.code {
                    KeyCode::Down => {
                        // Go down
                        self.states.incr_list_index();
                        Msg::OnKey(key)
                    }
                    KeyCode::Up => {
                        // Go up
                        self.states.decr_list_index();
                        Msg::OnKey(key)
                    }
                    KeyCode::PageDown => {
                        // Scroll by step
                        let step: usize = self.states.calc_max_step_ahead(
                            match self.props.own.get(PROP_MAX_STEP) {
                                Some(PropPayload::One(PropValue::Usize(step))) => *step,
                                _ => 8,
                            },
                        );
                        (0..step).for_each(|_| self.states.incr_list_index());
                        Msg::OnKey(key)
                    }
                    KeyCode::PageUp => {
                        // Scroll by step
                        let step: usize = self.states.calc_max_step_behind(
                            match self.props.own.get(PROP_MAX_STEP) {
                                Some(PropPayload::One(PropValue::Usize(step))) => *step,
                                _ => 8,
                            },
                        );
                        (0..step).for_each(|_| self.states.decr_list_index());
                        Msg::OnKey(key)
                    }
                    KeyCode::End => {
                        self.states.list_index_at_last();
                        Msg::OnKey(key)
                    }
                    KeyCode::Home => {
                        self.states.list_index_at_first();
                        Msg::OnKey(key)
                    }
                    _ => Msg::OnKey(key),
                }
            } else {
                Msg::OnKey(key)
            }
        } else {
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns always None
    fn get_state(&self) -> Payload {
        Payload::None
    }

    // -- events

    /// ### blur
    ///
    /// Blur component
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {
        if self.scrollable() {
            self.states.focus = true;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::props::{TableBuilder, TextSpan};

    use crossterm::event::KeyEvent;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_list_scrollable() {
        // Make component
        let mut component: List = List::new(
            ListPropsBuilder::default()
                .with_foreground(Color::Red)
                .with_background(Color::Blue)
                .with_highlighted_color(Color::Yellow)
                .hidden()
                .visible()
                .bold()
                .italic()
                .rapid_blink()
                .reversed()
                .slow_blink()
                .strikethrough()
                .underlined()
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_highlighted_str(Some("🚀"))
                .with_max_scroll_step(4)
                .scrollable(true)
                .with_title("my data")
                .with_rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("name"))
                        .add_col(TextSpan::from("age"))
                        .add_row()
                        .add_col(TextSpan::from("omar"))
                        .add_col(TextSpan::from("1"))
                        .add_row()
                        .add_col(TextSpan::from("mark"))
                        .add_col(TextSpan::from("2"))
                        .add_row()
                        .add_col(TextSpan::from("tom"))
                        .add_col(TextSpan::from("3"))
                        .add_row()
                        .add_col(TextSpan::from("pippo"))
                        .add_col(TextSpan::from("5"))
                        .add_row()
                        .add_col(TextSpan::from("carl"))
                        .add_col(TextSpan::from("8"))
                        .add_row()
                        .add_col(TextSpan::from("charlie"))
                        .add_col(TextSpan::from("13"))
                        .add_row()
                        .add_col(TextSpan::from("thomas"))
                        .add_col(TextSpan::from("21"))
                        .add_row()
                        .add_col(TextSpan::from("cammello"))
                        .add_col(TextSpan::from("34"))
                        .build(),
                )
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert!(component.props.modifiers.intersects(Modifier::BOLD));
        assert!(component.props.modifiers.intersects(Modifier::ITALIC));
        assert!(component.props.modifiers.intersects(Modifier::UNDERLINED));
        assert!(component.props.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(component.props.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(component.props.modifiers.intersects(Modifier::REVERSED));
        assert!(component.props.modifiers.intersects(Modifier::CROSSED_OUT));
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(
            component.props.palette.get(COLOR_HIGHLIGHTED).unwrap(),
            &Color::Yellow
        );
        assert_eq!(
            component.props.own.get(PROP_HIGHLIGHTED_TXT).unwrap(),
            &PropPayload::One(PropValue::Str(String::from("🚀")))
        );
        assert_eq!(
            component.props.own.get(PROP_MAX_STEP).unwrap(),
            &PropPayload::One(PropValue::Usize(4))
        );
        assert_eq!(component.states.list_len, 9);
        assert_eq!(component.states.list_index, 0);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Check messages
        // Handle inputs
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Down))),
            Msg::OnKey(KeyEvent::from(KeyCode::Down))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Up))),
            Msg::OnKey(KeyEvent::from(KeyCode::Up))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageDown))),
            Msg::OnKey(KeyEvent::from(KeyCode::PageDown))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 5);
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageDown))),
            Msg::OnKey(KeyEvent::from(KeyCode::PageDown))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 8);
        // Index should be 0
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageUp))),
            Msg::OnKey(KeyEvent::from(KeyCode::PageUp))
        );
        assert_eq!(component.states.list_index, 4);
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageUp))),
            Msg::OnKey(KeyEvent::from(KeyCode::PageUp))
        );
        assert_eq!(component.states.list_index, 0);
        // End
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::End))),
            Msg::OnKey(KeyEvent::from(KeyCode::End))
        );
        assert_eq!(component.states.list_index, 8);
        // Home
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Home))),
            Msg::OnKey(KeyEvent::from(KeyCode::Home))
        );
        assert_eq!(component.states.list_index, 0);
        // Update
        let props = ListPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .hidden()
            .with_rows(
                TableBuilder::default()
                    .add_col(TextSpan::from("name"))
                    .add_col(TextSpan::from("age"))
                    .build(),
            )
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.visible, false);
        assert_eq!(component.states.list_len, 1);
        assert_eq!(component.states.list_index, 0);
        // Get value
        assert_eq!(component.get_state(), Payload::None);
        // Event
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }

    #[test]
    fn test_components_list() {
        // Make component
        let mut component: List = List::new(
            ListPropsBuilder::default()
                .with_foreground(Color::Red)
                .with_background(Color::Blue)
                .hidden()
                .visible()
                .bold()
                .italic()
                .rapid_blink()
                .reversed()
                .slow_blink()
                .strikethrough()
                .underlined()
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("name"))
                        .add_col(TextSpan::from("age"))
                        .add_row()
                        .add_col(TextSpan::from("omar"))
                        .add_col(TextSpan::from("24"))
                        .build(),
                )
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert!(component.props.modifiers.intersects(Modifier::BOLD));
        assert!(component.props.modifiers.intersects(Modifier::ITALIC));
        assert!(component.props.modifiers.intersects(Modifier::UNDERLINED));
        assert!(component.props.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(component.props.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(component.props.modifiers.intersects(Modifier::REVERSED));
        assert!(component.props.modifiers.intersects(Modifier::CROSSED_OUT));
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        component.active();
        component.blur();
        // Update
        let props = ListPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.visible, false);
        // Get value
        assert_eq!(component.get_state(), Payload::None);
        // Event
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }
}
