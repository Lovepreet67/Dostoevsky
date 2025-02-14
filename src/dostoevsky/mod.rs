use std::{collections::VecDeque, io::Read, net::TcpListener, sync::{Arc, Mutex}};

use handler::{ConsumerHandler, ProducerHandler};

mod handler; 

pub(super) struct DostoevskyNode{
    listener:TcpListener,
    message_queue:Arc<Mutex<VecDeque<[u8;256]>>>  
}  
impl DostoevskyNode{ 
    pub fn new(port:&str)->Result<Self,std::io::Error>{
        let listener = TcpListener::bind(format!("127.0.0.1:{}",port))?;
        Ok(DostoevskyNode {listener,message_queue:Arc::new(Mutex::new(VecDeque::<[u8; 256]>::new()))})
    }
    pub fn read_all(&mut self){
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
                                    let _ = ph.spawn(); 
                                    continue; 
                                }
                                else if words[2] == "consumer" {  
                                    let ch = ConsumerHandler::new(words[4].to_string(), stream, Arc::clone(&self.message_queue));   
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
