use glium::{Frame, DrawError};
use Id;
use Mat;
use Transform;
use texture;
use render;
use sprite::graphics::Graphics;


/// Sprite game object.
pub struct Sprite {
    pub id: Id,
    pub graphics: Graphics,
    pub transform: Transform,
}


impl Sprite {
    pub fn new(tex: &texture::Ref, rect: texture::Rect, (width, height): (f32, f32)) -> Sprite {
        let id = Id::new();
        let transform = Transform::new();
        let graphics = Graphics::new(tex, rect, width, height);
        Sprite {
            id: id,
            transform: transform,
            graphics: graphics,
        }
    }

    pub fn render(&self,
                  target: &mut Frame,
                  renderer: &render::Renderer<Graphics>,
                  camera: &Mat)
                  -> Result<(), DrawError> {
        self.graphics.render(target, renderer, camera, self.transform.matrix())
    }
}
