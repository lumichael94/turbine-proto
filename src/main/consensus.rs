// use std::os;
// use std::sync;

// use network::{server, proto};
use data::{account, state, database, log, tenv};
use vm::env;
use std::time::Duration;
use std::thread;
// use util::{helper, krypto};
// use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
// use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

//====================================================================
// GENERAL CONSENSUS FUNCTIONS
//====================================================================

// Main consensus function
// pub fn consensus_loop(from_threads: Receiver<String>, arc:  Arc<Mutex<HashMap<String, Sender<String>>>>){
pub fn consensus_loop(local_stat: Arc<RwLock<(String, String)>>,
tenv_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::log>>>){
    let conn = database::connect_db();
    // Listening Loop
    loop{
            let te = tenv_stat.clone();
            let marc = local_stat.clone();
            let curr_state = state::get_current_state(&conn);
            set_main_stat(marc, "LISTENING".to_string(), curr_state.hash.clone());

            let larc = curr_logs.clone();
            let lmap = (*larc).read().unwrap();
            if lmap.len() == 0 {
                continue;
            }
            if should_commit(te.clone()){
                let poss_logs = curr_logs.clone();
                let poss_state = env::execute_state(poss_logs);
                state::save_state(&poss_state, &conn);
                // State has been committed. Logs are cleared.
                curr_logs.write().unwrap().clear();
            } else if should_propose(te){
                println!("=>> Proposal Phase for State {:?}", curr_state.hash);
                // Setting local status to proposing.
                let m_arc = local_stat.clone();
                let curr_state = state::get_current_state(&conn);
                set_main_stat(m_arc, "PROPOSING".to_string(), curr_state.hash);
                // Proposal Loop
                loop {
                    let tenvs = tenv_stat.clone();
                    if should_commit(tenvs){
                        let poss_logs = curr_logs.clone();
                        let poss_state: state::state = env::execute_state(poss_logs);
                        let main_arc = local_stat.clone();

                        state::save_state(&poss_state, &conn);

                        let curr_state = state::get_current_state(&conn);
                        set_main_stat(main_arc, "COMMITTED".to_string(), curr_state.hash);
                        // // Waiting for the network to synchronize
                        while !should_listen(tenv_stat.clone()){
                            thread::sleep(Duration::from_millis(500));
                        }
                    }
                }
        }
    }
    // TODO: Makes this reachable.
    // database::close_db(conn);
}

pub fn set_main_stat(local_stat: Arc<RwLock<(String, String)>>, status:String, state:String){
    let marc = local_stat.clone();
    let mut m_tup = marc.write().unwrap();
    *m_tup = (status, state);
}

//====================================================================
// LOCAL NODE PHASE FUNCTIONS
//====================================================================

// Cycles through connected nodes to set update local node status: "listening", "proposing", "committed".
// "listening": Node is accepting all ongoing logs and accounts of a state.
// "proposing": Node is proposing set of logs and accounts. It will only allow modifications
//              if more than 80% of trusted node list form a consensus on accepting a log
//              or account.
// "committed": Node has computed, verified and committed a state from the logs and accounts.
// Status only applies if node is working on its most current state. Otherwise, it is perpetually
// listening.

//** Pre-phase Checking **//
//TODO: Condense into single functions

//Checking if the local node should listen
pub fn should_listen(tenv_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>)-> bool{
    // Determine majority state, count
    let arc = tenv_stat.clone();
    let tenvs = arc.read().unwrap();
    let h_map = (*tenvs).clone();

    let mut counter = 0 as i32;
    let size = tenvs.len() as i32;
    for (_, te) in h_map {
        if te.t_stat != "SYNCED".to_string(){
            counter+=1;

        }
   }
    let threshold = 0.8 as i32;
    if size == 0 {return false};
    let percentage = counter / size;
    if percentage > threshold {return true};
    return false;
}

// Checking if the local node should propose
pub fn should_propose(tenv_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>) -> bool{
    // Determine majority state, count
    let arc = tenv_stat.clone();
    let tenvs = arc.read().unwrap();
    let h_map = (*tenvs).clone();
    //drop(tenvs); // Little memory managementf
    let mut counter = 0 as i32;
    let size = h_map.len() as i32;

    for (_, te) in h_map {
        if te.t_stat == "SYNCED".to_string(){
            counter+=1;
        }
   }
    // If counter is more than 70% of trusted nodes and node state is equal to
    // the current state, then local phase = "proposing"
    // let n_nodes = nodes_stat.read().unwrap().len() as i32;
    let threshold = 0.8 as i32;
    if size == 0 {return false};
    let percentage = counter / size;
    if percentage > threshold {
        return true
    };
    return false;
}

// Checking if the local node should commit the current state
// Broadcasts state with peers and determine state reward distribution.
pub fn should_commit(thread_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>) -> bool{
    // Determine majority state, count
    let arc = thread_stat.clone();
    let tenv = arc.read().unwrap();
    let h_map = tenv.clone();
    let mut counter = 0 as i32;
    let size = tenv.len().clone() as i32;
    drop(tenv);
    for (_, te) in h_map {
        if (te.t_stat == "READY") | (te.t_stat == "SYNCED") {
            counter+=1;
        }
   }
    let threshold = 0.8 as i32;
    if size == 0 {return false};
    let percentage = counter / size;
    if percentage > threshold {
        return true
    };
    return false;
}

// Checking if the local node should execute the current logs and accounts
pub fn should_execute(nodes_stat: Arc<RwLock<HashMap<String, tenv::tenv>>>) -> bool{
    // Determine majority state, count
    let arc = nodes_stat.clone();
    let nodes = arc.read().unwrap();
    let h_map = nodes.clone();

    let mut counter = 0 as i32;
    let size = nodes.len() as i32;

    for (_, te) in h_map {
        if te.t_stat == "EXECUTE"{
            counter+=1;
        }
   }
    let threshold = 0.7 as i32;
    let percentage = counter / size;
    if percentage > threshold {return true};
    return false;
}
