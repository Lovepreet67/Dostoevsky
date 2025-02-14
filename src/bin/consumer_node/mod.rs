use std::{io::{Read, Write}, net::TcpStream, usize};

pub(super) struct ConsumerNode{
    port:String,
    dostoevsky:TcpStream
}

impl ConsumerNode { 
    pub fn new(port:String,dostoevsky_addr:&str)->Result<Self,std::io::Error>{
        let dostoevsky = TcpStream::connect(dostoevsky_addr)?;
        Ok(ConsumerNode{port,dostoevsky})   
    }
    pub fn start(&mut self)->Result<(),std::io::Error>{ 
        self.dostoevsky.write_all(format!("i am consumer named {}",self.port).as_bytes())?;   
        Ok(())  
    }
    pub fn recv(&mut self,buffer:&mut [u8;256])->Result<usize,std::io::Error>{
        let read_bytes = self.dostoevsky.read(buffer)?;
        Ok(read_bytes)   
    } 
}
impl ConsumerNode {
 

}  
