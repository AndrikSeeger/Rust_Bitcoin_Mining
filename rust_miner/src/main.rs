use lib::*;
// Startet das mining
fn main(){
    // Input1 Accountname von Slushpool
    // Slushpooladresse
    // Workername kann beliebig sein
    let mut conn=PoolConnection::new("ITA_Miner","eu.stratum.slushpool.com:3333","worker1");
    // Eigentlich sollte asynchron mit mehrern Threads gearbeitet werden aber zu wenig ZEit
    conn.handle_datastream();
}