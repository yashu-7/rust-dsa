fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    
    for number in numbers{
        println!("{}",number);
    }

    let added_two: Vec<i32> = numbers.iter().map(|n| n+2).collect();
    println!("{:?}",added_two);

}