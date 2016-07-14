use glium::glutin::Event;
use glium::{ Display, Surface, Frame };


/// Main rendering loop.
pub struct Loop<F: FnMut(&mut Frame)>
{
    display: Display,
    body: F,
    clear_color: (f32, f32, f32, f32),
}

impl<F: FnMut(&mut Frame)> Loop<F>
{
    pub fn new(display: &Display, body: F) -> Loop<F>
    {
        Loop
        {
            display: display.clone(),
            body: body,
            clear_color: (0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Running the main loop.
    pub fn start(&mut self)
    {
        use utils::sleep_ms;

        loop {
            let mut target = self.display.draw();
            {
                let (r, g, b, a) = self.clear_color;
                target.clear_color(r, g, b, a);
            }
            (self.body)(&mut target);
            target.finish().unwrap();
            for event in self.display.poll_events()
            {
                match event
                {
                    Event::Closed => return,
                    _ => (),
                }
            }
            sleep_ms(4);
        }
    }
}
