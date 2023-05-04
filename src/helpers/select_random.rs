use std::fs;
use rand::Rng;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Problem { // would be like classes in OOP
    done: bool,
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    problems: Vec<Problem>
}

pub fn select_random_problem() {
    let data = fs::read_to_string("./data.json") // read json file
        .expect("Unable to read file");
    let data_obj: Data = serde_json::from_str(&data) // parse json to vector
        .expect("ERROR: Should read the file");
    
    let mut not_done: Vec<Problem> = Vec::new(); // create vector

    for prb in data_obj.problems.iter() { // add "not done" to vector
        if prb.done == false { 
            not_done.push(Problem { 
                done: prb.done, 
                title: prb.title.to_string(),
            });
        }
    }
    
    let mut rng = rand::thread_rng(); // generate random seed
    let rand:usize = rng.gen_range(0..not_done.len()) // build random uint
        .try_into().unwrap();     
    println!("\nProblem Chosen ---> {:?}", not_done[rand].title.to_string());
    
}
