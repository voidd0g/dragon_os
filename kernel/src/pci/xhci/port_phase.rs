#[derive(PartialEq, Eq)]
pub enum PortPhase {
    NotConnected,
    ResettingPort,
    EnablingSlot,
    HasSlot,
}
