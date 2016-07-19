extern crate rier;
extern crate glium;
extern crate image;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use glium::Surface;
use glium::glutin::Event;
use rier::texture;
use rier::sprite::Sprite;
use rier::Context;
use rier::camera::{Camera, Camera2D};
use rier::loader::Resource;
use rier::utils::sleep_ms;
use rier::event::{Notifier, Return};


struct Block {
    sprite: Rc<RefCell<Sprite>>,
}

impl Block {
    fn new(ctx: Context, notifier: &mut Notifier<Event>) -> Block {
        let texture = texture::Raw::load(&PathBuf::from("examples/assets/block.png"))
            .unwrap()
            .process(&ctx)
            .unwrap();
        let sprite = Sprite::new(
            &texture::Ref::new(texture),
            texture::Rect { w: 256, h: 256, x: 0, y: 0 },
            (100.0, 100.0));
        let block = Block{ sprite: Rc::new(RefCell::new(sprite)) };
        block.event_register(notifier);
        block
    }

    fn event_register(&self, notifier: &mut Notifier<Event>) {
        let weak = Rc::downgrade(&self.sprite);
        notifier.register(move |e| {
            println!("{:?}", e);
            match e {
                &Event::MouseMoved(x, y) => {
                    match weak.upgrade() {
                        None => Return::Dead,
                        Some(sprite) => {
                            let mut sprite = sprite.borrow_mut();
                            sprite.transform.set_position(x as f32, y as f32, 0.0);
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
    let ctx = Context::create("Sprite", (800, 600));
    let renderer = rier::render::Renderer::new(ctx.clone()).unwrap();
    let mut notifier = Notifier::new();
    let camera = Camera2D::new(ctx.clone());
    let block = Block::new(ctx.clone(), &mut notifier);
    'main: loop {
        let (w, h) = ctx.display.get_framebuffer_dimensions();
    
        for event in ctx.display.poll_events() {
            match event {
                Event::Closed => break 'main,
                Event::MouseMoved(x, y) => notifier.notify(Event::MouseMoved(x, h as i32 - y)),
                e => notifier.notify(e),
            }
        }
        ctx.draw(|mut target| {
            target.clear_color(0., 0., 0., 0.);
            let cam = camera.matrix();
            block.sprite.borrow().render(&mut target, &renderer, &cam).unwrap();
        }).unwrap();
        sleep_ms(4);
    }
}
