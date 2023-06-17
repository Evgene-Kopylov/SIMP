use macroquad::prelude::*;

use crate::ui::UI;
use crate::{UNIT_COLOR, UNIT_SIZE};

pub struct DarkGrayRectangle {
    pos: Vec2,
    size: f32,
    d: Vec2,
    zoom: f32,
}

impl UI for DarkGrayRectangle {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn zoom(&self) -> f32 {
        self.zoom
    }
    fn d(&self) -> Vec2 {
        self.d
    }
    fn size(&self) -> f32 {
        self.size
    }
    fn unit_color(&self) -> Color {
        UNIT_COLOR
    }
    fn rotation(&self) -> f32 {
        0.
    }
}

impl DarkGrayRectangle {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(
                screen_width() * 0.5 - UNIT_SIZE * 0.5 + 100.,
                screen_height() * 0.5 - UNIT_SIZE * 0.5 + 130.,
            ),
            size: UNIT_SIZE,
            d: Vec2::new(0., 0.),
            zoom: 1.,
        }
    }

    pub fn update(&mut self, dt: f32, d: Vec2, zoom: f32) -> &str {
        self.d = d;
        self.zoom = zoom;
        let mut keycode: &str = "";
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
            keycode = "Left"
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
            keycode = "Right"
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
            keycode = "Up"
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
            keycode = "Down"
        }

        if self.pos.x < 1f32 {
            x_move = 1f32;
        }

        if self.pos.y < 1f32 {
            y_move = 1f32;
        }

        self.pos.x += x_move * dt * self.size;
        self.pos.y += y_move * dt * self.size;
        keycode
    }

    pub fn draw(&self) {
        let visible_pos = self.visible_coords();
        let visible_size = self.visible_size();

        draw_rectangle(
            visible_pos.x,
            visible_pos.y,
            visible_size,
            visible_size,
            self.unit_color(),
        );
    }
}
