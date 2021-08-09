mod ftest;

fn main() {
    println!("Hethirsllo, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!(" main , world!");
    println!(" main , world!");
    // main.rs file  function call ;
    tests();
    // fin_test.rs file function call using scope--resolution oprerator;
    ftest::add_to_waitlist();
    
    


}
//main.rs file function decleration;
fn tests() {
    println!("Hello, world!");
    println!("Hello, world!");
}