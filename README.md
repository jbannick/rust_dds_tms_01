# rust_dds_tms_01
This app is a prototype Tactical Microgrid Standard implementation written in Rust.  

It publishes TMS Heartbeats from a TMS-enabled Device, such as a Generator (genset)  
and receives those messages at a TMS-enabled Dashboard.  

It uses an Object Management Group, Data Distribution Service library implemented  
in pure Rust (no C or C++) by Atostek Oy, a Finnish company.  
See: https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=285095  
See: https://www.omg.org/omg-dds-portal/  
See: https://github.com/jhelovuo/RustDDS  

Usage:  
  cargo run -- --servertype [pub | sub]  

How to run:  
 
On your Linux, Windows, or MacOS computer  
In Terminal Window One  

Step 1. Go to the directory containing the Cargo.toml file for this app.  
Step 2. Execute: cargo run -- --servertype sub  
        Should start this app as a TMS Subscriber and wait for connections  
 
In Terminal Window Two  
 
Step 3. Repeat Steps 1. 2 above except execute: cargo run -- --servertype pub  
        Should start this app as a TMS Publisher, connect to all Subscribers,  
        and begin publishing TMS Heartbeat messages one per second.  
        All TMS Subscribers should receive and display those messages.  

You can have multiple Publishers and multiple Subscribers, one Terminal window each.  
You can stop and restart Publishers and Subscribers.  
You can start and stop Publishers and Subscribers in any order.  
