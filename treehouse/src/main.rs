#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => {
                println!("Welcome to the treehouse, {}", self.name);
            }
            VisitorAction::Probation => {
                println!("{} is now a probationary member", self.name);
            }
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Refuse => println!("Sorry, you are not on the list"),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("home", VisitorAction::Accept, 45),
        Visitor::new("bart", VisitorAction::Accept, 9),
        Visitor::new(
            "lisa",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            8,
        ),
        Visitor::new("maggie", VisitorAction::Refuse, 2),
    ];

    loop {
        println!("Hello, what's your name ? (Press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => {
                visitor.greet_visitor();
            }
            None => {
                if name.is_empty() {
                    println!("The final list of all visitors:");
                    println!("{:#?}", visitor_list);
                    break;
                }
                println!("You are not on the list");
                visitor_list.push(Visitor::new(&name, VisitorAction::Probation, -1));
            }
        }
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}
