//mod fun_test;
use std::io;

/*let mut input = String::new();
    io::stdin::().read_line(&mut input).expect(“error: unable to read user input”);
    println!("{}",input);*/

pub fn calculate_marks(){
    
    println!("Enter Your marks of classs 12th ");  

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    println!("{:?}", n);

    println!("noe check");
    println!("{:?}",n);

}