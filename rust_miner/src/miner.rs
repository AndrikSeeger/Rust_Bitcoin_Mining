use std::iter::Iterator;
use crypto_hash::{Algorithm, digest};
use std::{fmt, num::ParseIntError};
use num_bigint::BigUint;
use std::ops::Shl;
use std::time::SystemTime;
use super::*;
//Constant for conversion to uint512
pub fn conv_add()->Vec<u8>
{
    vec![0,0,0,128,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128,2,0,0]
}

//Builds an Extranonce of specified length, consisting of only 0
pub fn extranonce2(length: &u32) -> Vec<u8>
{
    let mut extranonce2 = vec![];
    for _i in 0..*length/2
    {
        extranonce2.extend(vec![0]);
    }
    extranonce2
}

//Builds Coinbase out of Single Elements
pub fn build_coinbase(coinb1: &Vec<u8>, coinb2: &Vec<u8>, extranonce1: &Vec<u8>, extranonce2: &Vec<u8>) -> Vec<u8>
{
    let mut coinbase = vec![];
    coinbase.extend(coinb1);
    coinbase.extend(coinb2);
    coinbase.extend(extranonce1);
    coinbase.extend(extranonce2);
    doublesha(&coinbase)
}

//Calculate merkle root out of coinbase and given merkle branches
pub fn build_root(branches: &[Vec<u8>], coinbase: &Vec<u8>) -> Vec<u8>
{
    let mut root = coinbase.to_vec();
    for i in 0..branches.len() 
    {   
        if !branches[i].is_empty(){
            root.extend(&branches[i]);
            root = doublesha(&root);
        }
       
    }
    return revec(&root);
}

//Build Blockheader out of single Elements
pub fn build_header(version: &Vec<u8>, prevhash: &Vec<u8>, merkle_root: &Vec<u8>, ntime: &Vec<u8>, nbits: &Vec<u8>, nonce: &u32)->Vec<u8>
{
    let mut header = vec![];
    header.extend(prevhash);
    header.extend(merkle_root);
    header.extend(ntime);
    header.extend(nbits);
    header.extend(u32_u8(&nonce));
    header.extend(conv_add());
    return header;
}

//Convert Hexstring to u8
fn strhex_to_u8(hex: &str) -> u8 
{
    let res = u8::from_str_radix(hex, 16); 
    match res {
        Ok(v) => v,
        Err(e) => panic!("Conversion hex to u8 failed: {}", e),
    }
}

//Perform Double-Sha256-Algorithm
pub fn doublesha(prehash: &Vec<u8>)->Vec<u8>{
    return crypto_hash::digest(crypto_hash::Algorithm::SHA256, &crypto_hash::digest(crypto_hash::Algorithm::SHA256, &prehash));
}

//Reverse Vector of Type u8
pub fn revec(vec: &Vec<u8>)->Vec<u8>{
    let v = vec.to_vec();
    return v.into_iter().rev().collect();
}

//Convert u32 to Vector of Type u8
pub fn u32_u8 (u: &u32) -> Vec<u8> {
    vec![
    (u >> 0) as u8,
    (u >> 8) as u8,
    (u >> 16) as u8,
    (u >> 24) as u8,]
}

//Calculate Difficulty -> Target threshold under which the hash is accepted. Very optimized.
//See: https://medium.com/@dongha.sohn/bitcoin-6-target-and-difficulty-ee3bc9cc5962
pub fn calc_diff(nbit: &u32)->Vec<u8>
{
    let index = 8*((nbit >> 24)-3); 
    let coeff: u32 = nbit & 0x00FFFFFF; //Mask to get the first 24 Bit
    let target = BigUint::from(coeff).shl(index);
    target.to_bytes_le() 
}

//Mining Process
//See: https://braiins.com/stratum-v1/docs (additional: http://www.righto.com/2014/02/bitcoin-mining-hard-way-algorithms.html)
pub fn mine(extranonce1: &Vec<u8>, extranonce2_length: &u32, prevhash: &Vec<u8>, coinb1: &Vec<u8>, coinb2: &Vec<u8>, merkle_branches: &[Vec<u8>], version: &Vec<u8>, nbits: &u32, ntime: &Vec<u8>)->Option<(u32,Vec<u8>)>
{
    let nbits_vec = u32_u8(nbits);
    let extranonce2=extranonce2(extranonce2_length);
    let coinbase = build_coinbase(&coinb1, &coinb2, &extranonce1,&extranonce2);
    let merkle_root = build_root(&merkle_branches, &coinbase);
    let target = calc_diff(nbits);
    let mut nonce = 0;
    let begin = SystemTime::now();
    // max FFFFFFFF
    while nonce < 0xFFFFFF //Limit
    {
        let header = doublesha(&build_header(&version, &prevhash, &merkle_root, &ntime, &nbits_vec, &nonce));
        if compare_headers(&header, &target) //Check if result is a solution
        {
            println!("Jackpot Found valid share, lucky number: {}", nonce);
            return Some((nonce,extranonce2));
        }
        nonce = nonce + 1;
    }
    let duration = begin.elapsed().unwrap().as_secs_f32();
    println!("Hashrate was: {}GH/s", (nonce as f32/duration)/1000000000.0);
    return None;
}

//Compare Header to check if result is an acceptable solution
pub fn compare_headers(header: &Vec<u8>, target: &Vec<u8>)->bool
{
    //Header has to be smaller than Target in Decimal Comparison. Pseudo: header < target == true
    let diff = header.len() - target.len();
    let mut result = false;
    if(header.starts_with(&vec![0; diff]))  //Number of Bytes which Header is longer than Target have to be completely 0 otherwise header>target
    {
        result = true;
        for i in (0..target.len()).rev() //For remaining (unchecked Bytes)
        {
            if(header[i]<target[target.len()-i]) //Move Bytewise through Target and Header and check that every Byte of Header is Smaller than according Target-Byte
            {
                result = false;
                break;
            }
        }
    }
    return result;
}

//Convert Hexstring to Vector of u8
pub fn extract_u8(convert: &str)->Vec<u8>
{
    let mut result: Vec<u8> = vec![];
    for i in 0..(convert.chars().count()/2)
    {
        result.push(strhex_to_u8(&convert[2*i..2*(i+1)])); //Convert two Hex-Digits (one byte) and add them
    }
    return result;
}

//Convert Hexstring to u32
pub fn extract_u32(convert: &str)->u32
{
    let res = u32::from_str_radix(convert, 16);
    match res {
        Ok(v) => v,
        Err(e) => panic!("Conversion hex to u32 failed: {}", e),
    }
}

//Example Mining-Process with real values gathered from the Slush-Bitcoin-Pool via our own Stratum-Connection
/*pub fn test_miner()
{
    let ex1 = extract_u8("2f650800b66d40");
    let ex2_l: u32 = 8;
    let job_id = "22187d70f";
    let phash = extract_u8("46dc83b487c8b6bbdd456f3a5be6dbfb9f05e6c0000524c80000000000000000");
    let coinb1 = extract_u8("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4b03bb6d0afabe6d6dace18894c4f05903ad5014ded3e8e7f6cc8aaefdecb72b93e5a4ce8e41633cb00100000000000000");
    let coinb2 = extract_u8("0fd78721022f736c7573682f00000000035077ec28000000001976a9147c154ed1dc59609e3d26abb2df2ea3d587cd8c4188ac00000000000000002c6a4c2952534b424c4f434b3ab94a2dea82d9a758eed5cbc128792ebc878a10fc86b316e87219fe22003313710000000000000000266a24aa21a9ed9edd94a5707777af0eb2c7857035ddb39e95ed9a1d5338b26c77e704e7e7f79f00000000");
    let m_branch: [&str; 11] = ["9842754b2d45f1f6565acdc09166a141b930398a8e9869bcad8a8051137ee10d","93d119c3f778a310cbdb58de328699b9c07bf49462b3e0a642695a3aafb7fcdd","7f5f6f4221e34f945d97a28d3aa5c0038bc0dc6001d617d61854beb8a8587be9","b109fad673bdc695a92420a55727446fd085091feab8de9ed34b4b877e726120","1680077f50cc95ba34930cd13abebeb800b6aee727b6f43d3b8896423a69cd35","4a7ecec6112e7daaa28916754977b600a67ae115fb62a32c866116d0fd1a9d1e","9205d7fc38b856bc580e942d23a1f9d085582609c25608356f148ff2ad0eaedf","9bca39ed258050d81ec4d8f08f33e33484c644b80dbeb97ad1781a3b79227c4a","78db32d49b5d8a8f1ba91d4889038c0a9328de0371f02d3acb5a33e5b0012056","3a517d1fc8dfa1bb84682525437783a9833fd632a8a80301da4fe2da3eafd233","cd96abb880c4ef6c595d237c6ac27babb918143a8a3039d7897be1dd1496729d"];
    let mut m_branch_u8: [Vec<u8>; 11] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    for i in 0..m_branch.len() //Convert whole array 
    {
        m_branch_u8[i] = extract_u8(&m_branch[i]);
    }
    let vers = extract_u8("20000004");
    let nbits = extract_u32("170b3ce9");
    let ntime = extract_u8("609d3434");
    let clean_jobs = true;
    let _n = mine(&ex1, &ex2_l, &revec(&phash), &coinb1, &coinb2, &m_branch_u8, &vers, &nbits, &ntime);
}*/
pub fn start_miner(job:&Job)->Option<(u32,Vec<u8>)>
{
  return mine(&job.extranonce1, &job.extranonce2, &revec(&job.prev_block_hash), &job.coinb1, &job.coinb2, &job.merkle_branch, &job.version, &job.nbits, &job.ntime);
 
}