use macroquad::prelude::{
    draw_circle_lines, draw_texture_ex, Color, DrawTextureParams, Texture2D, Vec2, BLUE,
};

pub trait UI {
    fn pos(&self) -> Vec2;
    fn zoom(&self) -> f32;
    fn d(&self) -> Vec2;
    fn size(&self) -> f32;
    fn unit_color(&self) -> Color;
    fn rotation(&self) -> f32;

    fn visible_coords(&self) -> Vec2 {
        Vec2::new(
            self.pos().x * self.zoom() + self.d().x,
            self.pos().y * self.zoom() + self.d().y,
        )
    }

    fn visible_size(&self) -> f32 {
        self.size() * self.zoom()
    }

    fn draw_circle_collision(&self) {
        draw_circle_lines(
            self.pos().x * self.zoom() + self.d().x,
            self.pos().y * self.zoom() + self.d().y,
            self.size() / 2. * self.zoom(),
            1.,
            BLUE,
        )
    }
}

pub trait UnitWithTexture: UI {
    fn texture(&self) -> Texture2D;

    /// deform - соотношение сторон - ширина к длинне
    fn draw_texture(&self, deform: f32) {
        let d = deform;
        draw_texture_ex(
            self.texture(),
            (self.pos().x - self.size() * d * 0.5) * self.zoom() + self.d().x,
            (self.pos().y - self.size() * 0.5) * self.zoom() + self.d().y,
            self.unit_color(),
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    (self.size() * d) * self.zoom(),
                    self.size() * self.zoom(),
                )),
                rotation: self.rotation() - f32::to_radians(90.),
                ..Default::default()
            },
        );
    }
}
