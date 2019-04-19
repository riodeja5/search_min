use std::io;

fn main() {

    let max_num = 100;
    let input_num = 5;
    let mut vec_input = vec![0; input_num];

    for x in 0..input_num {

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        vec_input[x] = input;
    }

    let min = vec_input.iter().cloned()
        .fold(max_num, i32::min);
    println!("{}", min);
}
