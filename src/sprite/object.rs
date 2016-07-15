use Id;
use Transform;
use texture;
use super::graphics::Graphics;


/// Sprite game object.
pub struct Sprite
{
    /// Object id.
    pub id: Id,
    /// Graphics component, for rendering.
    pub graphics: Graphics,
    /// Transform component.
    pub transform: Transform,
}


impl Sprite
{
    pub fn new(tex: &texture::Ref, tex_rect: ::Rect, width: f32, height: f32) -> Sprite
    {
        let id = Id::new();
        let transform = Transform::new();
        let graphics = Graphics::new(tex, tex_rect, width, height);
        Sprite
        {
            id: id,
            transform: transform,
            graphics: graphics,
        }
    }
}
