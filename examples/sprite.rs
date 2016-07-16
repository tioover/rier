extern crate rier;
extern crate image;
use std::path::PathBuf;
use rier::texture;
use rier::sprite::Sprite;
use rier::camera::{ Camera, Camera2D };
use rier::loader::Resource;

fn main()
{
    let display = rier::utils::build_display("Sprite", (800, 600));
    let tex_path = PathBuf::from("examples/assets/block.png");
    let renderer = rier::render::Renderer::new(&display).unwrap();
    let camera = Camera2D::new(&display);
    let sprite =
    {
        let texture = texture::Raw::load(&tex_path)
            .unwrap()
            .process(&display)
            .unwrap();

        Sprite::new(
            &texture::Ref::new(texture),
            texture::Rect { w: 256, h: 256, x: 0, y: 0 },
            (100.0, 100.0))
    };

    rier::Loop::new(&display, move |mut target|
    {
        let cam = camera.matrix();
        sprite.render(target, &renderer, &cam).unwrap();
    }).start();
}
