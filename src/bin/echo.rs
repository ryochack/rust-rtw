use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for (i, v) in args[1..].iter().enumerate() {
        print!("{}{}",
               if i == 0 {
                   ""
               } else {
                   " "
               }, v);
    }
    println!()
}
