use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{prelude::*, Frame};

use super::Component;

pub struct Input {
    pub title: String,
    pub input: String,
    pub selected_button: Option<Button>, // Добавляем состояние выбранной кнопки
    pub visible: bool,
}

// Для управления состоянием кнопок создадим перечисление
enum Button {
    Yes,
    No,
}

impl Input {
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => {
                self.selected_button = Some(Button::Yes);
            }
            KeyCode::Right => {
                self.selected_button = Some(Button::No);
            }
            KeyCode::Enter => match self.selected_button {
                Some(Button::Yes) => {
                    println!("Yes selected!");
                    self.hide(); // Скрываем попап после выбора
                }
                Some(Button::No) => {
                    println!("No selected!");
                    self.hide(); // Скрываем попап после выбора
                }
                None => {}
            },
            KeyCode::Esc => {
                self.hide(); // Добавим возможность закрыть попап клавишей Esc
            }
            KeyCode::Char('p') => {
                self.show(); // Добавим возможность открыть попап клавишей 'p'
            }
            _ => {}
        }
    }

    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            input: String::new(),
            visible: false, // Инициализация видимости как false
            selected_button: Some(Button::Yes),
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}

impl Component for Input {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let title_str: &str = &self.title;

        let block = Block::default()
            .title(title_str)
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::ROUNDED);
        f.render_widget(block, area);

        // Определим область для текста и кнопок
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Область для текста
                Constraint::Length(1), // Область для кнопок
            ])
            .split(area);

        // Отрисовка текста
        // let text =
        //     Paragraph::new(self.input.as_ref()).block(Block::default().borders(Borders::NONE));
        // f.render_widget(text, chunks[0]);

        // Отрисовка кнопок
        let buttons = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let yes_button = Block::default()
            .title("Yes")
            .borders(Borders::ALL)
            .border_style(match self.selected_button {
                Some(Button::Yes) => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            });
        f.render_widget(yes_button, buttons[0]);

        let no_button = Block::default()
            .title("No")
            .borders(Borders::ALL)
            .border_style(match self.selected_button {
                Some(Button::No) => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            });
        f.render_widget(no_button, buttons[1]);

        Ok(())
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
