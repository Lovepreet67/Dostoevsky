use std::{io::{Read, Write}, net::TcpStream, sync::{Arc,Mutex}, thread::{self, sleep}, time::Duration};

use super::message::{Message, MessageQueue};

pub(super) struct ProducerHandler{
    name:String, 
    stream:TcpStream,
    message_queue:Arc<Mutex<MessageQueue>>  
}  
impl ProducerHandler{ 
    pub fn new(name:String,producer_stream:TcpStream,message_queue:Arc<Mutex<MessageQueue>>)->Self{ 
        ProducerHandler {name,stream:producer_stream,message_queue}  
    }
    pub fn spawn(mut self,consumer_count:i32)->Result<(),std::io::Error>{     
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
                let mut message_queue = self.message_queue.lock().unwrap();
                message_queue.push(Message::new(buffer,consumer_count)); 
            }
        });
        Ok(())   
    }
}

pub(super) struct ConsumerHandler{
    id:i32,
    name:String,
    stream:TcpStream,
    message_queue:Arc<Mutex<MessageQueue>>, 
    consumable_index:usize 
}
impl ConsumerHandler {
    pub fn new(id:i32,name:String,consumer_stream:TcpStream,message_queue:Arc<Mutex<MessageQueue>>)->Self{
        ConsumerHandler{id,name,stream:consumer_stream,message_queue,consumable_index:0}    
    }
    pub fn spawn(mut self)->Result<(),std::io::Error>{
        thread::spawn(move || {
                loop {
                    println!("consumer {} (id: {}) is trying to consumer",self.name,self.id);  
                    let mut message_queue = self.message_queue.lock().unwrap();

                    match message_queue.get(self.consumable_index)  
                    { 
                        Some(message)=>{       
                             self.stream.write_all(&message.get_buffer()).expect("error while writing to the consumer");
                             message.decrement_remaining_consumed_count();  
                            self.consumable_index+=1;    
                        }
                        None=>{
                            println!("Nothing to consume for consumer : {}",self.name); 
                        }
                    } 
                    sleep(Duration::from_secs(10)); 
                }
        });
        Ok(()) 
    }

}
