# flushreload
Computer Security FlushReload.c in rust. 
Taken from figure 13.2 in https://www.handsonsecurity.net/. 

Basically a homework assignment. 

Rust complains a lot if you try to create a mutable global, which is the central data structure in the original program. 

The master branch contains a version based on a `Box`ed  local. 
The `static_data` contains a version with a mutex wrapped static buffer.
