extern crate rand;
extern crate pcg;

use std::rc::Rc;
use std::cell::RefCell;

use rand::{
    Rng,
    SeedableRng,
    OsRng,
};
use pcg::PcgRng;

// For this thread rng, we just have a PcgRng originally seeded by an OsRng. We could wrap it in a
// ReseedingRng, but I think that just using PcgRng is probably fine for our uses.

pub fn thread_pcg_rng() -> ThreadPcgRng {
    thread_local!(static THREAD_RNG_KEY: Rc<RefCell<PcgRng>> = {
        let mut os_rng = match OsRng::new() {
            Ok(r) => r,
            Err(e) => panic!("could not initialize os rng to seed thread rng: {}", e),
        };
        let pcg_rng = PcgRng::from_seed([os_rng.next_u64(), os_rng.next_u64()]);

        Rc::new(RefCell::new(pcg_rng))
    });

    ThreadPcgRng {
        inner: THREAD_RNG_KEY.with(|t| t.clone())
    }
}

#[derive(Clone)]
pub struct ThreadPcgRng {
    inner: Rc<RefCell<PcgRng>>,
}

impl Rng for ThreadPcgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.inner.borrow_mut().next_u32()
    }
}
