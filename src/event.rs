use std::rc::{ Rc, Weak };
pub use glium::glutin::Event as WindowEvent;


pub trait Event: Clone + Sized + 'static
{
}

impl Event for WindowEvent {}


pub struct SubscriberList<E: Event>
{
    list: Vec<Weak<Subscriber<E>>>,
}


impl<E: Event> SubscriberList<E>
{
    pub fn new() -> SubscriberList<E>
    {
        SubscriberList { list: Vec::new() }
    }

    pub fn register(&mut self, subscriber: &Rc<Subscriber<E>>)
    {
        self.list.push(Rc::downgrade(subscriber));
    }

    pub fn notify(&mut self, event: E)
    {
        self.list.retain(|x| {
            let x = x.upgrade();
            if let Some(x) = x
            {
                x.notify(&event);
                true
            }
            else { false }
        })
    }
}


pub trait Subscriber<E: Event>
{
    fn notify(&self, event: &E);
}
