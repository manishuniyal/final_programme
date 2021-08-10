use std::io;
use crate::array_and_vector::dy_vector;
use crate::fun_test::funny;

/*let mut input = String::new();
    io::stdin::().read_line(&mut input).expect(“error: unable to read user input”);
    println!("{}",input);*/

pub fn calculate_marks(){
    
    println!("Enter Your total subjects of classs 12th ");  

    let mut total_subjects = String::new();
    io::stdin().read_line(&mut total_subjects).expect("failed to read input.");
    let total_subjects: i32 = total_subjects.trim().parse().expect("invalid input");
    
    //array_and_vector::dy_Vector(total_subjects);
    dy_vector(total_subjects);
    println!("total subjects  {:?}", total_subjects);

    println!("noe check");
    println!("{:?}",total_subjects);

}
pub fn scholoarship_check(){
    println!("ENTER YOUR MARKS FOR SCHOLORSHIP");
    let mut scholorship_marks = String::new();
    io::stdin().read_line(&mut scholorship_marks).expect("failed to read input.");
    let scholorship_marks: i32 = scholorship_marks.trim().parse().expect("invalid input");
    funny(scholorship_marks);

}