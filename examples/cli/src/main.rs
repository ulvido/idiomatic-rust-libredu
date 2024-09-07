use idiomatic_rust::*;

fn main() {
    println!("== Para Ayrıştır ==");

    println!("Miktar ve parabirimi girin.");

    loop {
        let mut para = String::new();

        std::io::stdin()
            .read_line(&mut para)
            .expect("Failed to read line");

        match para.trim().parse::<Para>() {
            Ok(p) => {
                println!("{p}")
            }
            Err(e) => {
                println!("{e}")
            }
        };
    }
}
