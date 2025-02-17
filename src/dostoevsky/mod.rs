use std::{collections::HashMap, io::Read,sync::{Arc,Mutex}, net::TcpListener, thread::{self, sleep}, time::Duration};
 
use handler::{ConsumerHandler, ProducerHandler};
mod topic; 
mod handler;  
mod message;
use topic::Topic; 
pub(super) struct DostoevskyNode{
    listener:TcpListener,
    topics:HashMap<String,Arc<Mutex<Topic>>> 
}   
impl DostoevskyNode{  
    pub fn new(port:&str)->Result<Self,std::io::Error>{
        let listener = TcpListener::bind(format!("127.0.0.1:{}",port))?;
        Ok(DostoevskyNode {listener,topics:HashMap::new()})  
    }
    pub fn read_all(&mut self){
        println!("creating a topic t1");
        self.create_topic("t1".to_string());
        println!("creating a topic t2");
        self.create_topic("t2".to_string());  
        let mut consumer_id_seeder = 0;
        for stream in self.listener.incoming(){ 
            match stream { 
                Ok(mut stream)=>{ 
                   let mut buffer =[0;100];    
                   let _ = stream.read(&mut buffer); 
                   match std::str::from_utf8(&buffer) {
                       Ok(res) =>{ 
                           let  words:Vec<&str> = res.split_whitespace().collect();
                           println!("{:?}",words); 
                           if words.len()>=6{
                                println!("new {} named {} connected with the valid connection request ",words[2],words[4]);                                let topic = match self.topics.get(words[5]){  
                                        Some(topic) =>topic,  
                                        None =>{
                                            println!("\nno topic specified");
                                            continue; 
                                        }
                                    };
                                if words[2]=="producer" {
                                    let ph = ProducerHandler::new(words[4].to_string(),stream,vec![Arc::clone(topic)]);    
                                    let _ = ph.spawn();   
                                    continue; 
                                }
                                else if words[2] == "consumer" { 
                                    //TODO; increment the consumer count   
                                    let ch = ConsumerHandler::new(consumer_id_seeder+1,words[4].to_string(), stream,vec![Arc::clone(topic)]);  
                                    let mut topic_mut = topic.lock().unwrap();
                                    topic_mut.add_consumer(); 
                                    drop(topic_mut); 
                                    consumer_id_seeder+=1;
                                    let _ = ch.spawn();
                                    continue;  
                                }
                           }
                            println!("thread joined with the invalid connection request"); 
                       }, 
                       Err(_err)=>{
                           println!("invalid data"); 
                       }
                    };
                }  
                Err(err)=>{ 
                    println!("{:?}",err); 
                }
            }
        }
    }
}

impl DostoevskyNode{
    fn create_topic(&mut self,topic_name:String){
        self.topics.entry(topic_name.clone()).or_insert_with(||{  
            let topic = Topic::new(&topic_name);
           Arc::new(Mutex::new(topic))  

        }); 
            
         
    }
}
