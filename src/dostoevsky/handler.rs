use std::{collections::VecDeque,io::{Read, Write}, net::TcpStream, sync::{Arc,Mutex}, thread::{self, sleep}, time::Duration};

pub(super) struct ProducerHandler{
    name:String, 
    stream:TcpStream,
    message_queue:Arc<Mutex<VecDeque<[u8;256]>>>  
}  
impl ProducerHandler{ 
    pub fn new(name:String,producer_stream:TcpStream,message_queue:Arc<Mutex<VecDeque<[u8;256]>>>)->Self{ 
        ProducerHandler {name,stream:producer_stream,message_queue}  
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
                let mut message_queue = self.message_queue.lock().unwrap();
                message_queue.push_back(buffer);
            }
        });
        Ok(())  
    }
}

pub(super) struct ConsumerHandler{
    name:String,
    stream:TcpStream,
    message_queue:Arc<Mutex<VecDeque<[u8;256]>>>  
}
impl ConsumerHandler {
    pub fn new(name:String,consumer_stream:TcpStream,message_queue:Arc<Mutex<VecDeque<[u8;256]>>>)->Self{
        ConsumerHandler{name,stream:consumer_stream,message_queue} 
    }
    pub fn spawn(mut self)->Result<(),std::io::Error>{
        thread::spawn(move || {
                loop {
                    let mut message_queue = self.message_queue.lock().unwrap();
                    match message_queue.pop_front() {
                        Some(item)=>{
                            self.stream.write_all(&item).expect("error while writing to the consumer");   
                        }
                        None=>{
                            drop(message_queue);
                            sleep(Duration::from_secs(10));
                        }
                    }  
                }
        });
        Ok(()) 
    }

}
