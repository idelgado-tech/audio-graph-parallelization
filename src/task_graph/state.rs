#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskState {
    WaitingDependencies,
    Ready,
    Scheduled,  // Used by static sheduling algorithms only
    Processing, // Used by dynamic scheduling algorithms only
    Completed,  // Used by dynamic scheduling algorithms only
}