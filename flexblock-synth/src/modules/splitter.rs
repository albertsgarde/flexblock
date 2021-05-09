use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;
use std::sync::{Arc, RwLock};

struct Splitter<S: Module> {
    source: S,
    cur_sample_num: u64,
    cur_sample: f32,
}

impl<S: Module> Splitter<S> {
    fn new(source: S) -> Splitter<S> {
        Splitter {
            source,
            cur_sample_num: u64::MAX,
            cur_sample: 0.,
        }
    }

    fn next(&mut self, sample_num: u64) -> f32 {
        if sample_num == self.cur_sample_num {
            self.cur_sample
        } else if sample_num == 0 || sample_num == self.cur_sample_num + 1 {
            self.cur_sample_num = sample_num;
            self.cur_sample = self.source.next(self.cur_sample_num);
            self.cur_sample
        } else {
            panic!(
                "This should never happen! sample_num should always increase only by 1 or reset."
            );
        }
    }
}

#[module]
pub struct SplitterOut<S: Module> {
    source: Arc<RwLock<Splitter<S>>>,
}

impl<S: Module> SplitterOut<S> {
    fn new(source: Arc<RwLock<Splitter<S>>>) -> SplitterOut<S> {
        SplitterOut { source }
    }
}

impl<S: Module> Module for SplitterOut<S> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.source.write().unwrap().next(sample_num)
    }
}

pub fn split_signal<S: Module>(
    module: ModuleTemplate<S>,
    num_outs: usize,
) -> Vec<ModuleTemplate<SplitterOut<S>>> {
    let splitter = Arc::new(RwLock::new(Splitter::new(module.module)));
    let mut result: Vec<ModuleTemplate<SplitterOut<S>>> = Vec::with_capacity(num_outs);
    for _ in 0..num_outs {
        result.push(ModuleTemplate {
            module: SplitterOut::new(Arc::clone(&splitter)),
        });
    }
    result
}
