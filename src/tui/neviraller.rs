use color_eyre::{eyre::WrapErr, Result};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use super::tui::Tui;

#[derive(Debug)]
pub struct Dep {
    name: String,
    command: String,
    state: bool,
}

#[derive(Debug, Default)]
pub struct Neviraller {
    dependencies: Vec<Dep>,
    exit: bool,
}

impl Neviraller {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }
}

impl Widget for &Neviraller {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" NEVIRALLER ".green().bold());
        let instructions = Title::from(Line::from(vec![
            " Up ".into(),
            "<K>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let container_block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        // Рендерим контейнерный блок
        container_block.clone().render(area, buf);

        // Получаем внутреннюю область контейнерного блока
        let inner_area = container_block.inner(area);

        // Разделяем внутреннюю область на верхнюю и нижнюю части
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Высота для табов
                Constraint::Min(0),    // Оставшееся пространство для колонок
            ])
            .split(inner_area);

        // Рендерим табы в верхней части
        Tabs::new(vec!["Auto", "Manual", "Info", "Logs"])
            .style(Style::default().white())
            .highlight_style(Style::default().green())
            .select(1)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(vertical_chunks[0], buf);

        // let items = ["Item 1", "Item 2", "Item 3"];
        // let list = List::new(items)
        //     .block(Block::default().title("List").borders(Borders::ALL))
        //     .style(Style::default().fg(Color::White))
        //     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        //     .highlight_symbol(">>")
        //     .repeat_highlight_symbol(true)
        //     .direction(ListDirection::BottomToTop);
        // frame.render_widget(list, area);

        // Разделяем нижнюю часть на две колонки
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 50% ширины для первой колонки
                Constraint::Percentage(70), // 50% ширины для второй колонки
            ])
            .split(vertical_chunks[1]);

        // Создаём и отрисовываем блоки в каждой из колонок
        let left_block = Block::default();
        left_block.render(horizontal_chunks[0], buf);

        let right_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Preview ");
        right_block.render(horizontal_chunks[1], buf);
    }
}
