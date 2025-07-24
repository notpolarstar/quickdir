use ratatui::{layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Paragraph, Widget}, Frame};
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct DirList {
    pub has_highlight: bool,
    pub show_hidden: bool,
    cursor_pos: usize,
    pub parent_dir: String,
    pub curr_dir: String,
    dir_entries_len: usize,
    files: Option<Vec<String>>,
}

impl Widget for &DirList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from(self.curr_dir.as_str().bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);

        let window_size = area.height as usize - 2;
        let start = if self.cursor_pos >= window_size / 2 {
            self.cursor_pos - window_size / 2
        } else {
            0
        };
        let files_lines: Vec<Line> = match &self.files {
            Some(files) => {
                let end = usize::min(start + window_size, files.len());
                files.iter()
                    .enumerate()
                    .skip(start)
                    .take(end - start)
                    .map(|(i, file)| {
                        let idx = i + start;
                        if self.has_highlight && idx == self.cursor_pos + start {
                            Line::from(file.as_str().reversed())
                        } else if file.starts_with(".") {
                            Line::from(file.as_str().dark_gray())
                        } else {
                            Line::from(file.as_str())
                        }
                    })
                    .collect()
            }
            None => vec![Line::from("Failed to read directory")],
        };

        Paragraph::new(files_lines)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl DirList {
    pub fn new(&mut self, path: &str) {
        self.change_dir(path);
    }

    pub fn change_dir(&mut self, path: &str) {
        self.parent_dir = match fs::canonicalize(path) {
            Ok(canon_path) => {
                match canon_path.parent() {
                    Some(parent_path) => parent_path.to_string_lossy().into(),
                    None => String::new(),
                }
            },
            Err(_) => String::new(),
        };
        self.cursor_pos = 0;
        self.curr_dir = path.to_string();
        match fs::read_dir(Path::new(path)) {
            Ok(read_dir) => {
                let mut files_vec: Vec<String> = read_dir
                    .filter_map(|entry| entry.ok())
                    .filter(|e| e.metadata().ok().map(|m| m.is_dir()).unwrap_or(false))
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect();
                files_vec.sort();
                self.dir_entries_len = files_vec.len();
                self.files = Some(files_vec);
            }
            Err(_) => {
                self.dir_entries_len = 0;
                self.files = None;
            }
        }
    }

    pub fn has_issue(&self) -> bool {
        self.curr_dir == ""
    }

    pub fn get_next_dir(&self) -> String {
        match &self.files {
            Some(files) => {
                if let Some(entry) = files.get(self.cursor_pos) {
                    fs::canonicalize(self.curr_dir.clone()).ok().unwrap().join(entry).to_string_lossy().into()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, rect: &Rect) {
        frame.render_widget(self, *rect);
    }

    pub fn is_dir_empty(&self) -> bool {
        self.files != None
    }

    pub fn scroll_up(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.dir_entries_len > 1 && self.cursor_pos < self.dir_entries_len - 1 {
            self.cursor_pos += 1;
        }
    }
}
