extern crate rier;
extern crate glium;
use rier::event::{Notifier, Return};


fn main()
{
    let ctx = rier::Context::create("Event", (800, 600));

    let mut notifier = Notifier::new();

    // register callback
    notifier.register(|e| {
        println!("{:?}", e);
        Return::None
    });

    'main: loop {

        for event in ctx.display.poll_events() {
            match event {
                rier::WindowEvent::Closed => break 'main,
                e => notifier.notify(e),
            }
        }
    }
}
