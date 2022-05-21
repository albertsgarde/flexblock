use std::{time::{Instant, Duration}, thread};

pub struct Tick {
    start: Instant,
    tick: u128,
    ticks_per_second: u128,
}

impl Tick {
    pub fn start(ticks_per_second: u32, tick: usize) -> Tick {
        Tick {start: Instant::now(), tick: tick.try_into().unwrap(), ticks_per_second: ticks_per_second.try_into().unwrap()}
    }

    pub fn sync_next_tick(&mut self) {
        self.tick += 1;
        while self.start.elapsed().as_nanos()*self.ticks_per_second < self.tick*10u128.pow(9) {
            thread::sleep(Duration::from_nanos(
                ((self.tick*10u128.pow(9)-self.start.elapsed().as_nanos()*self.ticks_per_second)/self.ticks_per_second).try_into().unwrap(),
            ));
        }
    }
}