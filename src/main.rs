
use std::env::{args,Args};


fn main() {
    let mut args:Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    let first_number:f32 = first.parse().unwrap();
    let second_number:f32 = second.parse().unwrap();
    let result = operate(first_number,second_number,operator);
    println!("{:?}",display(first_number,second_number,operator,result));
}

fn operate(first_number:f32, second_number:f32, operator:char)->f32{
    if operator == '+' {
        first_number + second_number
    }else if operator == '-' {
        first_number - second_number
    }else if operator== '*' {
        first_number *second_number
    }else if operator == '/' {
        first_number / second_number
    }else{
        0.0
    }
}

fn display(first_number:f32,second_number:f32,operator:char,result:f32)->String{
    format!("{:?} {:?} {:?} = {:?}",first_number,operator,second_number,result)
}