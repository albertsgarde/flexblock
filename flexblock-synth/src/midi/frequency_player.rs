use crate::{
    midi::MidiPlayer,
    modules::{Module, ModuleTemplate},
};
use array_init;
use midi::Message;

struct Note<M: Module> {
    module: Option<M>,
    sample_num: u64,
}

impl<M: Module> Default for Note<M> {
    fn default() -> Note<M> {
        Note {
            module: None,
            sample_num: 0,
        }
    }
}

pub struct FrequencyPlayer<M: Module> {
    module_template: ModuleTemplate<M>,
    notes: [Note<M>; 128],
    frequencies: [f32; 128],
}

impl<M: Module> FrequencyPlayer<M> {
    pub fn new(module: ModuleTemplate<M>) -> Self {
        FrequencyPlayer {
            module_template: module,
            notes: array_init::array_init(|_| Note::default()),
            frequencies: array_init::array_init(|i| {
                440. * (2. as f32).powf((i as f32 - 69.) / 12.)
            }),
        }
    }

    fn start_note(&mut self, note: usize) {
        let note = &mut self.notes[note];
        note.module = Some(self.module_template.create_instance());
        note.sample_num = 0;
    }

    fn stop_note(&mut self, note: usize) {
        self.notes[note].module = None;
    }
}

impl<M: Module> MidiPlayer for FrequencyPlayer<M> {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::NoteOn(_, note, _) => self.start_note(note as usize),
            Message::NoteOff(_, note, _) => self.stop_note(note as usize),
            Message::AllNotesOff(_) => (0..128).for_each(|note| self.stop_note(note)),
            _ => {}
        }
    }

    fn next(&mut self) -> (f32, f32) {
        let result = (0..128).fold(0., |cur, note| {
            cur + match &mut self.notes[note].module {
                Some(module) => {
                    self.notes[note].sample_num += 1;
                    module.next(self.notes[note].sample_num)
                }
                None => 0.,
            }
        });
        (result, result)
    }
}
