//First implementation: Tom Schubert

/// STRATUM Protocol
/// 
/// Client                                Server
///|                                     |
///| --------- mining.subscribe -------> |
///| --------- mining.authorize -------> |
///|                                     |
///| <---------- mining.notify --------- |<--/
///|                                     |
///| ---------- mining.submit ---------> |
use std::net::{TcpStream};
use std::io::{BufRead,BufReader, Write};
use std::fmt::{self, Debug, Formatter};
use std::str;
use serde_json::{Value};
use super::*;
pub struct PoolConnection{
    pub username:String,
    pub workername:String,
    pub address:String,
    pub stream:TcpStream,
    pub active_job_queue:Vec<Job>,
    pub extranonce1: Vec<u8>,
    pub extranonce2_size: u32,
}

impl PoolConnection{
    pub fn new (username: &str ,address: &str, workername:&str)->Self
    {
        let stream = match TcpStream::connect(address) {
            Ok(stream) =>{stream}
            Err(e) => {panic!("Failed to connect: {}", e);}
        };
            return PoolConnection{
                    extranonce1:Vec::new(),
                    extranonce2_size:0,
                    username:username.to_owned(),
                    workername:workername.to_owned(),
                    address:address.to_owned(),
                    stream:stream,
                    active_job_queue:vec![],
                };
    }

    pub fn submit_share(& mut self,job_id:&String,extranonce2:Vec<u8>,ntime:&Vec<u8>,nonce:u32){
        let extranonce2_str=&hex::encode(extranonce2);
        let ntime_str=&hex::encode(ntime);
        let nonce_str=format!("{:x}",nonce);
        let message=r#"{"id": 4, "method": "mining.submit","params":[""#.to_owned()+&self.username+"."+&self.workername+r#"",""#+&job_id+r#"",""#+&extranonce2_str+r#"",""#+&ntime_str+r#"",""#+&nonce_str+r#""]}"#;
        let solution= message+"\n";
        self.send_message(solution);
    }

    fn send_message(& mut self, message:String){
       match self.stream.write(message.as_bytes()){
        Ok(stream) =>{stream}
        Err(e) => {panic!("Failed to connect: {}", e);}
       };
    }

    fn subscribe_pool(& mut self){
        let subscribe=concat!(r#"{"id": 1, "method": "mining.subscribe", "params":[]}"#,"\n");
        self.send_message(subscribe.to_owned());
        println!("Trying to subscribe to {}",self.address);
    }

    fn authorize_pool(& mut self){

        let message= r#"{"id": 2, "method": "mining.authorize", "params":[""#.to_owned()+&self.username+"."+&self.workername+r#"",""]}"#;
        let authorize= message+"\n";
        println!("Trying to authorize worker {}",self.workername);
        self.send_message(authorize);
    }

    fn create_jobs(& mut self,json: Value)->Job{

            let params=&json["params"];
            let job_id:String=justhex_symbols(params[0].to_string());
            let extranonce1=&self.extranonce1;
            let extranonce2=self.extranonce2_size;
            let prev_block_hash: Vec<u8>=hex::decode(justhex_symbols(params[1].to_string())).expect("Conversion failure");
            let coinb1: Vec<u8>=hex::decode(justhex_symbols(params[2].to_string())).expect("Conversion failure");
            let coinb2: Vec<u8>=hex::decode(justhex_symbols(params[3].to_string())).expect("Conversion failure");

            let mut merkle_branch: [Vec<u8>; 12] = [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
            for i in 0..merkle_branch.len()-1{

                if params[4][i].to_string()!="null"{
                    merkle_branch[i]=hex::decode(justhex_symbols(params[4][i].to_string())).expect("Conversion failure");
                }
            }
            let version:Vec<u8>=hex::decode(justhex_symbols(params[5].to_string())).expect("Conversion failure");
            let nbits:u32= u32::from_str_radix(&justhex_symbols(params[6].to_string()), 16).expect("Conversion failure");
            let ntime:Vec<u8>=hex::decode(justhex_symbols(params[7].to_string())).expect("Conversion failure");
            return Job::new(job_id,extranonce1.to_vec(),extranonce2,prev_block_hash,coinb1,coinb2,merkle_branch,version,nbits,ntime);
    }

    pub fn handle_datastream(& mut self){

        self.subscribe_pool();
        self.authorize_pool();

        loop{

        let mut reader = BufReader::new(&self.stream);
        let mut data: Vec<u8>= Vec::new();

        match reader.read_until(b'\n',&mut data) {
            Ok(_) => {

                let message = str::from_utf8(&data).expect("Could not convert to String");
                let json: Value =match serde_json::from_str(message){
                    Ok(json) => json,
                    Err(_) => continue,
                };
                
                if json["id"]==1{
                    self.extranonce2_size=u32::from_str_radix(&json["result"][2].to_string(), 16).expect("Conversion failure");
                    self.extranonce1=hex::decode(justhex_symbols(json["result"][1].to_string())).expect("Conversion failure");
                    println!("Succesfully subscribed to {}",self.address);
                }
                if json["id"]==2{
                     if json["result"]==true{
                        println!("{}.{} succesfully authorized on Pool",self.username,self.workername);
                     }
                     else{
                        panic!("Pool is not accepting Connection for worker Check Username")
                     }
                     
                 }

                if json["method"]=="mining.notify"{

                    if json["params"][8]==true{
                        println!("Reset active Job stack!");
                        self.active_job_queue=Vec::new();
                    }

                    else{

                    let new_job:Job=self.create_jobs(json);
                    println!("New Mining Job: {:?}",new_job);

                    self.active_job_queue.push(new_job);
                    
                    };

                    if self.active_job_queue.len()>0{
                        let job_to_mine=self.active_job_queue[self.active_job_queue.len()-1].clone();
                        println!("Start Working on Puzzle for Job {}",job_to_mine.job_id);
                        let result= miner::start_miner(&job_to_mine);
                        println!("Finished Puzzle");
                        if result.is_some() {
                            println!("Solution found submitting");
                                let mining_input = result.unwrap();
                                let ntime=&job_to_mine.ntime;
                                let job_id=&job_to_mine.job_id;
                                println!("Found solution nonce: {}",mining_input.0);

                                println!("{:?}",job_to_mine);
                                self.submit_share(job_id,mining_input.1,ntime,mining_input.0);
                        }
                        else{
                            println!("No Solution found for Job");
                        }

                        let index = self.active_job_queue.iter().position(|x| *x == job_to_mine).unwrap();
                        self.active_job_queue.remove(index);
                    }
                    
                };
                println!("{:?}",self);
                
            },
            Err(e) => {
                println!("Failed to Read Data {}",e);
                continue;
            }
        }
        
    }
}
}

impl Debug for PoolConnection {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "PoolConnection: {} connected to {} with {} active job(s) in queue ",
            &self.username,
            &self.address,
            &self.active_job_queue.len(),
        )
    }
}
