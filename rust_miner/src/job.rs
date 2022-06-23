// imports
use std::fmt::{self, Debug, Formatter};
use std::cmp::PartialEq;

// Struct der einen Puzzle Job repräsentiert
#[derive(Clone,PartialEq)]
pub struct Job{
    pub job_id: String,
    pub extranonce1: Vec<u8>,
    pub extranonce2: u32,
    pub prev_block_hash: Vec<u8>,
    pub coinb1: Vec<u8>,
    pub coinb2: Vec<u8>,
    pub merkle_branch:[Vec<u8>;12],
    pub version:Vec<u8>,
    pub nbits:u32,
    pub ntime:Vec<u8>,
}

// Own debug trait
impl Debug for Job {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Job {}: with prev_block_hash {} ...",
            self.job_id,
            &hex::encode(&self.prev_block_hash),
        )
    }
}
impl Job{
    // Konstruktor
     pub fn new (job_id:String,extranonce1: Vec<u8>, extranonce2: u32, prev_block_hash: Vec<u8>, coinb1: Vec<u8>, coinb2: Vec<u8>,
    merkle_branch:[Vec<u8>;12], version:Vec<u8>, nbits:u32, ntime:Vec<u8>)->Self{
            Job{
                 job_id:job_id,
                 extranonce1,
                 extranonce2,
                 prev_block_hash,
                 coinb1,
                 coinb2,
                 merkle_branch,
                 version,
                 nbits,
                 ntime,
            }
        }
}
