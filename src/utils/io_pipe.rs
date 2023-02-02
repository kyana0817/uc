use std::io;

pub fn io_pipe(f: Box<dyn Fn(&str) -> String>) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read pipe");
        
        if input == "" {
            break;
        }

        input = input.trim().to_string();
            
        let converted = f(&input);

        println!("{}", converted);

    }
}
