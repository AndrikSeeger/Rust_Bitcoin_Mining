use lib::*;
fn main(){
    let mut conn=PoolConnection::new("ITA_Miner","eu.stratum.slushpool.com:3333","worker1");
    conn.handle_datastream();
}
