use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use crate::data_processor::DataManager;
use crate::internal_lang::{FakeDatum, KeyType, OperationsLang};
use crate::thread_manager::ThreadManager;


/*
* Mocking up real world functionality before I start getting into parsing/command structure
*/

pub fn run_fake_web_server(n_threads: usize, fake_data: Vec<FakeDatum>, mut fake_commands: Vec<OperationsLang<FakeDatum>>) {
    let mut operations: VecDeque<OperationsLang<FakeDatum>> = Default::default();

    //Please tell me there's an easier way to copy between the two
    copy_vec_to_deque(&mut fake_commands, &mut operations);


    let mut datastore: Arc<Mutex<DataManager<FakeDatum>>> = Arc::new(Mutex::new(DataManager::new()));
    let mut threadpool: ThreadManager<FakeDatum> = ThreadManager::new(n_threads, &mut datastore);
    println!("Loading DB");
    for (i, d) in fake_data.iter().enumerate() {
        datastore.lock().unwrap().insert(&(i as KeyType), *d as FakeDatum);
    }
    println!("DB Loaded");
    'fakeCommandLoop: while let Some(cmd) = operations.pop_front() {
        if handle_command(&mut threadpool, cmd) {
            break 'fakeCommandLoop;
        }
    }

    threadpool.terminate();
}

fn copy_vec_to_deque(vec: &mut Vec<OperationsLang<FakeDatum>>, deque: &mut VecDeque<OperationsLang<FakeDatum>>) {
    while let Some(item) = vec.pop() {
        deque.push_front(item);
    }
}

fn handle_command<T: Clone + Send + Display>(
    threadpool: &mut ThreadManager<T>,
    cmd: OperationsLang<T>)
    -> bool {

    threadpool.schedule( cmd );
    false
}

// TODO: Implement Wrap and Schedule (wrap jobs in ARC and schedule them)

fn get_datum(datastore: &Arc<Mutex<DataManager<FakeDatum>>>, k: KeyType) -> Option<Arc<Mutex<FakeDatum>>> {
    let datum = {
        let mut db = datastore.lock().unwrap();
        let datum = db.get_reference(&k);
        datum
    };
    datum
}
