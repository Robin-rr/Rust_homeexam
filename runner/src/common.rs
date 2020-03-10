use std::collections::{HashMap, HashSet};

// Errorhandler
pub mod error_checker {
    #[derive(Debug)]

    // Error types
    pub enum ScheduleError {
        MissedDeadline,
        CPULoadFactorGTOne,
    }

    pub type ScheduleResult = Result<f32, ScheduleError>;

    pub fn load_factor(load: f32) -> ScheduleResult {
        if load > 1.0 {
            Err(ScheduleError::CPULoadFactorGTOne)
        } else {
            Ok(load)
        }
    }

    pub fn deadline_check(deadl: u32, resp: f32) -> ScheduleResult {
        if resp > deadl as f32 {
            Err(ScheduleError::MissedDeadline)
        } else {
            Ok(resp)
        }
    }
}

// common data structures
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub prio: u8,
    pub deadline: u32,
    pub inter_arrival: u32,
    pub trace: Trace,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub id: String,
    pub start: u32,
    pub end: u32,
    pub inner: Vec<Trace>,
}

#[derive(Debug)]
pub enum InfoVec {
    Task(Task),
    Float(f32),
    Int(u32),
}


// uselful types

// Our task set
pub type Tasks = Vec<Task>;

// A set of used tasks (used in preemtion recursion to prevent infinite use of tasks with same prio.)
pub type UsedTasks = HashSet<String>;

// A map from Task/Resource identifiers to priority
pub type IdPrio = HashMap<String, u8>;

// A map from Task identifiers to a set of Resource identifiers
// pub type TaskResources = HashMap<String, HashSet<String>>;
pub type TaskResources = HashMap<String, HashMap<String, u32>>;

// A map for wcet and interarrival per task.
pub type TaskTimes = HashMap<String, Vec<u32>>;


// Derives the above maps from a set of tasks
pub fn pre_analysis(tasks: &Tasks) -> (IdPrio, TaskResources, TaskTimes) {
    let mut ip = HashMap::new();
    let mut tr: TaskResources = HashMap::new();
    let mut tt: TaskTimes = HashMap::new();
    for t in tasks {
        update_prio(t.prio, &t.trace, &mut ip);
        update_tt(t.id.clone(), t.inter_arrival, &t.trace, &mut tt);
        for i in &t.trace.inner {
            update_tr(t.id.clone(), i, &mut tr);
        }
    }
    (ip, tr, tt)
}

// helper functions
fn update_tt(t_id: String, t_arrival: u32, trace: &Trace, tt: &mut TaskTimes) {
    let t_worstcase = trace.end - trace.start;
    let vec = vec![t_arrival, t_worstcase];
    tt.insert(t_id, vec);
}

fn update_prio(prio: u8, trace: &Trace, hm: &mut IdPrio) {
    if let Some(old_prio) = hm.get(&trace.id) {
        if prio > *old_prio {
            hm.insert(trace.id.clone(), prio);
        }
    } else {
        hm.insert(trace.id.clone(), prio);
    } /*
    for cs in &trace.inner {
        update_prio(prio, cs, hm);
    } */
}

fn update_tr(s: String, trace: &Trace, trmap: &mut TaskResources) {
    if let Some(seen) = trmap.get_mut(&s) {
        if seen.get(&trace.id).is_some() {
            if seen.get(&trace.id).unwrap() < &(trace.end - trace.start) {
                seen.insert(trace.id.clone(), trace.end.clone() - trace.start.clone());
            }
        } else {
            seen.insert(trace.id.clone(), trace.end.clone() - trace.start.clone());
        }
    } else {
        let mut hm = HashMap::new();
        hm.insert(trace.id.clone(), trace.end.clone() - trace.start.clone());
        trmap.insert(s.clone(), hm);
    }
    for trace in &trace.inner {
        update_tr(s.clone(), trace, trmap);
    }
}