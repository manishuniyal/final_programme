
pub fn fun_test_call() {
    println!("manish, world!");
    println!("manish, world!");
    
    }
/*
pub fn arr()->[i32]{
    let array : [u32; 10]= [3,6,9,12,15,18,21,24,27,30];
    array[3];
}*/

pub fn odd_even(n:i32) -> () {
    // if nmber is odd
    if top(n){
        println!("odd number");
    }else{
        // if number is even
        println!("numbr is even");
    }
    
}
fn top (x:i32)-> bool{
    {
        return x%2==0;
    }

}
pub fn funny (score:i32) {
    if score >95{
        println!("Congratulation !!! you won the scholoarship for Studying in America");
    }else if score>85{
        println!("Congratulation !!! you won the scholoarship for Studying in England");
    }else {
        println!("Better luck next time!!! you'r not qualify for scholoarship");
    }
}