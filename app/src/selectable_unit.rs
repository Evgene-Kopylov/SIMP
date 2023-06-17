use crate::settings::UNIT_ROTATION_SPEED;
use crate::ui::{UnitWithTexture, UI};
use crate::{UNIT_COLOR, UNIT_SIZE, UNIT_SPEED};
use macroquad::prelude::*;

pub(crate) struct SelectableUnit {
    pub(crate) pos: Vec2,
    rotation: f32,
    order: Vec<Vec2>,
    pub(crate) selected: bool,
    texture: Texture2D,
    pub(crate) d: Vec2,
    pub(crate) zoom: f32,
    size: f32,
    dt: f32,
}

impl UI for SelectableUnit {
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
        self.rotation
    }
}

impl UnitWithTexture for SelectableUnit {
    fn texture(&self) -> Texture2D {
        self.texture
    }
}

impl SelectableUnit {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            pos: Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
            zoom: 1.,
            d: Vec2::new(0., 0.),
            size: UNIT_SIZE,
            texture,
            rotation: f32::to_radians(90.0),
            order: Vec::new(),
            selected: false,
            dt: get_frame_time(),
        }
    }

    pub fn update(&mut self, dt: f32, d: Vec2, zoom: f32) {
        self.dt = dt;
        self.d = d;
        self.zoom = zoom;
        // указание цели мышкой
        if self.selected && is_mouse_button_released(MouseButton::Right) {
            if !is_key_down(KeyCode::LeftShift) && !is_key_down(KeyCode::LeftControl) {
                self.order.clear();
            }
            self.order.push(
                (
                    (mouse_position().0 - self.d.x) / self.zoom,
                    (mouse_position().1 - self.d.y) / self.zoom,
                )
                    .into(),
            );
        }

        let mut y_move = -1f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
        }

        // поворот юнита в сторону курсора
        if self.order.len() > 0 {
            self.rotation = self.rotation % f32::to_radians(360.);
            let mut dx = self.pos.x - self.order[0].x;
            if dx == 0f32 {
                dx += 1f32;
            };

            let mut dy = self.pos.y - self.order[0].y;
            if dy == 0f32 {
                dy += 1f32;
            };

            let a;
            if dx >= 0f32 {
                a = (dy / dx).atan();
            } else {
                a = (dy / dx).atan() - f32::to_radians(180.);
            }

            // останавливаться перед целью
            if dx.powf(2f32) + dy.powf(2f32) < (UNIT_SIZE / 2.).powf(2f32) {
                y_move = 0f32;
                self.order.remove(0);
            }
            let mut da = self.rotation - a;
            if da <= f32::to_radians(-180.) {
                da += f32::to_radians(360.)
            }
            if da > f32::to_radians(180.) {
                da -= f32::to_radians(360.)
            }
            // сохранение направления движения
            if da.abs() > f32::to_radians(9.) {
                if da > 0. {
                    self.rotation -= dt * UNIT_ROTATION_SPEED
                } else {
                    self.rotation += dt * UNIT_ROTATION_SPEED
                }
            }

            self.pos.x += y_move * dt * UNIT_SPEED * self.rotation.cos();
            self.pos.y += y_move * dt * UNIT_SPEED * self.rotation.sin();
        }
    }

    pub fn draw_path(&self) {
        let dt = self.dt;
        let mut eta: f32 = 0.0;
        for i in 0..self.order.len() {
            let x1;
            let y1;
            if i == 0 {
                x1 = self.pos.x;
                y1 = self.pos.y;
            } else {
                x1 = self.order[i - 1].x;
                y1 = self.order[i - 1].y;
            }
            let x2 = self.order[i].x;
            let y2 = self.order[i].y;
            draw_line(
                x1 * self.zoom + self.d.x,
                y1 * self.zoom + self.d.y,
                x2 * self.zoom + self.d.x,
                y2 * self.zoom + self.d.y,
                1f32,
                BLUE,
            );
            let dx = x1 - self.order[i].x;
            let dy = y1 - self.order[i].y;

            eta += (dx.powf(2.) + dy.powf(2.)).sqrt() * dt / UNIT_SPEED * 200.0;
            draw_text(
                format!("ETA: {:.1}", eta).as_str(),
                self.order[i].x * self.zoom + self.d.x - 12.,
                self.order[i].y * self.zoom + self.d.y - 5.,
                18.,
                BLACK,
            );
        }
    }

    pub fn draw(&self) {
        if self.selected {
            self.draw_path();
        }
        self.draw_texture(0.9);
        if self.selected {
            self.draw_circle_collision();
        }
    }
}
