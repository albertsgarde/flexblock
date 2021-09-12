/// Represents events the logic around the state might have to react to.
/// This is things like saving and loading, but does not include state events like jumping and moving and does not include key presses.
pub enum LogicEvent {
    Save,
    LoadLatest,
}
