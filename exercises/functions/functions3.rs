// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    call_me(7);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call {}", i + 1);
    }
}
