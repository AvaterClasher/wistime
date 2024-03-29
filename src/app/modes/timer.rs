use chrono::{DateTime, Duration, Local};
use wistime::bricks_text::BricksText;
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use super::{format_duration, render_centered};

pub struct Timer {
    pub size: u16,
    pub style: Style,
    duration: Duration,
    ended_at: Option<DateTime<Local>>,
}

impl Timer {
    pub(crate) fn new(duration: Duration, size: u16, style: Style) -> Self {
        Self {
            duration,
            size,
            style,
            ended_at: Some(Local::now() + duration),
        }
    }

    pub(crate) fn is_paused(&self) -> bool {
        self.ended_at.is_none()
    }

    pub(crate) fn pause(&mut self) {
        if let Some(end_at) = self.ended_at {
            if end_at <= Local::now() {
                self.duration = Duration::zero();
            } else {
                self.duration = end_at - Local::now();
            }
            self.ended_at = None;
        }
    }

    pub(crate) fn resume(&mut self) {
        if self.ended_at.is_none() {
            self.ended_at = Some(Local::now() + self.duration);
        }
    }

    pub(crate) fn remaining_time(&self) -> Duration {
        if let Some(end_at) = self.ended_at {
            let now = Local::now();
            if end_at <= now {
                Duration::zero()
            } else {
                end_at.signed_duration_since(now)
            }
        } else {
            self.duration
        }
    }
}

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let time_str = format_duration(self.remaining_time());
        // println!("{}", time_str);
        let text = BricksText::new(time_str.as_str(), self.size, self.size, self.style);
        render_centered(area, buf, &text);
    }
}