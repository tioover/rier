use time;
use rand;


/// Unique identifier.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
pub struct Id {
    /// Created time.
    pub time: time::Timespec,
    /// Random number.
    pub random: u32,
}


impl Id {
    // Creates unique id.
    pub fn new() -> Id {
        Id {
            random: rand::random(),
            time: time::now().to_timespec(),
        }
    }
}
