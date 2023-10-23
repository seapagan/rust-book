fn main() {
    print_labeled_measurements(5, 'h');
}

fn print_labeled_measurements(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}
