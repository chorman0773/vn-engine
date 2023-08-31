#[derive(Clone, Debug)]
pub enum GameEvent {
    ScriptNotify(u32, String),
    Periodic(f32),
}
