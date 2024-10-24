pub struct Person {
    pub name: String,
}

pub struct Message {
    pub message: String,
}

pub trait Printer {
    fn print(&self);
}

impl Printer for Person {
    fn print(&self) {
        println!("Person name {}", &self.name);
    }
}

impl Printer for Message {
    fn print(&self) {
        println!("Message {}", &self.message);
    }
}

pub trait ConsolePrinter {
    fn console_print(&self);
}

impl<T: Printer> ConsolePrinter for T {
    fn console_print(&self) {
        println!("******Печать на консоль*****");
        self.print();
        println!();
    }
}
