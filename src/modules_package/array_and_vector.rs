use std::io;
pub fn table (x: i32) ->i32{
    let mut sum=0;
    println!(" ***************multiplaction table of {} *********************", x);
    for i in 1..11 {
        println!("{}  x  {} =   {}", x,i, x*i);
        sum+=x*i;
    }
    //println!("sum of multiplaction table {} =   {}", x,sum);
    return sum;
}
pub fn sum_of_numbers (x: i32){
    let mut sum=0;
    for i in 1..x+1 {
        
        //let mut n = String::new();
        //io::stdin().read_line(&mut n).expect("failed to read input.");
        //let n: i32 = n.trim().parse().expect("invalid input");
        sum+=i;
    }
    println!("Sum of N Numbers {}", sum);
}

pub fn vector(){
    let data = vec![1,2,3,4,5,6,7];

    for x in data.iter().step_by(2) {
        println!("{}", x)
    }
    let  vec: Vec<u32> = (0..10).collect();
    println!("{:?}", vec); //  whole value print;
    println!("{}",vec[2]); // selected index value printed;
}

pub fn dy_vector( x:i32) -> Vec<i32> {
    let mut marks_vector_list = vec![];
    let mut buffer = String::new();
    println!("Dynamic vector list starting  ! insert your {} subjects marks",x);
    for _ in 0..x {
        io::stdin().read_line(&mut buffer).expect("Failed to read");
        let  entered_marks: i32 = buffer.trim().parse().expect("Invalid number");
        marks_vector_list.push(entered_marks);
        buffer.clear();
    }

    // Display the numbers.
    println!("marks_vactor_list: {:?}", marks_vector_list);
    return marks_vector_list;
}
