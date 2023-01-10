use std::io;
fn main() {
    println!("Enter a sequence number:");
    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number)
            .expect("Failed to read the line");
        let number: f32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let first = f32::powf(0.6180339887498948482, number);
        let second = f32::powf(1.6180339887498948482, number);
        let third = second - first;
        let fin = third / 2.2360679774997896964;
        println!("{:.0}", fin);
        break;
    };
}
