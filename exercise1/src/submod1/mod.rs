pub mod submod2;

pub fn print_lower_a_to_upper_z() {
    for c in ('Z'..='a').rev() {
        print!("{}", c);
    }
    println!("");
}
