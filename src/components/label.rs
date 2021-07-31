//! ## Label
//!
//! `Label` represents a read-only text component without any container.

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
use crate::props::{Alignment, PropPayload, PropValue, Props, PropsBuilder};
use crate::tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};
use crate::{Component, Event, Frame, Msg, Payload};

const PROP_ALIGNMENT: &str = "text-alignment";
const PROP_TEXT: &str = "text";

// -- Props

pub struct LabelPropsBuilder {
    props: Option<Props>,
}

impl Default for LabelPropsBuilder {
    fn default() -> Self {
        LabelPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for LabelPropsBuilder {
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

impl From<Props> for LabelPropsBuilder {
    fn from(props: Props) -> Self {
        LabelPropsBuilder { props: Some(props) }
    }
}

impl LabelPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for component
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for component
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
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

    /// ### with_text
    ///
    /// Set text
    pub fn with_text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_TEXT,
                PropPayload::One(PropValue::Str(text.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_text_alignment
    ///
    /// Set text alignment for paragraph
    pub fn with_text_alignment(&mut self, alignment: Alignment) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_ALIGNMENT,
                PropPayload::One(PropValue::Alignment(alignment)),
            );
        }
        self
    }
}

// -- Component

/// ## Label
///
/// represents a read-only text component without any container.
pub struct Label {
    props: Props,
}

impl Label {
    /// ### new
    ///
    /// Instantiates a new `Label` component.
    pub fn new(props: Props) -> Self {
        Label { props }
    }
}

impl Component for Label {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.visible {
            // Make text
            let text: String = match self.props.own.get(PROP_TEXT).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => t.to_string(),
                _ => String::default(),
            };
            // Text properties
            let alignment: Alignment = match self.props.own.get(PROP_ALIGNMENT) {
                Some(PropPayload::One(PropValue::Alignment(alignment))) => *alignment,
                _ => Alignment::Left,
            };
            render.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background)
                            .add_modifier(self.props.modifiers),
                    )
                    .alignment(alignment),
                area,
            );
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
            Msg::OnKey(key)
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
    fn blur(&mut self) {}

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::tui::style::Color;
    use crossterm::event::{KeyCode, KeyEvent};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_label() {
        let mut component: Label = Label::new(
            LabelPropsBuilder::default()
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
                .with_text(String::from("Hello, world!"))
                .with_text_alignment(Alignment::Center)
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
        assert_eq!(
            component.props.own.get(PROP_TEXT).unwrap(),
            &PropPayload::One(PropValue::Str("Hello, world!".to_string()))
        );
        assert_eq!(
            *component.props.own.get(PROP_ALIGNMENT).unwrap(),
            PropPayload::One(PropValue::Alignment(Alignment::Center))
        );
        component.active();
        component.blur();
        // Update
        let props = LabelPropsBuilder::from(component.get_props())
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

    #[test]
    fn test_components_label_propsbuilder() {
        let props: Props = LabelPropsBuilder::default()
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .strikethrough()
            .reversed()
            .rapid_blink()
            .slow_blink()
            .with_text(String::from("test"))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert!(props.modifiers.intersects(Modifier::BOLD));
        assert!(props.modifiers.intersects(Modifier::ITALIC));
        assert!(props.modifiers.intersects(Modifier::UNDERLINED));
        assert!(props.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(props.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(props.modifiers.intersects(Modifier::REVERSED));
        assert!(props.modifiers.intersects(Modifier::CROSSED_OUT));
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(
            props.own.get(PROP_TEXT).unwrap(),
            &PropPayload::One(PropValue::Str("test".to_string()))
        );
    }
}
