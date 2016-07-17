extern crate rier;
extern crate glium;
extern crate image;
use std::path::PathBuf;
use glium::Surface;
use glium::glutin;
use rier::texture;
use rier::sprite::Sprite;
use rier::camera::{Camera, Camera2D};
use rier::loader::Resource;
use rier::utils::sleep_ms;
use rier::event::{Notifier, Return};


fn main()
{
    let display = rier::utils::build_display("Sprite", (800, 600));
    let tex_path = PathBuf::from("examples/assets/block.png");
    let renderer = rier::render::Renderer::new(&display).unwrap();

    let mut notifier = Notifier::new();
    // register callback
    notifier.register(|e| {
        println!("{:?}", e);
        Return::None
    });

    let camera = Camera2D::new(&display);
    let sprite = {
        let texture = texture::Raw::load(&tex_path)
            .unwrap()
            .process(&display)
            .unwrap();

        Sprite::new(
            &texture::Ref::new(texture),
            texture::Rect { w: 256, h: 256, x: 0, y: 0 },
            (100.0, 100.0))
    };

    'main: loop {

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                e => notifier.notify(e),
            }
        }
        let mut target = display.draw();
        target.clear_color(0., 0., 0., 0.);
        let cam = camera.matrix();
        sprite.render(&mut target, &renderer, &cam).unwrap();
        target.finish().unwrap();
        sleep_ms(4);
    }
}
