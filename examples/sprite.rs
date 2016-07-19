extern crate rier;
extern crate glium;
extern crate image;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use glium::{Surface, Display, glutin};
use rier::texture;
use rier::sprite::Sprite;
use rier::camera::{Camera, Camera2D};
use rier::loader::Resource;
use rier::utils::{sleep_ms, build_display};
use rier::event::{Notifier, Return};


struct Block {
    sprite: Rc<RefCell<Sprite>>,
}

impl Block {
    fn new(display: &Display, notifier: &mut Notifier<glutin::Event>) -> Block {
        let texture = texture::Raw::load(&PathBuf::from("examples/assets/block.png"))
            .unwrap()
            .process(display)
            .unwrap();
        let sprite = Sprite::new(
            &texture::Ref::new(texture),
            texture::Rect { w: 256, h: 256, x: 0, y: 0 },
            (100.0, 100.0));
        let block = Block{ sprite: Rc::new(RefCell::new(sprite)) };
        block.event_register(notifier);
        block
    }

    fn event_register(&self, notifier: &mut Notifier<glutin::Event>) {
        let weak = Rc::downgrade(&self.sprite);
        notifier.register(move |e| {
            match e {
                &glutin::Event::MouseMoved(x, y) => {
                    // TODO: Get real HiDPI factor, not `2.0`.
                    let (x, y) = (x as f32 / 2.0, y as f32 / 2.0);
                    match weak.upgrade() {
                        None => Return::Dead,
                        Some(sprite) => {
                            let mut sprite = sprite.borrow_mut();
                            sprite.transform.set_position(x, y, 0.0);
                            Return::None
                        }
                    }
                },
                _ => Return::None,
            }
        })
    }
}


fn main()
{
    let display = build_display("Sprite", (800, 600));
    let renderer = rier::render::Renderer::new(&display).unwrap();
    let mut notifier = Notifier::new();
    let camera = Camera2D::new(&display);
    let block = Block::new(&display, &mut notifier);
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
        block.sprite.borrow().render(&mut target, &renderer, &cam).unwrap();
        target.finish().unwrap();
        sleep_ms(4);
    }
}
