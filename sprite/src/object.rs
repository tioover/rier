use rier::{Mat, Id, Transform, texture};
use rier::render::{Frame, DrawError, Renderer};
use graphics::Graphics;


/// Sprite game object.
pub struct Sprite {
    pub id: Id,
    pub graphics: Graphics,
    pub transform: Transform,
}


impl Sprite {
    pub fn new(tex: &texture::Ref, rect: texture::Rect, (width, height): (f32, f32)) -> Sprite {
        Sprite {
            id: Id::new(),
            graphics: Graphics::new(tex, rect, width, height),
            transform: Transform::new(),
        }
    }

    pub fn render(&self,
                  target: &mut Frame,
                  renderer: &Renderer<Graphics>,
                  camera: &Mat)
                  -> Result<(), DrawError> {
        self.graphics.render(target, renderer, camera, self.transform.matrix())
    }
}
