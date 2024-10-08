# Tactical Microgrid Prototype in Rust
This app is a prototype Tactical Microgrid Standard (**TMS**) implementation written in Rust.  

TMS enables interoperability of hardware and software necessary to operate an electrical generation and distribution microgrid.  

TMS was developed for military use on the battlefield, but could be applied to civilian electric grids as well.  

See: **[MIL-STD-3071 Tactical Microgrid Communications and Control](https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=285095)**

This prototype publishes TMS Heartbeats from a TMS-enabled Device, such as a Generator (genset) and receives those messages at a subscribing TMS-enabled Dashboard. All messaging is via localhost. Messages are published to the `TMS_Heartbeat` DDS topic.

This app uses an Object Management Group (**OMG**) , Data Distribution Service (**DDS**) open source library implemented in pure Rust (no C or C++) by **[Atostek Oy](https://atostek.com/en/company/#yhteys)**, a Finnish company.  

See: **[OMG Data Distribution Service Portal](https://www.omg.org/omg-dds-portal/)**  


See: **[GitHub repo for RustDDS implementation](https://github.com/jhelovuo/RustDDS)** 

The advantage of using Rust is that Rust provides greater protections from cyber-attack than other common programming languages.  

See: **[Rust Software Security: A Current State Assessment. Carnegie Mellon University, 2022](https://insights.sei.cmu.edu/blog/rust-software-security-a-current-state-assessment/)**  

Usage:  
  `cargo run -- --servertype [pub | sub]`  

Note: The extra `--` are required by Cargo to parse the app's arguments.  

### Before you run this app:
On your Linux, Windows, or MacOS computer  

Step 1. Install Rust on your computer.  
See: https://www.rust-lang.org/tools/install.  

Step 2. Install Git on your computer.  
See: https://git-scm.com/downloads

Step 3. On your computer, create a working directory.  

Step 4. In a Terminal Window, CD to that working directory.  

Step 5. Clone this repo to that working directory.

Enter on the command line:  
`git clone https://github.com/jbannick/rust_dds_tms_01.git`  

Step 6. Build the app. 

In the same directory as the cargo.toml file:  
Enter:  
`cargo build`  

You should see cargo downloading its dependencies and building the app.  

### How to run this app:  

Note: This single app runs as either a Publisher or Subscriber server.  You run it in two Terminal Windows on the same computer.  
 
On your Linux, Windows, or MacOS computer  

Step 1. Open and go to Terminal Window One  

Step 2. In Terminal Window One: 

Go to the directory containing the `Cargo.toml` file for this app. 
  
Step 3. Start the Subscriber server. 

Enter:
`cargo run -- --servertype sub`  

Should start this app as a TMS Subscriber and wait for connections  

![Subscriber](TmsServerSub.png "Subscriber")

Step 4. Open and go to Terminal Window Two  

Step 5. In Terminal Window Two  

Repeat Steps 1, 2, 3 above  

Except enter: `cargo run -- --servertype` **pub**    

Should start this app as a TMS Publisher, connect to all Subscribers, and begin publishing TMS Heartbeat messages one per second.  

![Publisher](TmsServerPub.png "Publisher")        
        
* You can have multiple Publishers and multiple Subscribers, one Terminal window each.  
* You can stop and restart Publishers and Subscribers.  
* You can start and stop Publishers and Subscribers in any order.  

All TMS Dashboards should receive and display TMS Heartbeats from all TMS publishing Devices. 
### Validation:
This app has been tested on:

* Linux: Ubuntu 22.04 86x64  
* Windows 11 86x64 Intel i9  
* MacOS 13 Intel i5

Note: The startup message:  
"rustdds::network::udp_listener - UDPListener::new_multicast() not implemented for IpV6"  
is not a failure condition. It simply notifies that this implementation of DDS does not support ipV6.

### Beyond:  

This is a "Hello World!" app for TMS in Rust.  

It does not implement major DDS features available in the Rust DDS implementation, including:  

* Message encryption  
* Signed DDS configuration files  
* Quality of service parameters  
* Communication outside of localhost  

It does:

* Prove that a pure Rust TMS microgrid can be implemented  
* Provide a template for such implementation  

TMS includes almost 200 objects.  

MIL-STD-3071 contains an Appendix that defines each of those objects, and their interactions, in a manner and in detail sufficient to build a production TMS microgrid.
