mod solutions;

use solutions::structures_and_traits::structures_and_traits::{
    Circle, Drawable, Drower, Rectangle,
};

use solutions::associated_types::{
    Circle as AssociatedCircle, Rectangle as AssociatedRectangle, Shape,
};
use solutions::conditional_conformance::{Celsius, Convertable, Fahrenheit};
use solutions::global_traits::{ConsolePrinter, Message, Person, Printer};
use solutions::operators::Point;
use solutions::trait_bounds::figure_printer::{process, Rectangle as OtherRectangle, Square};
use solutions::trait_objects::{
    Circle as TraitCircle, Rectangle as TraitRectangle, Shape as TraitShape,
};
fn main() {
    println!("Running ownership and borrowing task:");
    let sentence: &str = "Ya ebal cho tak slojno";
    let the_longest_word: &str = solutions::ownership_borrowing::longest_word(sentence);
    println!("TASK 1 ANSWER: {}", the_longest_word);

    println!("Running mutable references task:");
    let mut array: Vec<i16> = Vec::from([1, 2, 3, 4, 5, 6]);
    let new_array: &mut Vec<i16> = solutions::mutable_references::change_values(&mut array);
    println!("TASK 2 ANSWER: {:?}", new_array);

    println!("Running iterator task:");
    let array: Vec<i16> = Vec::from([1, 2, 3, 4, 5, 6]);
    let new_array: Vec<i16> = solutions::iterators::choose_even(array);
    println!("TASK 3 ANSWER: {:?}", new_array);

    println!("Running matching task:");
    let a: i16 = 20;
    let b: i16 = 5;
    let operation: solutions::matching::Operations = solutions::matching::Operations::Add;
    let answer: i16 = solutions::matching::match_operations(a, b, operation);
    println!("TASK 4 ANSWER: {:?}", answer);

    println!("Running matching task:");
    let a: i16 = 20;
    let b: i16 = 5;
    let operation: solutions::matching::Operations = solutions::matching::Operations::Add;
    let answer: i16 = solutions::matching::match_operations(a, b, operation);
    println!("TASK 4 ANSWER: {:?}", answer);

    println!("Running structures and traits task:");
    let a: i16 = 20;
    let b: i16 = 5;
    let circle: Circle = Circle { radius: 10 };
    let rectangle: Rectangle = Rectangle { a: 20, b: 10 };

    let drower: Drower = Drower {};

    drower.draw(&circle);
    drower.draw(&rectangle);

    let other_rectangle = OtherRectangle { a: 10, b: 20 };
    let sqare = Square { a: 20 };

    process(&other_rectangle);
    process(&sqare);

    let person: Person = Person {
        name: String::from("Harry"),
    };
    let message: Message = Message {
        message: String::from("SOSI"),
    };

    person.print();
    message.print();

    person.console_print();
    message.console_print();

    let point_1: Point = Point { x: 1.0, y: 1.0 };
    let point_2: Point = Point { x: 2.0, y: 3.0 };

    let summ: Point = point_1 + point_2;

    println!("Summed point: {}", summ);

    let rectangle: AssociatedRectangle = AssociatedRectangle {
        width: 10,
        height: 20,
    };
    let circle: AssociatedCircle = AssociatedCircle { radius: 15.0 };

    println!("{}", rectangle.area());
    println!("{}", circle.area());
    println!();

    println!("Using trait objects");

    let shape: &dyn TraitShape = &TraitCircle { radius: 10.0 };
    println!("Circle area {}", shape.area());

    let shape = &TraitRectangle {
        width: 10.0,
        height: 10.0,
    };
    println!("Rectangle area {}", shape.area());

    let celsius: Celsius = Celsius(100.0);
    let fahrenheit: Fahrenheit = Fahrenheit(212.0);

    let converted_celsius: Celsius = fahrenheit.convert();
    let converted_fahrenheit: Fahrenheit = celsius.convert();

    println!("212°F in Celsius: {:.2}°C", converted_celsius.0);
    println!("100°C in Fahrenheit: {:.2}°F", converted_fahrenheit.0);
}
