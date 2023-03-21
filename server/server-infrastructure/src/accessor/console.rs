use server_domain::console::ConsoleAccessor;
use shaku::Component;

#[derive(Component)]
#[shaku(interface=ConsoleAccessor)]
pub struct ConsoleAccessorImpl;

impl ConsoleAccessor for ConsoleAccessorImpl {
    fn println(&self) {
        println!("printtttttttt")
    }
}
