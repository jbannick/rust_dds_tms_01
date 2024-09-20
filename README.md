# Tactical Microgrid Prototype in Rust
This app is a prototype Tactical Microgrid Standard (**TMS**) implementation written in Rust.  

TMS enables interoperability of hardware and software necessary to operate an electrical generation and distribution microgrid.  

TMS was developed for military use on the battlefield, but could be applied to civilian electric grids as well.  


This prototype publishes TMS Heartbeats from a TMS-enabled Device, such as a Generator (genset) and receives those messages at a subscribing TMS-enabled Dashboard. All messaging is via localhost. Messages are published to the `TMS_Heartbeat` DDS topic.

This app uses an Object Management Group (**OMG**) , Data Distribution Service (**DDS**) open source library implemented in pure Rust (no C or C++) by **Atostek Oy**, a Finnish company.  

The advantage of using Rust is that Rust provides greater protections from cyber-attack than other common programming languages.  

See: https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=285095  
See: https://www.omg.org/omg-dds-portal/  
See: https://github.com/jhelovuo/RustDDS  
See: https://insights.sei.cmu.edu/blog/rust-software-security-a-current-state-assessment/  

Usage:  
  `cargo run -- --servertype [pub | sub]`  

Note: The extra `--` are required by Cargo to parse the app's arguments.  

### Before you run this app:
On your Linux, Windows, or MacOS computer  

Step 1. Install Rust on your computer.  
See: https://www.rust-lang.org/tools/install.  

Step 2. Install Git to your computer.  
See: https://git-scm.com/downloads

Step 3. On your computer, create a working direcory.  

Step 4. In a Terminal Window, CD to that working directory.  

Step 5. Clone this repo to that working directory.

Enter on the command line:  
git clone https://github.com/jbannick/rust_dds_tms_01.git. 

### How to run this app:  
 
On your Linux, Windows, or MacOS computer  

Step 1. Open and go to Terminal Window One  

Step 2. In Terminal Window One: 

Go to the directory containing the `Cargo.toml` file for this app. 
  
Step 3. Execute: `cargo run -- --servertype sub`  
        Should start this app as a TMS Subscriber and wait for connections  

Step 4. Open and go to Terminal Window Two  
 
Step 5. In Terminal Window Two  

Repeat Steps 1. 2 above except execute: `cargo run -- --servertype` **pub**    
Should start this app as a TMS Publisher, connect to all Subscribers, and begin publishing TMS Heartbeat messages one per second.  
        
All TMS Subscribers should receive and display those messages.  

* You can have multiple Publishers and multiple Subscribers, one Terminal window each.  
* You can stop and restart Publishers and Subscribers.  
* You can start and stop Publishers and Subscribers in any order.  
