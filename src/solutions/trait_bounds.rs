pub mod figure_printer {
    pub struct Rectangle {
        pub a: i16,
        pub b: i16,
    }

    pub struct Square {
        pub a: i16,
    }

    pub trait Printer {
        fn print(&self);
    }

    pub trait Sender {
        fn send(&self);
    }

    impl Printer for Rectangle {
        fn print(&self) {
            println!("printing rectangle: a - {}, b - {}", self.a, self.b)
        }
    }

    impl Sender for Rectangle {
        fn send(&self) {
            println!("sending rectangle");
        }
    }

    impl Printer for Square {
        fn print(&self) {
            println!("printing square: a - {}, b - {}", self.a, self.a)
        }
    }
    impl Sender for Square {
        fn send(&self) {
            println!("sending square");
        }
    }

    pub fn process<T: Sender + Printer>(obj: &T) {
        obj.print();
        obj.send();
    }
    // pub fn process(obj: &(impl Printer + Sender)) {
    //     obj.print();
    //     obj.send();
    // }
    // pub fn process<T>(obj: T)
    // where
    //     T: Printer + Sender,
    // {
    //     obj.print();
    //     obj.send();
    // }
}
