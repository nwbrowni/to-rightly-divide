pub mod cli;

pub trait UI {
    fn launch(&self);
    fn close(&self);
}