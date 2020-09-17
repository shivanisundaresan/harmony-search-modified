extern crate rand;
use rand::Rng;
use std::io;
use std::time::{Instant};
fn main() {
    let mut input = vec![vec![2, 2, 1], vec![1, 3, 4],vec![5, 3, 3]];
    let mut best_score = std::i32::MAX;
    let mut best_vec: Vec<i32> = Vec::new();
    let mut harmonic_memory: Vec<Vec<i32>> = Vec::new();
    let mut harmony_memory: Vec<i32> = Vec::new();
    let mut harmony = std::i32::MAX;
    let mut harmonic: Vec<i32> = Vec::new();

    //print the initial input
    println!("The current shuffled input is: ");
    for i in 0..input.len(){
        println!("{:?}", input[i]);
    }

    println!();

    //user input for number of iterations
    println!("Please input the number of iterations");
    let mut num_of_iterations = String::new();
    io::stdin().read_line(&mut num_of_iterations).expect("Failed to read line");
    let num_of_iterations: usize = num_of_iterations.trim().parse().expect("Please type a number");
    println!();
    
    let now = Instant::now();
    for _i in 0..num_of_iterations
    {
        //shuffle input
        input = shuffle(input);
        println!();
        println!("The current shuffled input is: ");
        for i in 0..input.len(){
            println!("{:?}", input[i]);
        }
        //push the best score into harmony memory
        for i in 0..input.len(){
            if equation(input[i].clone()) < best_score{
                best_score = equation(input[i].clone());
                best_vec = input[i].clone();
            }
        }

        //push the best vector into harmonic memory and best score into harmony memory
        harmonic_memory.push(best_vec.clone());
        harmony_memory.push(best_score);
    }

    println!();
    println!("Harmony memory {:?}", harmony_memory);
    println!("Harmonic memory {:?}", harmonic_memory);

    //search harmonic memory for the best vector
    for i in 0..harmony_memory.len(){
        if harmony_memory[i] < harmony{
            harmony = harmony_memory[i];
            harmonic = harmonic_memory[i].clone();
        }
    }

    //print the best vector
    let time = now.elapsed().subsec_nanos();
    println!();
    println!("The most optimized sequence in {} iterations is {:?} with a score of {}. It took {:?} nanoseconds to find this solution", num_of_iterations, harmonic, harmony, time);
}
//optimization equation (given)
fn equation(input : Vec<i32>) -> i32{
    let a = input[0];
    let b = input[1];
    let c = input[2];
    return (a-2)*(a-2) + (b-3)*(b-3)*(b-3)*(b-3) + (c-1)*(c-1) + 3;
}
//shuffles the vector
fn shuffle(mut input : Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    for i in 0..input.len(){
        for j in 0..input[i].len(){
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0, input[i].len());
            let y = rng.gen_range(0, input[i].len());
            let temp = input[i][j];
            input[i][j] = input[x][y];
            input[x][y] = temp;

        }
    }
    return input;
}
