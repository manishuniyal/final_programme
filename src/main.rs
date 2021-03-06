/*mod fun_test;
mod fun_test2;
mod array_and_vector;
mod input_and_output;*/
use std::io;
#[path = "modules_package/module1.rs"] mod module1;
#[path = "modules_package/fun_test.rs"] mod fun_test;
#[path = "modules_package/array_and_vector.rs"] mod array_and_vector;
#[path = "modules_package/fun_test2.rs"] mod fun_test2;

#[path = "modules_package/input_and_output.rs"] mod input_and_output;

#[path = "modules_package/module1.rs"] mod module2;



//use crate::fun_test2::scholoarship_check;

fn main() {
    println!("Hello, world!");
    println!(" Army , world!");
    println!("Civilian, world!");

    // main.rs file  function call ;
    tests();

    // fin_test.rs file function call using scope--resolution oprerator;
    fun_test::fun_test_call();   
    fun_test::odd_even(55);


    /*println!("ENTER YOUR MARKS FOR SCHOLORSHIP");
    let mut scholorship_marks = String::new();
    io::stdin().read_line(&mut scholorship_marks).expect("failed to read input.");
    let scholorship_marks: i32 = scholorship_marks.trim().parse().expect("invalid input");
    */
    fun_test2::scholoarship_check();

    /*
    important for converting intput string into integer
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    println!("{:?}", n);

    */
    let s = "Hello";
    let i = 42;

    print_type_of(&s); // &str
    print_type_of(&i); // i32

    // fin_test2.rs file function call using scope--resolution oprerator;

    fun_test2::calculate_marks();
    let m =array_and_vector::table(2);
    array_and_vector::sum_of_numbers(10);
    println!("sum of multiplaction table 2 =   {}", m);
    input_and_output::get_input_integer();
    input_and_output::get_input_string();
    //array_and_vector::vector();
    //array_and_vector::Dy_Vector();
    module1::module_test();


}
 //******************** OutSide Main Function Decleration ****************
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
//main.rs file function decleration;
fn tests() {
    println!("Hello, world!");
    println!("Hello, world!");
}