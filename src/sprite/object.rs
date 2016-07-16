use Id;
use Transform;
use texture;
use super::graphics::Graphics;


/// Sprite game object.
pub struct Sprite
{
    pub id: Id,
    pub graphics: Graphics,
    pub transform: Transform,
}


impl Sprite
{
    pub fn new(tex: &texture::Ref, rect: texture::Rect, (width, height): (f32, f32)) -> Sprite
    {
        let id = Id::new();
        let transform = Transform::new();
        let graphics = Graphics::new(tex, rect, width, height);
        Sprite
        {
            id: id,
            transform: transform,
            graphics: graphics,
        }
    }
}
