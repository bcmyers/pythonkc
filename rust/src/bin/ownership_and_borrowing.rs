#[allow(unused_assignments, unused_variables)]
fn main() {
    // variables are immutable by default...
    let a = 42;
    // a = 7;
    // error[E0384]: re-assignment of immutable variable `a`

    // ... but you can declare them as mutable
    let mut a = 42;
    a = 7;

    // ownership...
    let name = String::from("Brian");
    do_a_thing(name);
    // println!("Hello again, {}", name);
    // error[E0382]: use of moved value: `name`

    // borrowing...
    let name = String::from("Alice");
    do_another_thing(&name);
    println!("Hello again, {}\n", name);

    // mutable borrowing...
    let mut two_people: [&str; 2] = ["Alice", "Bob"];
    do_yet_another_thing(&mut two_people);
    println!("Hello again, {} and {}\n", two_people[0], two_people[1]);
}

fn do_a_thing(name: String) {
    println!("\nHello, {}\n", name);
}

fn do_another_thing(name: &str) {
    println!("Hello, {}", name);
}

fn do_yet_another_thing(two_people: &mut [&str; 2]) {
    *two_people = ["Alice", "Brian"];
    println!("Hello, {} and {}", two_people[0], two_people[1]);
}
