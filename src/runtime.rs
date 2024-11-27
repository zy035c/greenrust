use crate::greenlet;

static mut RUNTIME: usize = 0;

pub struct Runtime {
    threads: Vec<greenlet::Thread>,
    current: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread_id = 0;
        let base_thread = greenlet::Thread::new_with_state(
            base_thread_id,
            greenlet::State::Running
        );

        let mut threads = vec![base_thread];
        let mut available_threads = (1..MAX_THREADS + 1).map(|i| greenlet::Thread::new(i)).collect();

        Runtime {
            threads,
            current: base_thread_id,
        }
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| matches!(t.state, greenlet::State::Available))
            .expect("no available green thread.");

        println!("RUNTIME: spawning task on green thread {}", available.id);
        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !7) as *mut u8;

            available.ctx.ra = task_return as u64;
            available.ctx.sp = s_ptr as u64;
            available.ctx.entry = f as u64;
        }
        available.state = greenlet::State::Ready;
    }

    fn t_return(&mut self) {
        self.threads[self.current].state = greenlet::State::Available;
        self.t_schedule();
    }

    pub fn run(&mut self) {
        while self.t_yield() {}
        println!("All tasks finished!");
    }

    fn t_yield(&mut self) -> bool {
        self.threads[self.current].state = greenlet::State::Ready;
        self.t_schedule()
    }

    fn t_schedule(&mut self) -> bool {
        let thread_count = self.threads.len();

        let mut pos = (self.current + 1) % thread_count;
        while self.threads[pos].state != greenlet::State::Ready {
            pos = (pos + 1) % thread_count;
            if pos == self.current {
                return false;
            }
        } 
        true
    }
}

fn task_return() {
    unsafe {
        let r_ptr = RUNTIME as *mut Runtime;
        (*r_ptr).t_return();
    }
}

pub fn r#yield() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    }
}