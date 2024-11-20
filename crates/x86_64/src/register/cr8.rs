use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct CR8: u64 {}
}

impl CR8 {}
