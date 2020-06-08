/* Computer Science Diversity Simulation.

This program creates 3 groups of threads.
Low CS Group -> Meant to represent people without much CS privilege. (0)
Medium CS Group -> Meant to represent people with medium CS privilege. (20)
High CS Group -> Meant to represent  people with high CS priviledge. (50)

These groups start off at different point values to begin.

Each group will be able to study(), a blocking function which has a probability to 
increase an individuals' CS points. 

The goal of a thread is to finish 3 CS classes (simulated to be CS106A, CS106B, and CS107, courses at Stanford.)
They will send a message to the CS Department thread in the following 
    - Identifier for the thread (LOW_GROUP_1 or MED_GROUP_5)
    - Class they're currently taking 
    - Amount of CS Points to their name. 

The department will inform the thread if they can move on to another class, of if they've failed and require 
another try. Each thread is given 3 tries (for all the classes). If they run out of tries, they 'drop out' the major.

After all threads in group finish, the CS Department thread will calculate statistics, such as: 
"70% of the LOW_GROUP dropped out."
     
What statistics will we find? Run the executable and get a simulation for yourself :) 

Organizations to support CS Diversity: 
http://www.code2040.org/
https://girlswhocode.com/


*/
use std::thread;
use crossbeam::channel; // For communucation with CS Department Thread
use std::collections::HashMap; // For storing data to eventually run statistics. 
use std::io;
use rand::Rng;
use std::time::Duration;
use std::fs::File;

/// study function. Each thread will run this individually, which will force them to study!
/// Returns number of points that should be added to their total
fn study(points: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 30);
    let time_to_sleep_seconds = num as f64 / points; //* the more points you have, the less you need to study!
    
    thread::sleep(Duration::new(time_to_sleep_seconds as u64, 0)); 

    let upper = 20.0 / (1.0 + points.powf(0.5)); 
    rng.gen_range(0.0, upper) as f64
}

/// student_thread_routine will create a new student. Their goal is to finish all 3 CS classes, without
/// dropping out. 
/// Params:
/// identifier: String -> How a thread will be known in data collection (LOW_GROUP_1)
/// points: u32 -> The initial amount of points this student was spawned with. 

fn student_thread_routine(identifier: String, mut points: f64, channel_tuple: (crossbeam::channel::Sender<(String, u32, f64)>, crossbeam::channel::Receiver<bool>)) {
    //* Channel for a student to recieve info on if they've passed a class. 
    let mut curr_class_idx = 0; 
    let mut num_tries = 3;
    
    loop {
        points += study(points);
        channel_tuple.0.send((identifier.clone(), curr_class_idx, points)).expect("Sending to CS DEPT failed");
        let ret = channel_tuple.1.recv().expect("Failed to recieve.");
        //* We've failed the class...
        if !ret {
            num_tries -= 1;
        } else {
            curr_class_idx += 1;
        }
        if num_tries == 0 {
            println!("{} has ran out of tries and quit the CS major.", identifier);
            return;
        }
        if curr_class_idx == 3 {
            println!("{} has finished the intro CS Core!", identifier);
            return;
        }
    }
}

//* Gets a number from stdin. If not a number, will try again. 
fn get_number(message: &str) -> f64 {
    println!("{}", message);
    loop {
        let stdin = io::stdin();
        let mut buf = String::from("");
        stdin.read_line(&mut buf).expect("Reading from STDIN failed... Try again, that should be rare.");
        let value =  buf.trim().parse(); 
        
        match value {
            Ok(val) => return val,
            Err(_msg) => {
                println!("Hmm.. that doesn't seem to a number. Try again");
            }
        }
    }
}

fn cs_dept_thread_routine(tables: &mut (HashMap<String, crossbeam::channel::Sender<bool>>, HashMap<String, Vec<f64>>), receiver_dpt: crossbeam::channel::Receiver<(String, u32, f64)>) {
    loop {
        let tuple = receiver_dpt.recv().unwrap();
        //* main thread will send this when the routine should end.
        if tuple.0 == "" {
            println!("CS Dept Thread Finished.");
            println!("Printing Progress Table in JSON to new File");
            let file = File::create("table.txt").expect("File creation failed");
            serde_json::to_writer(file, &tables.1).expect("Writing table to file failed");
            return;
        } 

        // .0 -> identifer, .1 -> class, .2 -> pts
        //* First, lets add their new point value to their vector. 
        let vec_points = tables.1.get_mut(&tuple.0).unwrap();
        vec_points.push(tuple.2);

        //* Probability to pass class is based on a log function, log(x / y), where y 
        //* depends on which class you're taking. 
        let probability = (tuple.2 / tuple.1 as f64).log10();
        let mut rng = rand::thread_rng();
        let hit = rng.gen_range(0.0, 1.0);

        let sender = tables.0.get(&tuple.0).unwrap();
        if hit <= probability {
            sender.send(true).unwrap(); // you pass!    
        } else {
            sender.send(false).unwrap(); // you fail D: 
        }
    }
}
fn main() {
    //* Constant vector on heap representing classes that need to get taken.
    let k_num_threads = 50; // how many threads for each group.
    let mut student_threads = Vec::new(); 

    //* Lets read from input, so that users can be allowed to change pt values for different groups. 
    
    let low_pts = get_number("Enter value for LOW group.");
    let med_pts = get_number("Enter value for MEDIUM group");
    let high_pts = get_number("Enter value for HIGH group");
    let point_values = vec![low_pts, med_pts, high_pts];

    //* Channel for a student to send info about their pts and class.
    let (sender_dpt, reciever_dpt) = channel::unbounded::<(String, u32, f64)>();
    
    //* These tables are for the CS DEPT thread. 
    //* .0 -> Table with channel endpoints for each individual thread.
    //* .1 -> Table with point values for each thread to track growth. 
    let mut tables  = (HashMap::new(), HashMap::new());


    //* Spawn in each group of threads. 
    // LOW_GROUP_1, MED_GROUP_1, HIGH_GROUP_1...
    for i in 0..k_num_threads {
        let low = format!("LOW_GROUP_{}", i);
        let med = format!("MED_GROUP_{}", i);
        let high = format!("HIGH_GROUP_{}", i);
        let all_strs = vec![low, med, high];
        
        for i in 0..all_strs.len() {
            let identifier = all_strs[i].clone();
            let point_value = point_values[i];

            let (sender_stud, reciever_stud) = channel::unbounded::<bool>();
            tables.0.insert(identifier.clone(), sender_stud.clone()); 
            tables.1.insert(identifier.clone(), vec![point_value]);

            let sender_copy = sender_dpt.clone();
            student_threads.push(thread::spawn(move || {
                student_thread_routine(identifier,  point_value, (sender_copy, reciever_stud));
            }));
        }
    }
    //* After all threads have been made, we can create the CS Dept Thread, and give it ownership of the table.
    let cs_dept = thread::spawn(move ||  {
        cs_dept_thread_routine(&mut tables, reciever_dpt);
    });
    for thread in student_threads {
        thread.join().expect("A thread panicked!");
    }
    //* All student threads are finished here. Let's end the CS_Dept thread, and print out our info. 
    sender_dpt.send(("".to_string(), 0 , 0.0)).unwrap();
    cs_dept.join().expect("The CS Dept thread panicked!");
}
