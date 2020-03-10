// use std::collections::{HashMap, HashSet};
use runner::common::*;
use std::collections::HashSet;

fn main() {

    /*
    // Original task set.
    let t1 = Task {
        id: "T1".to_string(),
        prio: 1,
        deadline: 100,
        inter_arrival: 100,
        trace: Trace {
            id: "T1".to_string(),
            start: 0,
            end: 10,
            inner: vec![],
        },
    };

    let t2 = Task {
        id: "T2".to_string(),
        prio: 2,
        deadline: 200,
        inter_arrival: 200,
        trace: Trace {
            id: "T2".to_string(),
            start: 0,
            end: 30,
            inner: vec![
                Trace {
                    id: "R1".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![Trace {
                        id: "R2".to_string(),
                        start: 12,
                        end: 16,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R1".to_string(),
                    start: 22,
                    end: 28,
                    inner: vec![],
                },
            ],
        },
    };

    let t3 = Task {
        id: "T3".to_string(),
        prio: 3,
        deadline: 50,
        inter_arrival: 50,
        trace: Trace {
            id: "T3".to_string(),
            start: 0,
            end: 30,
            inner: vec![Trace {
                id: "R2".to_string(),
                start: 10,
                end: 20,
                inner: vec![],
            }],
        },
    };

    // builds a vector of tasks t1, t2, t3
    let tasks: Tasks = vec![t1, t2, t3];

    */

    // Task set 2, should work for exact preemtion but not approx.
    let t1 = Task {
        id: "T1".to_string(),
        prio: 1,
        deadline: 150,
        inter_arrival: 150,
        trace: Trace {
            id: "T1".to_string(),
            start: 0,
            end: 30,
            inner: vec![],
        },
    };

    let t2 = Task {
        id: "T2".to_string(),
        prio: 2,
        deadline: 150,
        inter_arrival: 150,
        trace: Trace {
            id: "T2".to_string(),
            start: 0,
            end: 30,
            inner: vec![
                Trace {
                    id: "R1".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![Trace {
                        id: "R2".to_string(),
                        start: 12,
                        end: 16,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R1".to_string(),
                    start: 22,
                    end: 28,
                    inner: vec![],
                },
            ],
        },
    };

    let t3 = Task {
        id: "T3".to_string(),
        prio: 2,
        deadline: 200,
        inter_arrival: 200,
        trace: Trace {
            id: "T3".to_string(),
            start: 0,
            end: 30,
            inner: vec![
                Trace {
                    id: "R2".to_string(),
                    start: 5,
                    end: 15,
                    inner: vec![Trace {
                        id: "R1".to_string(),
                        start: 10,
                        end: 12,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R3".to_string(),
                    start: 20,
                    end: 26,
                    inner: vec![],
                },
            ],
        },
    };
    
    let t4 = Task {
        id: "T4".to_string(),
        prio: 3,
        deadline: 150,
        inter_arrival: 150,
        trace: Trace {
            id: "T4".to_string(),
            start: 20,
            end: 50,
            inner: vec![Trace {
                id: "R2".to_string(),
                start: 30,
                end: 40,
                inner: vec![],
            }],
        },
    }; 

    let t5 = Task {
        id: "T5".to_string(),
        prio: 3,
        deadline: 150,
        inter_arrival: 150,
        trace: Trace {
            id: "T5".to_string(),
            start: 0,
            end: 30,
            inner: vec![Trace {
                id: "R3".to_string(),
                start: 10,
                end: 20,
                inner: vec![],
            }],
        },
    }; 

    // builds a vector of tasks t1, t2, t3, t4, t5
    let tasks: Tasks = vec![t1, t2, t3, t4, t5];


    // If you want to print  the entire tasks vector with one line.
    // println!("tasks {:?}", &tasks);
    // Seperate lines for tasks
    // print_tasks(&tasks);

    let (ip, tr, tt) = pre_analysis(&tasks);
    println!("Process priority: {:?}", ip);
    println!("Resources used by task: {:?}", tr);
    println!("Task times: {:?}", tt);

    println!("Total utilization {}", tot_util(&tasks));

    tasks_result(&tasks, &ip, &tr, &tt);

}

// A function that prints an informative vector for each task
// Task, Response, WCET, Blocking, Preemtion
// To change between approx/exact preemtion, change line 114 and 92 to "exact" or "approx"
fn tasks_result(tasks: &Vec<Task>, ip: &IdPrio, tr: &TaskResources, tt: &TaskTimes) {
    for t in tasks {
        let mut used_tasks: UsedTasks = HashSet::new();
        let mut seperate: UsedTasks = HashSet::new();
        let bl = blocking(&t, &ip, &tr);
        let pr = preemtion(&t, &ip, &tasks, &mut used_tasks, &tr, &tt, String::from("approx"));
        let rt = response(&t, &ip, &tasks, &mut seperate, &tr, &tt);
        let temp_vec = tt.get(&t.id);
        let worstcase = temp_vec.unwrap().get(1).unwrap();

        
        let vec = vec![
            InfoVec::Task(t.clone()),
            InfoVec::Float(rt),
            InfoVec::Int(*worstcase),
            InfoVec::Float(bl),
            InfoVec::Float(pr),
        ];  

        println!("Result vector: {:?}", vec);
    } 
}

// Calculates reponse time for a given task and checks that it meets its deadline, if not it throws error.
// To change preemtion between approx/exact, change line 114 to "exact" or "approx", as well as on line 92
fn response(task: &Task, ip: &IdPrio, tasks: &Vec<Task>, mut used_tasks: &mut UsedTasks, tr: &TaskResources, tt: &TaskTimes) -> f32 {
    let b = blocking(&task, &ip, &tr);
    let p = preemtion(&task, &ip, &tasks, &mut used_tasks, &tr, &tt, String::from("approx"));
    let temp_vec = tt.get(&task.id);
    let worstcase = temp_vec.unwrap().get(1).unwrap();
    let response_time = *worstcase as f32 + b + p;
    match error_checker::deadline_check(task.deadline, response_time) {
        Err(e) => panic!("{:?}", e),
        Ok(_response_time) => (),
    }
    response_time
}

// Calculates preemtion for tasks >= the prio of the given task, except itself.
// Exact version is based on the recursive equation 7.22 mentioned in the exam spec
fn preemtion(task: &Task, ip: &IdPrio, tasks: &Vec<Task>, mut used_tasks: &mut UsedTasks, tr: &TaskResources, tt: &TaskTimes, st: String) -> f32{
    let mut total_preemtion = 0.0;
    if st == "approx" {
        for t in tasks {
            if task.prio <= t.prio && task.id != t.id {
                let temp_vec = tt.get(&t.id);
                let arrival = temp_vec.unwrap().get(0).unwrap();
                let worstcase = temp_vec.unwrap().get(1).unwrap();
                let div_eq = task.deadline as f32 / *arrival as f32;
                total_preemtion += div_eq.ceil() * *worstcase as f32;
            } else {
                total_preemtion += 0.0;
            }
        }
    } else if st == "exact" {
        for t in tasks {
            if task.prio < t.prio && task.id != t.id {
                let temp_vec = tt.get(&t.id);
                let arrival = temp_vec.unwrap().get(0).unwrap();
                let worstcase = temp_vec.unwrap().get(1).unwrap();
                let resp = response(&t, &ip, &tasks, &mut used_tasks.clone(), &tr, &tt);
                let div_eq = resp / *arrival as f32;
                total_preemtion += div_eq.ceil() * *worstcase as f32;
            } else if task.prio == t.prio && task.id != t.id {
                if !used_tasks.contains(&t.id.to_string()) || used_tasks.is_empty() {
                    let temp_vec = tt.get(&t.id);
                    let arrival = temp_vec.unwrap().get(0).unwrap();
                    let worstcase = temp_vec.unwrap().get(1).unwrap();
                    used_tasks.insert(task.id.clone());
                    let resp = response(&t, &ip, &tasks, &mut used_tasks.clone(), &tr, &tt);
                    let div_eq = resp / *arrival as f32;
                    total_preemtion += div_eq.ceil() * *worstcase as f32;
                } else {
                    total_preemtion += 0.0;
                }
            } else {
                total_preemtion += 0.0;
            }
        }
    }
    total_preemtion 
}

// Calculates WCET blocking for a given task.
fn blocking(task: &Task, ip: &IdPrio, tr: &TaskResources) -> f32 {
    let task_res = tr.get(&task.id);
    let mut total_block = 0.0;
    if task_res.is_some() {
        for (key, val) in ip {
            if task.prio > *val {
                if tr.get(key).is_some() {
                    for (key_rs, _val_rs) in task_res.unwrap() {
                        for (key_ex, val_ex) in tr.get(key).unwrap() {
                            if (key_rs == key_ex) && (total_block < *val_ex as f32) {
                                total_block = *val_ex as f32;
                            }
                        }
                    }
                }
            }
        }
    }
    total_block
}

// Calculates CPU load for the different tasks and total CPU load, throws error if > 1
fn tot_util (tasks: &Vec<Task>) -> f32 {
    let mut tot = 0.0;
    for t in tasks {
        let util = (t.trace.end - t.trace.start) as f32 / t.inter_arrival as f32;
        tot += util;
        print!("{}", t.id);
        println!(" utilization ratio: {:?}", util);
    }
    match error_checker::load_factor(tot) {
        Err(e) => panic!("{:?}", e),
        Ok(_load) => (),
    }
    tot
}

// Prints each task on a seperate line.
fn print_tasks (tasks: &Vec<Task>) {
    let mut counter = 0;
    for t in tasks {
        print!("{}", t.id);
        println!(" {:?}", &tasks[counter]);
        counter += 1;
    }
}