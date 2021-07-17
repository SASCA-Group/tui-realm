//! ## Components
//!
//! `Components` provides a "standard" library of components.

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
// Modules
pub mod bar_chart;
pub mod canvas;
pub mod chart;
pub mod checkbox;
pub mod input;
pub mod label;
pub mod list;
pub mod paragraph;
pub mod progress_bar;
pub mod radio;
pub mod select;
pub mod span;
pub mod sparkline;
pub mod table;
pub mod textarea;
pub mod utils;

// Exports
pub use bar_chart::{BarChart, BarChartPropsBuilder};
pub use canvas::{Canvas, CanvasPropsBuilder};
pub use chart::{Chart, ChartPropsBuilder};
pub use checkbox::{Checkbox, CheckboxPropsBuilder};
pub use input::{Input, InputPropsBuilder};
pub use label::{Label, LabelPropsBuilder};
pub use list::{List, ListPropsBuilder};
pub use paragraph::{Paragraph, ParagraphPropsBuilder};
pub use progress_bar::{ProgressBar, ProgressBarPropsBuilder};
pub use radio::{Radio, RadioPropsBuilder};
pub use select::{Select, SelectPropsBuilder};
pub use span::{Span, SpanPropsBuilder};
pub use sparkline::{Sparkline, SparklinePropsBuilder};
pub use table::{Table, TablePropsBuilder};
pub use textarea::{Textarea, TextareaPropsBuilder};
