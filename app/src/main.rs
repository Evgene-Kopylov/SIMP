use macroquad::prelude::*;
mod camera;
mod dark_grey_rectangle;
mod selectable_unit;
mod selector_frame;
mod settings;
mod ui;

use crate::camera::Camera;
use crate::dark_grey_rectangle::DarkGrayRectangle;
use crate::selectable_unit::SelectableUnit;
use crate::selector_frame::SelectorFrame;
use crate::settings::{GROUND_COLOR, UNIT_COLOR, UNIT_SIZE, UNIT_SPEED};

#[macroquad::main("breakout")]
async fn main() {
    let mut camera = Camera::start();
    let mut dark_gray_rectangle = DarkGrayRectangle::new();
    let texture: Texture2D = load_texture("../assets/path3333.png").await.unwrap();
    let mut selectable_unit = SelectableUnit::new(texture);
    let mut selector_frame = SelectorFrame::new();

    loop {
        clear_background(GROUND_COLOR);

        // --------------------------------
        let dt = get_frame_time();

        let (camera_move, zoom) = camera.update(dt);
        camera.draw_coordination_greed();
        camera.draw_hexagon();

        // --------------------------------

        dark_gray_rectangle.update(dt, camera_move, zoom);
        dark_gray_rectangle.draw();

        // --------------------------------

        let mouse_position: Vec2 = mouse_position().into();
        selectable_unit.update(dt, camera_move, zoom);
        selector_frame.update(mouse_position, &mut selectable_unit);
        selectable_unit.draw();

        // --------------------------------

        next_frame().await
    }
}
