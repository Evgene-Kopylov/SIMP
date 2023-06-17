use crate::selectable_unit::SelectableUnit;
use crate::settings::{SELECTOR_COLOR, UNIT_SIZE};
use macroquad::prelude::{
    draw_rectangle, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
    mouse_position, Color, MouseButton, Vec2,
};

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
            if (unit.pos.x * unit.zoom + unit.d.x) < self.point1.x.max(self.point2.x)
                && (unit.pos.x * unit.zoom + unit.d.x) > self.point1.x.min(self.point2.x)
                && (unit.pos.y * unit.zoom + unit.d.y) < self.point1.y.max(self.point2.y)
                && (unit.pos.y * unit.zoom + unit.d.y) > self.point1.y.min(self.point2.y)
            {
                unit.selected = true;
            }
        }

        // одиночный клик
        if is_mouse_button_pressed(MouseButton::Left) {
            if (mouse_position.x - (unit.pos.x * unit.zoom + unit.d.x)).powf(2f32)
                + (mouse_position.y - (unit.pos.y * unit.zoom + unit.d.y)).powf(2f32)
                < (UNIT_SIZE * unit.zoom / 2.).powf(2f32)
            {
                unit.selected = true;
            }
        }
    }
}
