use std::io::Error; 

pub struct Message{
    buffer:[u8;256],
    consumed_count:i32  
}
impl Message{
    pub fn new(buffer:[u8;256])->Self{
        Message {buffer,consumed_count:0  }   
    }
    pub fn increment_consumed_count(&mut self){
        self.consumed_count+=1;  
    } 
    pub fn get_buffer(&self)->[u8;256]{
        self.buffer 
    }
    pub fn get_consumed_count(&self)->i32{
        self.consumed_count 
    } 
}


pub struct MessageQueue {
    start_index:usize,
    first_empty_index:usize,
    ring_buffer:Vec<Option<Message>> 
}
impl MessageQueue{ 
    pub fn new()->Self{    
        let mut ring_buffer = Vec::new(); 
        ring_buffer.resize_with(100,|| None);
        MessageQueue {ring_buffer ,start_index:0,first_empty_index:0} 
    } 
    pub fn push(&mut self,message:Message)->Result<(),Error>{  
        if self.size()==self.ring_buffer.len()  
    {
           return Err(Error::new(std::io::ErrorKind::Other,"buffer is full"));    
        } 
        println!("element added at : {}",self.first_empty_index);  
        self.ring_buffer[self.first_empty_index] = Some(message);
        self.first_empty_index = (self.first_empty_index+1)%self.ring_buffer.len();  
        Ok(()) 
    }  
    pub fn get(&mut self,index:usize)->Option<&mut Message>{   
        let rounded_index = index%self.ring_buffer.len();
        self.ring_buffer[rounded_index].as_mut()   
    }  
    pub fn pop(&mut self){
        if self.size()>0 {
            self.ring_buffer[self.start_index]=None;
            self.start_index = (self.start_index+1)%self.ring_buffer.len(); 
        }  
    }
}

impl MessageQueue { 
    fn size(&self)->usize{
        (self.first_empty_index+self.ring_buffer.len()-self.start_index)%self.ring_buffer.len() 
    }
} 
