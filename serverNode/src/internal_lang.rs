/*
* GOAL: Internal implementation of the simply typed lambda calculus, with extensions
* Will be extended, optimized to work with database
*/

/*
* CURRENT STATUS:
* Basic Read, write commands
*/
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use crate::data_processor::DataManager;
use OperationsLang::{*};
use crate::internal_lang::OperationsTypes::{ErrorVal, Triv, BasicVal};

pub type FakeDatum = u64;

pub type KeyType = usize;


#[derive(Clone)]
pub enum OperationsLang<T: Clone + Send + Display> {
    // Imperative Ops
    Get(KeyType),
    Set(KeyType, T)
}

#[derive(Clone)]
pub enum OperationsTypes<T: Clone + Send + Display> {
    BasicVal(T),
    Triv,
    ErrorVal
}


/*
* Executes task defined by Internal Lang. Returns true if successful, false otherwise
*/
pub fn execute <T: Clone + Send + Display> (task: OperationsLang<T>, db: &mut Arc<Mutex<DataManager<T>>>) -> OperationsTypes<T> {
    return match task {
        Get(key) => {
            let object = db.lock().unwrap().get_reference(&key);
            match object {
                None => {
                    ErrorVal
                }
                Some(d) => {
                    let data = d.lock().unwrap().clone();
                    println!("GET (KEY {key}) => {data}");

                    BasicVal(data)
                }
            }
        }
        Set(key, value) => {
            let object = db.lock().unwrap().get_reference(&key);
            match object {
                None => {
                    db.lock().unwrap().insert(&key, value);
                    Triv
                }
                Some(d) => {
                    *(d.lock().unwrap()) = value;
                    Triv
                }
            }
        }
    }
}