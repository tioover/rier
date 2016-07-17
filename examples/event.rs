extern crate rier;
extern crate glium;
use glium::glutin;
use rier::utils::{sleep_ms, build_display};
use rier::event::{Notifier, Return};


fn main()
{
    let display = build_display("Event", (800, 600));

    let mut notifier = Notifier::new();

    // register callback
    notifier.register(|e| {
        println!("{:?}", e);
        Return::None
    });

    'main: loop {

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                e => notifier.notify(e),
            }
        }
        sleep_ms(4);
    }
}
