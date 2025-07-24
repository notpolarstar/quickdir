use std::io;
use std::env;
use std::fs::write;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame};

pub mod dir_list;

#[derive(Debug, Default)]
pub struct App {
    dir_div_curr: dir_list::DirList,
    dir_div_prev: dir_list::DirList,
    dir_div_next: dir_list::DirList,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("QuickDir".bold());
        let instructions = Line::from(vec![
            "SAUCISSE".into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let count_tx = Text::from(vec![Line::from(vec![
            "pos ".into(),
        ])]);

        Paragraph::new(count_tx)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.dir_div_curr.new(env::current_dir().unwrap().to_str().unwrap());
        self.dir_div_prev.new(env::current_dir().unwrap().parent().unwrap().to_str().unwrap());
        self.dir_div_next.new(&self.dir_div_curr.get_next_dir());

        self.dir_div_curr.has_highlight = true;

        while !self.exit {
            term.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let main_chunks = Layout::vertical([
            Constraint::Min(95),
            Constraint::Min(1),
        ]).split(frame.area());

        let chunks = Layout::horizontal([
            Constraint::Min(33),
            Constraint::Min(33),
            Constraint::Min(33),
        ]).split(main_chunks[0]);

        self.dir_div_prev.draw(frame, &chunks[0]);
        self.dir_div_curr.draw(frame, &chunks[1]);
        self.dir_div_next.draw(frame, &chunks[2]);

        let instructions = Paragraph::new("↑↓: scroll | ←→: change dir | SPACE: change current dir | q: quit")
            .centered();
        frame.render_widget(instructions, main_chunks[1]);
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => {
                self.handle_key_event(key_ev)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => {
                self.dir_div_curr.scroll_up();
                self.dir_div_next.change_dir(&self.dir_div_curr.get_next_dir());
            },
            KeyCode::Down => {
                self.dir_div_curr.scroll_down();
                self.dir_div_next.change_dir(&self.dir_div_curr.get_next_dir());
            },
            KeyCode::Right => {
                if !self.dir_div_next.has_issue() {
                    self.dir_div_prev.change_dir(&self.dir_div_curr.curr_dir);
                    self.dir_div_curr.change_dir(&self.dir_div_curr.get_next_dir());
                    self.dir_div_next.change_dir(&self.dir_div_curr.get_next_dir());
                }
            },
            KeyCode::Left => {
                if !self.dir_div_prev.has_issue() {
                    // TEMP
                    let parent_dir = self.dir_div_curr.parent_dir.clone();
                    self.dir_div_curr.change_dir(&parent_dir);
                    self.dir_div_prev.change_dir(&self.dir_div_curr.parent_dir);
                    self.dir_div_next.change_dir(&self.dir_div_curr.get_next_dir());
                }
            },
            KeyCode::PageUp => {
                for _ in 0..10 {
                    self.dir_div_curr.scroll_up();
                }
                self.dir_div_next.change_dir(&self.dir_div_curr.get_next_dir());
            },
            KeyCode::PageDown => {
                for _ in 0..10 {
                    self.dir_div_curr.scroll_down();
                }
                self.dir_div_next.change_dir(&self.dir_div_curr.get_next_dir());
            },
            KeyCode::Char(' ') => {
                write("/tmp/quickdir_out", &self.dir_div_curr.curr_dir).unwrap();
                self.exit();
            },
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}