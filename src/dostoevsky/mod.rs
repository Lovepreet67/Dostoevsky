use std::{io::Read, net::TcpListener, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use handler::{ConsumerHandler, ProducerHandler};

mod handler; 
mod message;
use message::MessageQueue; 
pub(super) struct DostoevskyNode{
    listener:TcpListener,
    message_queue:Arc<Mutex<MessageQueue>>,
    consumer_count:i32 
}   
impl DostoevskyNode{  
    pub fn new(port:&str)->Result<Self,std::io::Error>{
        let listener = TcpListener::bind(format!("127.0.0.1:{}",port))?;
        Ok(DostoevskyNode {listener,message_queue:Arc::new(Mutex::new(MessageQueue::new())),consumer_count:0})
    }
    pub fn read_all(&mut self){
        let mut consumer_id_seeder = 0;
        self.remove_readed_messages(); 
        for stream in self.listener.incoming(){
            match stream { 
                Ok(mut stream)=>{ 
                   let mut buffer =[0;100];    
                   let _ = stream.read(&mut buffer); 
                   match std::str::from_utf8(&buffer) {
                       Ok(res) =>{ 
                           let  words:Vec<&str> = res.split_whitespace().collect();
                           if words.len()>=5{
                                println!("new {} named {} connected with the valid connection request ",words[2],words[4]);
                                if words[2]=="producer" {
                                    let ph = ProducerHandler::new(words[4].to_string(),stream,Arc::clone(&self.message_queue)); 
                                    let _ = ph.spawn(self.consumer_count);  
                                    continue; 
                                }
                                else if words[2] == "consumer" {  
                                    let ch = ConsumerHandler::new(consumer_id_seeder+1,words[4].to_string(), stream, Arc::clone(&self.message_queue));  
                                    consumer_id_seeder+=1;
                                    self.consumer_count+=1;
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
    fn remove_readed_messages(&mut self){
        let message_queue = Arc::clone(&self.message_queue);  
        thread::spawn(move || { 
            loop {
                println!("trying to clean the queue");
                let mut message_queue = message_queue.lock().unwrap(); 
                match message_queue.get(0){
                    Some(message)=>{
                        println!("\tremaining are : {}",message.get_remaining_consumed_count()); 
                        if message.get_remaining_consumed_count()<= 0 {
                            println!("cleaning the first message");  
                            message_queue.pop();      
                        }
                        else {
                            drop(message_queue);
                            sleep(Duration::from_secs(20));
                        }
                    }
                    None=>{
                        drop(message_queue);
                        sleep(Duration::from_secs(20));
                    }
                }
            }} 
            ); 
    }
}
