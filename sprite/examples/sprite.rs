extern crate rier;
extern crate sprite;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use rier::texture;
use rier::{Context, WindowEvent, Camera2D};
use rier::loader::Resource;
use rier::event::{Notifier, Return};
use sprite::Sprite;


struct Block {
    sprite: Rc<RefCell<Sprite>>,
}

impl Block {
    fn new(ctx: Context, notifier: &mut Notifier<WindowEvent>) -> Block {
        let texture = texture::Raw::load(&PathBuf::from("examples/block.png"))
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

    fn event_register(&self, notifier: &mut Notifier<WindowEvent>) {
        let weak = Rc::downgrade(&self.sprite);
        notifier.register(move |e| {
            match e {
                &WindowEvent::MouseMoved(x, y) => {
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
    let mut camera = Camera2D::new(ctx.clone());
    let block = Block::new(ctx.clone(), &mut notifier);
    'main: loop {
        let (_, h) = ctx.display.get_framebuffer_dimensions();
        camera.update();

        for event in ctx.display.poll_events() {
            match event {
                WindowEvent::Closed => break 'main,
                WindowEvent::MouseMoved(x, y) =>
                    notifier.notify(WindowEvent::MouseMoved(x, h as i32 - y)),
                e => notifier.notify(e),
            }
        }

        ctx.draw(|mut target| {
            block.sprite.borrow().render(&mut target, &renderer, &camera).unwrap();

        });


    }
}
