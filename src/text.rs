use super::*;

#[derive(Clone, Copy)]
pub enum HAlign {
    Left,
    Centre,
    Right,
}

#[derive(Clone, Copy)]
pub enum VAlign {
    Top,
    Centre,
    Bottom,
}

pub struct Text {
    pub text: String,
    pub halign: HAlign,
    pub valign: VAlign,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            halign: HAlign::Left,
            valign: VAlign::Top,
        }
    }

    pub fn wrap(text: &str) -> Box<Self> {
        Box::new(Self::new(text))
    }
}

impl Widget for Text {
    fn render(&mut self, rect: &Rect, _active: bool) {
        let s = self
            .text
            .get(..rect.w.floor() as usize)
            .unwrap_or(&self.text);
        let n = s.len();
        let free_hspace = rect.w - n as f32;
        // TODO(#3): Text does not support wrapping around
        let free_vspace = rect.h - 1.0;

        let window = initscr();
        match self.valign {
            VAlign::Top => {
                window.mv(rect.y as i32, rect.x as i32);
            }
            VAlign::Centre => {
                window.mv((rect.y + free_vspace * 0.5).floor() as i32, rect.x as i32);
            }
            VAlign::Bottom => {
                window.mv((rect.y + free_vspace).floor() as i32, rect.x as i32);
            }
        }

        match self.halign {
            HAlign::Left => {
                window.addstr(s);
            }
            HAlign::Centre => {
                let padding = (free_hspace * 0.5).floor() as usize;
                for _ in 0..padding {
                    window.addstr(" ");
                }
                window.addstr(s);
                for _ in 0..padding {
                    window.addstr(" ");
                }
            }
            HAlign::Right => {
                let padding = free_hspace.floor() as usize;
                for _ in 0..padding {
                    window.addstr(" ");
                }
                window.addstr(s);
            }
        }
    }

    fn handle_event(&mut self, _event: &Event) {}
}
