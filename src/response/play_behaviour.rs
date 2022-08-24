use std::fmt;

#[allow(dead_code)]
pub enum PlayBehaviour {
    Enqueue,
    ReplaceAll,
    ReplaceEnqueued,
}

impl fmt::Display for PlayBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PlayBehaviour::Enqueue => "ENQUEUE",
            PlayBehaviour::ReplaceAll => "REPLACE_ALL",
            PlayBehaviour::ReplaceEnqueued => "REPLACE_ENQUEUED",
        };
        write!(f, "{}", s)
    }
}