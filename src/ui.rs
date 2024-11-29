use crossterm::cursor::MoveTo;
use crossterm::style::{Attribute, Color, Print, PrintStyledContent, SetBackgroundColor, Stylize};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};
use crate::globals::BG_MENU;

pub trait Draw {
    fn draw_at(&self, stdout: &mut Stdout, pos_x: u16, pos_y: u16, forced: bool);
}


pub(crate) struct TextBox {
    pub header: String,
    pub header_color: Color,
    pub header_attribute: Attribute,
    pub text: Vec<String>,
    pub text_color: Color,
    pub text_attribute: Attribute,
    pub background_color: Color,
}

pub(crate) struct StyledBox {
    pub width: u16,
    pub height: u16,
    pub color: Color,
}

pub(crate) struct StyledString {
    pub string: String,
    pub text_color: Color,
    pub background_color: Color,
    pub attribute: Attribute,
}

impl Draw for StyledBox {
    fn draw_at(&self, stdout: &mut Stdout, pos_x: u16, pos_y: u16, _forced: bool) {
        stdout.queue(SetBackgroundColor(self.color)).expect("TODO: panic message");
        for iy in 0..self.height {
            stdout.queue(MoveTo(pos_x, pos_y + iy)).expect("TODO: panic message");
            for ix in 0..self.width {
                stdout.queue(MoveTo(ix + pos_x, pos_y + iy)).expect("Idk, I guess it couldnt move the ponter ?");
                stdout.queue(Print(" ")).expect("Idk, I guess it couldnt print the box ?");
            }
        }
        stdout.flush().expect("TODO: panic message");
    }
}

impl Draw for StyledString {
    fn draw_at(&self, stdout: &mut Stdout, pos_x: u16, pos_y: u16, _forced: bool) {
        stdout.queue(MoveTo(pos_x, pos_y)).expect("Idk, I guess it couldnt move the ponter ?");
        stdout.queue(PrintStyledContent(
            self.string.clone()
                .with(self.text_color)
                .attribute(self.attribute))
        ).unwrap();
    }
}

impl Draw for TextBox {
    fn draw_at(&self, stdout: &mut Stdout, pos_x: u16, pos_y: u16, forced: bool) {

        // Draw backound Box first
        if forced {
            let width_text = 2 + self.text.iter().map(|x| x.clone().len()).max().unwrap_or(16) as u16;
            let width_header = self.header.len() as u16;
            let width = 2 + vec![width_text, width_header].iter().max().unwrap();

            let height = 4 + self.text.len() as u16;
            let styled_box = StyledBox { width, height, color: self.background_color };
            styled_box.draw_at(stdout, pos_x, pos_y, false);
        }

        // Draw text header then (if any)
        stdout.queue(MoveTo(pos_x + 1, pos_y + 1)).expect("Idk, I guess it couldnt move the ponter ?");
        stdout.queue(PrintStyledContent(self.header.clone().with(self.header_color).on(BG_MENU).attribute(self.header_attribute))
        ).expect("Idk, I guess it couldnt print the header ?");

        // Then draws text content
        let mut iy = 3;
        for text_str in self.text.iter() {
            stdout.queue(MoveTo(pos_x + 1, pos_y + iy)).expect("Idk, I guess it couldnt move the ponter ?");
            stdout.queue(PrintStyledContent(
                text_str.clone()
                    .with(self.text_color)
                    .on(BG_MENU)
                    .attribute(self.text_attribute)))
                .expect("Idk, I guess it couldnt print the line ?");
            iy += 1;
        }
    }
}