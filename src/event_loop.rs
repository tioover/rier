use glium::{ Display, Surface, Frame };
use event::{ WindowEvent, SubscriberList };


/// Main rendering and event processing loop.
pub struct Loop<F: FnMut(&mut Frame)>
{
    display: Display,
    loop_body: F,
    clear_color: (f32, f32, f32, f32),
    subscribers: SubscriberList<WindowEvent>,
}

impl<F: FnMut(&mut Frame)> Loop<F>
{
    pub fn new(display: &Display, body: F) -> Loop<F>
    {
        Loop
        {
            display: display.clone(),
            loop_body: body,
            clear_color: (0.0, 0.0, 0.0, 0.0),
            subscribers: SubscriberList::new(),
        }
    }

    fn clear(&self, target: &mut Frame)
    {
        let (r, g, b, a) = self.clear_color;
        target.clear_color(r, g, b, a);
    }

    /// Running the main loop.
    pub fn start(&mut self)
    {
        use utils::sleep_ms;

        loop {
            let mut target = self.display.draw();
            self.clear(&mut target);
            (self.loop_body)(&mut target);
            target.finish().unwrap();
            for event in self.display.poll_events()
            {
                match event
                {
                    WindowEvent::Closed => return,
                    e => self.subscribers.notify(e),
                }
            }
            sleep_ms(4);
        }
    }
}
