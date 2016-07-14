extern crate rier;
extern crate image;
use rier::sprite::{ Sprite };
use rier::texture::{ Manager, Key };
use rier::camera::{ Camera, Camera2D };

fn main()
{
    let display = rier::utils::build_display("Sprite", (800, 600));
    let manager = Manager::new(&display);
    let tex_key = Key::from("examples/assets/block.png");
    // manager.load(&tex_key);
    let renderer = rier::render::Renderer::new(&display).unwrap();
    let camera = Camera2D::new(&display);
    let rect = rier::Rect { w: 256, h: 256, x: 0, y: 0 };
    // manager.receive();
    let tex = manager.load(&tex_key);
    let sprite = Sprite::new(&tex, rect, 100.0, 100.0);

    rier::Loop::new(&display,
        move |mut target|
        {
            let cam = camera.matrix();
            let transform = sprite.transform.matrix();
            sprite.graphics.render(target, &renderer, &cam, &transform).unwrap();
        }).start();
}
