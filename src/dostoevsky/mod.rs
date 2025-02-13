use std::{io::{Read }, net::TcpListener,thread::spawn};


pub(super) struct DostoevskyNode{
    listener:TcpListener
} 
impl DostoevskyNode{ 
    pub fn new(port:&str)->Result<Self,std::io::Error>{
        let listener = TcpListener::bind(format!("127.0.0.1:{}",port))?;
        Ok(DostoevskyNode {listener})  
    }
    pub fn read_all(&mut self){
        for stream in self.listener.incoming(){
            match stream { 
                Ok(mut stream)=>{  
                   // stream.write_all("thanks for connecting with me".as_bytes());
                   spawn( move ||  {
                       loop{
                   let mut buffer =[0;100];    
                   let _ = stream.read(&mut buffer); 
                   match std::str::from_utf8(&buffer) {
                       Ok(res) =>{
                           println!("{}",res);
                       }, 
                       Err(_err)=>{
                           println!("invalid data"); 
                       }
                    };
                }} );}   
                Err(err)=>{ 
                    println!("{:?}",err); 
                }
            }
        }
    }
}
impl DostoevskyNode{
    fn add_consumer(&mut self){
    unimplemented!("this method will add the consumer to the list");
    }  
}
