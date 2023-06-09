use macroquad::prelude::*;
use crate::settings::{SELECTOR_COLOR, UNIT_ROTATION_SPEED};
use crate::{UNIT_COLOR, UNIT_SIZE, UNIT_SPEED};

pub(crate) struct SelectableUnit {
    collision: Circle,
    rotation: f32,
    order: Vec<Vec2>,
    pub(crate) selected: bool,
    texture: Texture2D,
    d: Vec2,
    zoom: f32,
}

pub(crate) struct SelectorFrame {
    point1: Vec2,
    point2: Vec2,
    color: Color,
}

impl SelectorFrame {
    pub fn new() -> Self {
        let mouse_position = mouse_position().into();
        let mut color = SELECTOR_COLOR;
        color.a = 0.14;
        Self {
            point1: mouse_position,
            point2: mouse_position,
            color,
        }
    }

    pub fn update(&mut self, mouse_position: Vec2, unit: &mut SelectableUnit) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.point1 = mouse_position;
            unit.selected = false;
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.point2 = mouse_position;

            draw_rectangle(
                self.point1.x,
                self.point1.y,
                self.point2.x - self.point1.x,
                self.point2.y - self.point1.y,
                self.color,
            );
        }

        // выделение области
        if is_mouse_button_released(MouseButton::Left) {
            if
            (unit.collision.x * unit.zoom + unit.d.x) < self.point1.x.max(self.point2.x) &&
                (unit.collision.x * unit.zoom + unit.d.x) > self.point1.x.min(self.point2.x) &&
                (unit.collision.y * unit.zoom + unit.d.y) < self.point1.y.max(self.point2.y) &&
                (unit.collision.y * unit.zoom + unit.d.y) > self.point1.y.min(self.point2.y)
            {
                unit.selected = true;
            }
        }

        // одиночный клик
        if is_mouse_button_pressed(MouseButton::Left) {
            if
            (mouse_position.x - (unit.collision.x * unit.zoom + unit.d.x)).powf(2f32) +
                (mouse_position.y - (unit.collision.y * unit.zoom + unit.d.y)).powf(2f32) < (UNIT_SIZE * unit.zoom / 2.).powf(2f32)
            {
                unit.selected = true;
            }
        }

    }

}

impl SelectableUnit {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            collision: Circle::new(
                screen_width() * 0.5,
                screen_height() * 0.5,
                UNIT_SIZE / 2.
            ),
            rotation: f32::to_radians(90.0),
            order: Vec::new(),
            selected: false,
            texture,
            d: Vec2::new(0., 0.),
            zoom: 1.,
        }
    }

    pub fn update(&mut self, dt: f32, d: Vec2, zoom: f32) {
        self.d = d;
        self.zoom = zoom;
        // указание цели мышкой
        if self.selected && is_mouse_button_released(MouseButton::Right) {
            if !is_key_down(KeyCode::LeftShift) && !is_key_down(KeyCode::LeftControl) {
                self.order.clear();
            }
            self.order.push((
                (mouse_position().0 - self.d.x) / self.zoom,
                (mouse_position().1 - self.d.y) / self.zoom,
            ).into());
        }

        let mut y_move = -1f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
        }

        // // отталкиваться от краев карты
        // if self.collision.y < 1f32 {
        //     self.collision.y += 1f32;
        // }
        // if self.collision.y > screen_height() - UNIT_SIZE {
        //     self.collision.y -= 1f32;
        // }
        // if self.collision.x < 1f32 {
        //     self.collision.x += 1f32;
        // }
        // if self.collision.x > screen_width() - UNIT_SIZE {
        //     self.collision.x -= 1f32;
        // }

        // поворот юнита в сторону курсора
        if self.order.len() > 0 {
            self.rotation = self.rotation % f32::to_radians(360.);
            let mut dx = self.collision.x - self.order[0].x;
            if dx == 0f32 { dx += 1f32; };

            let mut dy = self.collision.y - self.order[0].y;
            if dy == 0f32 { dy += 1f32; };

            let a;
            if dx >= 0f32 {
                a = (dy / dx).atan(); }
            else {
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

            self.collision.x += y_move * dt * UNIT_SPEED * self.rotation.cos();
            self.collision.y += y_move * dt * UNIT_SPEED * self.rotation.sin();
        }
    }

    pub fn draw_collision(&self) {
        draw_circle_lines(
            self.collision.x * self.zoom + self.d.x,
            self.collision.y * self.zoom + self.d.y,
            self.collision.r * self.zoom,
            1.,
            BLUE
        )
    }

    pub fn draw_path(&self, dt: f32) {
        let mut eta: f32 = 0.0;
        for i in 0..self.order.len() {
            let x1;
            let y1;
            if i == 0 {
                x1 = self.collision.x;
                y1 = self.collision.y;
            } else {
                x1 = self.order[i-1].x;
                y1 = self.order[i-1].y;
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
                BLACK
            );
        }
    }

    pub fn draw(&self) {
        let d = 0.8; // соотношение сторон
        draw_texture_ex(
            self.texture,
            (self.collision.x - UNIT_SIZE * d * 0.5) * self.zoom + self.d.x,
            (self.collision.y - UNIT_SIZE * 0.5) * self.zoom + self.d.y,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new((
                                  UNIT_SIZE * d) * self.zoom,
                               UNIT_SIZE * self.zoom)),
                rotation: self.rotation - f32::to_radians(90.),
                ..Default::default()
            }
        );
    }
}

