use crate::multithreading::job::Job;

pub enum Message {
    NewJob(Job),
    Terminate,
}