use super::message::{Message, MessageQueue};


pub(super) struct Topic{ 
    name:String,
    message_queue:MessageQueue, 
    consumer_count:i32 
} 

impl Topic{
    pub fn new(name:&str)->Self{
        Topic {name:name.to_string(),message_queue:MessageQueue::new(),consumer_count:0}  
    }
    pub fn new_message(&mut self,buffer:&[u8;256]){
        self.message_queue.push(Message::new(*buffer));   
    } 
    pub fn get_message(&mut self,index:usize)->Option<[u8;256]>{     
        match self.message_queue.get(index) {
            Some(message)=>{
                 Some(message.get_buffer())      
            }   
            None=>{
                None
            } 
        } 
    }
    pub fn increment_consumer_count(&mut self,index:usize){
        match self.message_queue.get(index) {  
            Some(message)=>{ 
                message.increment_consumed_count();
                if message.get_consumed_count()==self.consumer_count {
                    self.message_queue.pop(); 
                }
            }
            None=>{
                 
            }
        }
    }
    pub fn add_consumer(&mut self){ 
        self.consumer_count+=1;  
    } 
    
}
