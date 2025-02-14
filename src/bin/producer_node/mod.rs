use std::{io::{self, Write}, net::TcpStream};

pub(super) struct ProducerNode{
    dostoevsky :TcpStream,
    port:String 
}  
impl ProducerNode {  
    pub fn new(port:String,dostoevsky:String)->Result<Self, std::io::Error>{
        let stream = TcpStream::connect(dostoevsky)?; 
        Ok(ProducerNode {dostoevsky:stream,port})      
    }     
    pub fn start(&mut self)->Result<(),io::Error>{      
        println!("starting the consumer node on the port : {}",self.port);  
        self.dostoevsky.write_all(format!("i am producer named {}",self.port).as_bytes())?;   
        Ok(())  
    } 
    pub fn send(&mut self,item:&str)->Result<(),std::io::Error>{
        println!("Sending value : {}",item);
        self.dostoevsky.write_all(item.as_bytes())?;  
        Ok(())     
    } 
}
impl ProducerNode {

} 
