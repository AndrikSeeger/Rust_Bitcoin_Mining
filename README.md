<!--Copyright Andrik Seeger 2022-->

# Bitcoin Mining implemented in Rust

First a TCP-connection to a mining-pool using the second stratum-protocol will be created. For testing purposes we used the included <a href="https://github.com/AndrikSeeger/Rust_Bitcoin_Mining/blob/main/Pool_Test_Access" target="_blank">login data</a>. After the connection is build up the pool data will be received. This data gets converted into different mining jobs. Using my own implementation of the Bitcoin mining algorithm a hashing-solution will be searched for by changing the Nonce and Extranonce and double-hashing the result. As soon as a valid solution is found the result gets send back to the server of the mining pool via the established TCP-connection.

## Additional Information
This project is a Proof of Concept and not a finalized product. Since this project focuses on using the CPU rather than the GPU the hashing efficiency is compromised. 

## FAQ
* **Can I use this miner to generate income?**

    Theoretically yes, practically no. Since no GPU implementation is used the hashing power is *less than 0.01%* of a regular mining rig with an array of GPUs. Hence it's **very unlikely** to find the solution to the hashing problem before anyone else. But you certainly should feel free to thoroughly test the program. Maybe you're lucky and find the mining solution before anyone else.  
    
* **Can I use your login data?**

    Sure, as long it's used for testing this application.

## Contributors
* Andrik Seeger
* Tom Schubert
