macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[repr(C)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub struct Thread {
    pub id: usize,
    pub stack: Vec<u8>,
    pub ctx: ThreadContext,
    pub state: State,
}

#[derive(PartialEq, Eq)]
pub enum State {
    Available,
    Running,
    Ready,
}

pub_struct! {
    ThreadContext {
        ra: u64,
        sp: u64,
        s0: u64,
        s1: u64,
        s2: u64,
        s3: u64,
        s4: u64,
        s5: u64,
        s6: u64,
        s7: u64,
        s8: u64,
        s9: u64,
        s10: u64,
        s11: u64,
        entry: u64,
    }
}

impl Thread {
    pub fn new_with_state(id: usize, state: State) -> Self {
        Thread {}
    }

    pub fn new(id: usize) -> Self {

    }
}