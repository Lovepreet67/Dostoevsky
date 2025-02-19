use std::{io::{Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use super::topic::Topic; 

pub(super) struct ProducerHandler{
    name:String, 
    stream:TcpStream,
    topics_subscbribed: Vec<Arc<Mutex<Topic>>>   
}  
impl ProducerHandler{  
    pub fn new(name:String,producer_stream:TcpStream,topic:Vec<Arc<Mutex<Topic>>>)->Self{  
        ProducerHandler {name,stream:producer_stream,topics_subscbribed:topic}   
    }
    pub fn spawn(mut self)->Result<(),std::io::Error>{     
        thread::spawn(move || {
            loop {
                sleep(Duration::from_secs(3));  
                let mut buffer  = [0;256]; //Vec<u8> = Vec::new();      
                let readed_bytes = self.stream.read(&mut buffer).expect("error");  
                if readed_bytes == 0 {   
                    continue; 
                }  
                    //.expect(format!("error while reading the data from producer {}",self.name));        
                println!("readed {} by {}",readed_bytes,self.name); 
                // now we will push the data to the after obtaining the mutex lock on the array
                for topic in &self.topics_subscbribed{
                    let mut topic  = topic.lock().unwrap();   
                    topic.new_message(&buffer);
                }              
            }
        });
        Ok(())   
    }
}

pub(super) struct ConsumerHandler{
    id:i32,
    name:String,
    stream:TcpStream,
    topics_subscbribed:Vec<Arc<Mutex<Topic>>>,  
    consumable_index:usize 
}
impl ConsumerHandler {  
    pub fn new(id:i32,name:String,consumer_stream:TcpStream,topics:Vec<Arc<Mutex<Topic>>>)->Self{
        ConsumerHandler{id,name,stream:consumer_stream,topics_subscbribed:topics,consumable_index:0}    
    }  
    pub fn spawn(mut self)->Result<(),std::io::Error>{
        thread::spawn(move || {
                loop {
                    println!("consumer {} (id: {}) is trying to consumer",self.name,self.id);  
                    for topic in &self.topics_subscbribed { 
                        let mut topic = topic.lock().unwrap();  
                        match topic.get_message(self.consumable_index)  
                        { 
                            Some(buffer)=>{          
                                self.stream.write_all(&buffer).expect("error while writing to the consumer");
                                topic.increment_consumer_count(self.consumable_index); 
                                self.consumable_index+=1;  
                                println!("consumebale index {}",self.consumable_index); 
                            }
                            None=>{
                                println!("Nothing to consume for consumer : {}",self.name); 
                            }
                        }
                    } 
                    sleep(Duration::from_secs(10)); 
                }
        });
        Ok(()) 
    }

}
