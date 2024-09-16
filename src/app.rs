use std::error::Error;

use ratatui::{
    backend::Backend, prelude::*, text::{Span, Text}, Frame, Terminal
};
use text::ToSpan;

pub struct App {
    file_name: String,
    file_contents: Vec<String>,
    exit: bool,

    cur_paragraph: Vec<char>,
    cur_paragraph_done: bool,
    paragraph_num: usize,
    cur_input: Vec<char>,
}

impl App {
    pub fn new(file_contents: Vec<&str>, file_name: String) -> App {
        let mut file_contents: Vec<String> = file_contents
            .iter()
            .filter_map(|line| {
                let line = (*line).trim().to_string();

                if line.is_empty() {
                    None
                } else {
                    Some(line)
                }
            }).collect();

        if file_contents.len() < 1 {
            file_contents.push(format!("Empty content in file [{}]", &file_name));
        }

        App {
            file_name,
            file_contents,
            cur_paragraph: Vec::new(),
            cur_paragraph_done: true,
            cur_input: Vec::new(),
            paragraph_num: 0,
            exit: false,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            if self.cur_paragraph_done {

                // next line
                if let Some(line) = self.file_contents.get(self.paragraph_num) {
                    self.cur_paragraph = line.chars().collect();
                    self.paragraph_num += 1;
                    self.cur_input.clear();
                    self.cur_paragraph_done = false;
                }
            }

            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_event()?;
        }
        Ok(())
    }
}

impl App {
    fn handle_event(&mut self) -> Result<(), Box<dyn Error>> {
        use ratatui::crossterm::event::{read, Event, KeyCode, KeyEventKind};

        match read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Up => {
                    if self.paragraph_num > 1 {
                        self.cur_paragraph = self.file_contents[self.paragraph_num - 2].chars().collect();
                        self.paragraph_num -= 1;
                        self.cur_input.clear();
                    }
                }
                KeyCode::Tab => {
                    self.cur_input.clear();
                }
                KeyCode::Down => {
                    if self.paragraph_num < self.file_contents.len() {
                        self.cur_paragraph_done = true;
                    }
                }
                KeyCode::Backspace => {
                    self.cur_input.pop();
                }
                KeyCode::Char(ch) => {
                    if self.cur_paragraph.len() > self.cur_input.len() {
                        self.cur_input.push(ch);
                    }

                    // finish a line
                    if self.cur_paragraph.eq(&self.cur_input) {
                        self.cur_paragraph_done = true;
                    }

                }
                KeyCode::Right => {
                    if self.cur_paragraph.len() > self.cur_input.len() {
                        self.cur_input.push(self.cur_paragraph[self.cur_input.len()]);
                    }

                    // finish a line
                    if self.cur_paragraph.eq(&self.cur_input) {
                        self.cur_paragraph_done = true;
                    }
                }
                KeyCode::Left => {
                    if self.cur_input.len() > 0 {
                        self.cur_input.pop();

                    // back to end of last line
                    } else if self.paragraph_num > 1 {
                        self.cur_paragraph = self.file_contents[self.paragraph_num - 2].chars().collect();
                        self.cur_input = self.cur_paragraph.clone();
                        self.cur_input.pop();
                        self.paragraph_num -= 1;
                    }
                }
                _ => {}
            }
            _ => {}
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        use ratatui::{
            layout::{Constraint, Layout},
            widgets::{
                block::Position, block::Title,
                Block, Borders, Paragraph, Wrap,
                Padding,
            },
        };

        let layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(23),
            Constraint::Fill(1),
        ])
        .split(frame.area());

        let layout = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(90),
            Constraint::Fill(1),
        ])
        .split(layout[1]);

        let file_contents_len = self.file_contents.len();
        let block = Block::default()
            .title(Title::from(vec![
                " ".to_span(),
                self.file_name.to_span().bold().yellow(),
                " ".to_span(),
                "[".to_span().on_light_magenta().white(),
                self.paragraph_num.to_span().on_light_magenta().white(),
                "/".to_span().on_light_magenta().white(),
                file_contents_len.to_span().on_light_magenta().white(),
                "]".to_span().on_light_magenta().white(),
                " ".to_span(),
            ]))
            .title(
                Title::from(vec![
                    " <Esc>".to_owned().bold().green(),
                    ": Quit, ".to_owned().green(),
                    "<Tab>".to_owned().bold().yellow(),
                    ": Clear, ".to_owned().yellow(),
                    "<Up>".to_owned().bold().red(),
                    ": Last, ".to_owned().red(),
                    "<Down>".to_owned().bold().cyan(),
                    ": Next ".to_owned().cyan(),
                ])
                .alignment(Alignment::Center)
                .position(Position::Bottom)
            )
            .padding(Padding::symmetric(2, 1))
            .border_style(Style::new().blue())
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(self.build_text())
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, layout[1]);
    }

    /// build three line text
    fn build_text(&self) -> Text {
        use ratatui::style::{Style, Stylize};

        let mut text = Text::default();

        // Top line for readed
        if self.paragraph_num > 1 {
            text.lines.push(
                Line::raw(&self.file_contents[self.paragraph_num - 2])
                .italic()
                .green()
            );
        }
        text.lines.push(Line::default());

        // Current middle line
        let mut index = 0;
        let mut build_line = Line::default();

        while index < self.cur_paragraph.len() {

            if let Some(ch) = self.cur_paragraph.get(index) {
                if let Some(ch_input) = self.cur_input.get(index) {

                    build_line.push_span(
                        if ch.eq(ch_input) {
                            Span::styled(
                                ch.to_string(),
                                Style::new().green().bold()
                            )
                        } else {
                            Span::styled(
                                ch.to_string(),
                                Style::new().on_red()
                            )
                        }
                    );
                    index += 1;

                } else {
                    build_line.push_span(Span::styled(
                        ch.to_string(),
                        Style::new()
                    ));
                    index += 1;
                }

            } else {
                break;
            }
        }

        // set style for cursor position
        if let Some(span) = build_line.spans.get_mut(self.cur_input.len()) {
            span.style = Style::new().on_light_cyan();
        }
        text.lines.push(build_line);
        text.lines.push(Line::default());

        // Bottom line for prepare reading
        if self.file_contents.len() > self.paragraph_num {
            text.lines.push(
                Line::raw(&self.file_contents[self.paragraph_num])
                .italic()
                .magenta()
            );
        } else {
            text.lines.push(Line::raw(String::new()));
        }

        text
    }
}

