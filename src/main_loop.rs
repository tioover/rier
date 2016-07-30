//! Main rendering loop and frame rate control.
//! use std::sync::mpsc::{channel, Sender, Receiver};
//! use std::thread::spawn;
use std::time::{Duration, Instant};
use std::thread::sleep;
use fps_counter::FPSCounter;

/// Main loop.
pub struct Loop<F: FnMut(Duration) -> Return> {
    last_time: Instant,
    func: F,
    fps: FPSCounter,
}


impl<F: FnMut(Duration) -> Return> Loop<F> {
    /// Creates with loop body function.
    pub fn new(f: F) -> Loop<F> {
        Loop {
            last_time: Instant::now(),
            func: f,
            fps: FPSCounter::new(),
        }
    }

    /// Start the loop.
    pub fn start(mut self) {
        const ONE_SEC: u32 = 1000000000;
        const FRAME_RATE: u32 = 60;
        let one_frame = Duration::new(0, ONE_SEC/FRAME_RATE - 1000);
        self.last_time = Instant::now();
        loop {
            let _fps = self.fps.tick();
            // println!("{}", _fps);
            let now = Instant::now();
            let delta = now - self.last_time;
            self.last_time = now;
            match (self.func)(delta) {
                Return::Exit => break,
                Return::Next => (),
            }
            let delta = Instant::now() - self.last_time;
            if delta < one_frame {
                sleep(one_frame - delta);
            }
        }
    }
}


/// Return value of loop body function.
pub enum Return {
    /// Stop main loop.
    Exit,
    /// Continue.
    Next,
}
