extern crate debugger;

fn main() {
    let pid = std::env::args().skip(1).next().expect("expected a PID");
    let process = match debugger::machine::attach_to(&pid) {
        Ok(p) => p,
        Err(e) => {
            println!("error: {}", e);
            return;
        },
    };

    println!("hello");
}
