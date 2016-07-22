extern crate rier;
extern crate glium;
use rier::event::{Notifier, Return};
use rier::{Gfx, Context};


fn main()
{
    let gfx = Context::create("Event", (800, 600)).gfx();

    let mut notifier = Notifier::new();

    // register callback
    notifier.register(|e| {
        println!("{:?}", e);
        Return::None
    });

    'main: loop {

        for event in gfx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => break 'main,
                e => notifier.notify(e),
            }
        }
    }
}
