use std::io;

pub fn get_input_string() -> String {
    let mut input_from_user = String::new();

    // Reads the input from STDIN and places it in the String named input.
        println!("Enter a String value:");
    io::stdin().read_line(&mut input_from_user)
        .expect("Failed to read input.");

    //print!("'{}'", input_from_user);
    //let input_from_user = input_from_user.trim();
    return input_from_user;

}


pub fn get_input_integer() -> i32 {
    let mut input_integer = String::new();

    // Reads the input from STDIN and places it in the String named input.
    println!("Enter a Integer Number value:");
    io::stdin().read_line(&mut input_integer)
        .expect("Failed to read input.");

    // Convert to an i32.
    let input_integer: i32 = input_integer.trim().parse().unwrap();

    //print!("'{}'", input_integer);
    return input_integer;
}