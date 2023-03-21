use shaku::Interface;

pub trait ConsoleAccessor: Interface {
    fn println(&self);
}
