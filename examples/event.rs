extern crate rier;
extern crate glium;
use rier::utils::sleep_ms;
use rier::event::{Notifier, Return, WindowEvent};


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
                WindowEvent::Closed => break 'main,
                e => notifier.notify(e),
            }
        }
        sleep_ms(4);
    }
}
