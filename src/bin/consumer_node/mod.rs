use std::{io::{Read, Write}, net::TcpStream, usize};

pub(super) struct ConsumerNode{
    port:String,
    dostoevsky:TcpStream, 
    topic:String
}

impl ConsumerNode {   
    pub fn new(port:String,dostoevsky_addr:&str,topic:String)->Result<Self,std::io::Error>{
        let dostoevsky = TcpStream::connect(dostoevsky_addr)?;
        Ok(ConsumerNode{port,dostoevsky,topic})    
    }
    pub fn start(&mut self)->Result<(),std::io::Error>{ 
        self.dostoevsky.write_all(format!("i am consumer named {} {}  ",self.port,self.topic).as_bytes())?;   
        Ok(())  
    }
    pub fn recv(&mut self,buffer:&mut [u8;256])->Result<usize,std::io::Error>{
        let read_bytes = self.dostoevsky.read(buffer)?;
        Ok(read_bytes)   
    } 
}
impl ConsumerNode {
 

}  
