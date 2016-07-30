//! Manage OpenGL context and window.
use std::string::ToString;
use std::rc::Rc;
use std::ops::Deref;
use std::cell::{UnsafeCell, RefCell, RefMut, Ref};
use glium::{Display, DisplayBuild};
use glium::glutin::WindowBuilder;

pub use glium::{Frame, Surface, SwapBuffersError, DrawError};


/// Just a reference of `Context`.
#[derive(Clone)]
pub struct Gfx(Rc<Context>);


impl Gfx {
    pub fn new(ctx: Context) -> Gfx {
        Gfx(Rc::new(ctx))
    }
}


impl Deref for Gfx {
    type Target = Context;

    fn deref(&self) -> &Context {
        &*self.0
    }
}


/// Context handle object.
///
/// Manage `glium::Display` context and current frame.
pub struct Context {
    pub display: Display,
    frame: UnsafeCell<Option<RefCell<Frame>>>,
    clear_color: (f32, f32, f32, f32),
}


impl Context {
    /// Builds OpenGL context and create a window.
    pub fn create<T: ToString>(title: T, (width, height): (u32, u32)) -> Context {
        let display = WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height)
            .with_depth_buffer(24)
            .with_vsync()
            .build_glium()
            .unwrap();
        Context {
            display: display,
            frame: UnsafeCell::new(None),
            clear_color: (0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Sets clear color.
    pub fn clear_color(self, r: f32, g: f32, b: f32, a: f32) -> Context {
        Context { clear_color: (r, g, b, a), ..self }
    }

    /// Into be a reference.
    pub fn gfx(self) -> Gfx {
        Gfx::new(self)
    }

    unsafe fn get_cell(&self) -> &mut Option<RefCell<Frame>> {
        self.frame.get().as_mut().unwrap()
    }

    /// Start a new frame.
    fn start_frame(&self) {
        unsafe {
            let mut cell = self.get_cell();
            if cell.is_some() {
                println!("Frame has already started.");
            } else {
                let mut frame = self.display.draw();
                frame.clear_color_and_depth(self.clear_color, 1.0);
                *cell = Some(RefCell::new(frame));
            }
        }
    }


    /// Get frame immutable reference.
    /// # Panics
    /// Panic if frame not created or something is mutable borrowing the frame.
    pub fn get_frame(&self) -> Ref<Frame> {
        unsafe {
            let cell = self.get_cell();
            if cell.is_none() {
                panic!("Frame not exist.")
            }
            cell.as_ref().unwrap().borrow()
        }
    }

    /// Get frame mutable reference.
    /// # Panics
    /// Panic if something is borrowing the frame.
    pub fn get_frame_mut(&self) -> RefMut<Frame> {
        unsafe {
            let cell = self.get_cell();
            if cell.is_none() {
                panic!("Frame not exist.")
            }
            cell.as_ref().unwrap().borrow_mut()
        }
    }

    /// End the frame.
    fn end_frame(&self) -> Result<(), SwapBuffersError> {
        unsafe {
            let mut cell = self.get_cell();
            if cell.is_none() {
                println!("Frame has already ended.");
                Ok(())
            } else {
                // Test whether the frame is borrowed.
                // TODO: Waiting `borrow_state` stabilization:
                // https://github.com/rust-lang/rust/issues/27733
                let _ = cell.as_ref().unwrap().borrow_mut();
                let frame = cell.take().unwrap().into_inner();
                frame.finish()
            }
        }
    }

    /// Start a new frame and auto end it.
    pub fn frame<F>(&self, f: F) -> Result<(), SwapBuffersError>
        where F: FnOnce()
    {
        // TODO: Refactor and error handling.
        self.start_frame();
        f();
        self.end_frame()
    }

    /// Returns the ratio between the backing framebuffer resolution
    /// and the window size in screen pixels.
    pub fn hidpi_factor(&self) -> f32 {
        self.display.get_window().unwrap().hidpi_factor()
    }
}
