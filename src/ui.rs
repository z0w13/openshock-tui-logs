use jiff::{Timestamp, Unit};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Cell, Padding, Row, Table, Widget},
};

use crate::api::{LogEntry, LogResponse};

#[derive(Debug)]
pub(crate) struct ViewModel {
    pub(crate) log: Vec<LogEntry>,
    pub(crate) last_updated: Timestamp,
    pub(crate) running_state: RunningState,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RunningState {
    Running,
    Done,
}

#[derive(Debug)]
pub(crate) enum Message {
    UpdateLog(LogResponse),
    Quit,
}

pub(crate) fn update(model: ViewModel, msg: Message) -> ViewModel {
    match msg {
        Message::UpdateLog(resp) => ViewModel {
            log: resp.logs,
            last_updated: Timestamp::now(),

            ..model
        },
        Message::Quit => ViewModel {
            running_state: RunningState::Done,
            ..model
        },
    }
}

pub(crate) fn view(model: &ViewModel, frame: &mut Frame) {
    frame.render_widget(AppWidget::new(model), frame.area());
}

struct AppWidget<'a> {
    model: &'a ViewModel,
}

impl<'a> AppWidget<'a> {
    fn new(model: &'a ViewModel) -> Self {
        Self { model }
    }
}

impl<'a> Widget for AppWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let log_block = Block::bordered()
            .border_type(BorderType::Double)
            .title_top(Line::from(" Shock Log "))
            .padding(Padding::horizontal(1))
            .title_bottom(
                Line::from(format!(
                    " Last Updated: {} ",
                    self.model.last_updated.strftime("%H:%M:%S")
                ))
                .right_aligned(),
            );

        LogWidget::new(&self.model.log).render(log_block.inner(area), buf);
        log_block.render(area, buf);
    }
}

struct LogWidget<'a> {
    model: &'a [LogEntry],
}

impl<'a> LogWidget<'a> {
    fn new(model: &'a [LogEntry]) -> Self {
        Self { model }
    }
}

impl<'a> Widget for LogWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let headers = ["Time", "Shocker", "Type", "Intensity", "Duration", "By"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::new().bold())
            .height(1);

        let shocker_name_max_len = self
            .model
            .iter()
            .take(area.height as usize)
            .map(|l| l.shocker_name.len())
            .max()
            .unwrap_or(0)
            .max(7);

        let rows = self
            .model
            .iter()
            .take(area.height as usize)
            .enumerate()
            .map(|(i, entry)| {
                let color = match i % 2 {
                    0 => Color::Rgb(10, 10, 10),
                    _ => Color::Reset,
                };

                [
                    Text::from(format_time_relative(entry.created_on)).right_aligned(),
                    Text::from(entry.shocker_name.as_str()),
                    Text::from(entry.typ.as_str()),
                    Text::from(entry.intensity.to_string())
                        .fg(intensity_color(entry.intensity))
                        .right_aligned(),
                    Text::from(format_duration(entry.duration)).right_aligned(),
                    Text::from(entry.controlled_by.name.as_str()),
                ]
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .style(Style::new().bg(color))
                .height(1)
            });

        Table::new(
            rows,
            [
                Constraint::Length(8),
                Constraint::Length(shocker_name_max_len as u16),
                Constraint::Length(9),
                Constraint::Length(6),
                Constraint::Length(8),
                Constraint::Fill(100),
            ],
        )
        .header(headers)
        .render(area, buf);
    }
}

fn format_time_relative(time: Timestamp) -> String {
    if let Ok(secs) = Timestamp::now()
        .since(time)
        .and_then(|span| span.total(Unit::Second))
        && secs < 10.0
    {
        return format!("{:.0}s ago", secs);
    }

    return time.strftime("%H:%M:%S").to_string();
}

fn format_duration(duration: u32) -> String {
    format!("{:.1}s", duration as f64 / 1000.0)
}

/// converts a percentage to an RGB gradient from red -> green
pub(crate) fn intensity_color(pct_health: u32) -> Color {
    let pct_health = (pct_health as f64).clamp(0., 100.);

    Color::Rgb(
        (pct_health * 2.55) as u8,
        ((100.0 - pct_health) * 2.55) as u8,
        0,
    )
}
