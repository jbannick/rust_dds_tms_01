#![allow(unused)]
use std::{io, time};
use std::fmt;

use rustdds::*;
use serde::{Serialize, Deserialize};

use log4rs::{
  append::console::ConsoleAppender,
  config::{Appender, Root},
  Config,
};
use log::LevelFilter;
use futures::StreamExt;
use smol::Timer;

use log::{debug, error, info, trace, warn};

use structopt::StructOpt;
use strum::VariantNames;

use uuid::Uuid;

const SECOND: time::Duration = time::Duration::from_millis(1000);

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long, possible_values = ServerType::VARIANTS)]
    servertype: ServerType,
}

#[derive(Debug, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum ServerType {
    Pub,
    Sub,
}

impl fmt::Display for ServerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerType::Pub => write!(f, "pub"),
            ServerType::Sub => write!(f, "sub"),
        }
    }
}

fn main() {
  println!("===========================================================================");
  println!("===                                                                     ===");
  println!("===                    RUST DDS TMS SERVER 1.0                          ===");
  println!("===                                                                     ===");
  println!("===========================================================================");
  
  configure_logging();
  info!("Logging is configured");
  
  let args = Args::from_args();
  debug!("{:?}", args);
  debug!("{:?}", args.servertype);
    
  let arg = args.servertype;
  debug!("{:?}", arg);
    
  let server_type = arg.to_string();   
  debug!("server_type = {:?}", server_type);
  
  let my_device_uuid = Uuid::new_v4();
  println!("TMS DEVICE ID = {}", my_device_uuid.simple().to_string());

  println!(""); // to visually separate ipV6 ERROR msgs
  
  // Create a DDS DomainParticipant
  let domain_participant = DomainParticipant::new(0).unwrap();
  info!("Created the DomainParticipant");

  println!("");
  
  // Create a DDS Quality of Service
  let qos = QosPolicyBuilder::new()
    .reliability(policy::Reliability::Reliable {max_blocking_time: rustdds::Duration::ZERO})
    .build();
  info!("Created the Quality of Service");
  
  // Create the DDS Topic
  let heartbeat_topic = domain_participant.create_topic("heartbeat_topic".to_string(), "TMS_Heartbeat".to_string(), &qos, TopicKind::NoKey).unwrap();
  println!("DDS TMS TOPIC = {:?}", heartbeat_topic.get_type().name()); 
  debug!("{:?}", heartbeat_topic);
  
  // Create a DDS Subscriber 
  let tms_dashboard = domain_participant.create_subscriber(&qos).unwrap();
  info!("Created the DDS TMS Dashboard");
  
  // Create a DDS Publisher
  let tms_device = domain_participant.create_publisher(&qos).unwrap();
  info!("Created the DDS TMS Device");
    
  #[derive(Serialize, Deserialize, Debug)]
  struct TmsHeartbeat {
    device_id: String,
    sequence_number: u32,
  }
  
  // ---
   
  if "sub" == server_type {
    println!("\nSTARTING TMS DASHBOARD - TMS deviceId = {:?}\n", my_device_uuid.simple().to_string());
    
    let reader = tms_dashboard
    .create_datareader_no_key::<TmsHeartbeat, CDRDeserializerAdapter<TmsHeartbeat>>(
      &heartbeat_topic,
      None)
    .unwrap();
  info!("Created the DDS DataReader");
  
    info!("-- Start TMS Dashboard message listening --");
    
    smol::block_on(async {
      let mut datareader_stream = reader.async_sample_stream();
      let mut datareader_event_stream = datareader_stream.async_event_stream();

      loop {
        futures::select! {
          r=datareader_stream.select_next_some()=>{
            match r{
              Ok(d)=>{println!("Received TMS Heartbeat:  devicdId {}, sequenceNumber {}",d.device_id, d.sequence_number)},
              Err(e)=> {
                println!("{:?}", e);
                break;
              }
            }
          }
          e=datareader_event_stream.select_next_some()=>{
            println!("DataReader event: {e:?}");
          }
        }
      }
    })    
    
  } else if "pub" == server_type {
    println!("\nSTARTING TMS DEVICE - TMS deviceId = {:?}\n", my_device_uuid.simple().to_string());
    let  writer = tms_device
    .create_datawriter_no_key::<TmsHeartbeat, CDRSerializerAdapter<TmsHeartbeat>> (
      &heartbeat_topic,
      None)
    .unwrap();
  info!("Created the DDS DataWriter");
  
    info!("-- Start Publisher message sending --");
    
    smol::block_on(async {
      let mut tick_stream = futures::StreamExt::fuse(Timer::interval(SECOND));

      let mut datawriter_event_stream = writer.as_async_status_stream();

      let mut i_u32: u32 = 0;
      
      loop {
        futures::select! {
          _= tick_stream.select_next_some()=>{
            let some_data = TmsHeartbeat { 
            device_id: my_device_uuid.simple().to_string(), 
            sequence_number: i_u32,
            };
            i_u32 += 1;
            writer.async_write(some_data,None).await.unwrap_or_else(|e| println!("DataWriter write failed: {e:?}"));
            println!("Sent TMS Heartbeat sequenceNumber {}", i_u32 -1);
          }
          e= datawriter_event_stream.select_next_some()=>{
            println!("DataWriter event: {e:?}");
          }
        }
      }
    })

  } else {
    error!("Invalid argument: {:?}", server_type);
    return;
  }
  
}

fn configure_logging() {
  log4rs::init_file(
    "logging-config.yaml",
    log4rs::config::Deserializers::default(),
  )
  .unwrap_or_else(|e| {
    match e.downcast_ref::<io::Error>() {
      Some(os_err) if os_err.kind() == io::ErrorKind::NotFound => {
        println!("No logging-config.yaml file found.");
        let stdout = ConsoleAppender::builder().build();
        let conf = Config::builder()
          .appender(Appender::builder().build("stdout", Box::new(stdout)))
          .build(Root::builder().appender("stdout").build(LevelFilter::Error))
          .unwrap();
        log4rs::init_config(conf).unwrap();
      }
      other_error => panic!("Logging config problem {other_error:?}"),
    }
  });
}