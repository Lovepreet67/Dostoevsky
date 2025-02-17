mod producer_node; 

use std::{env, io};
  
fn main()->Result<(),std::io::Error>{ 
    let args:Vec<String> = env::args().collect();  
    let port =  if args.len()>1
        {
            args[1].clone() 
        } else {
            "3306".to_string() 
        }; 
    let dostoevsky_addr = if args.len()>2 {
            args[2].clone() 
        } else {
            "127.0.0.1:3370".to_string()  
        }; 
    println!("trying to connect to the dostoevsky on address : {}",dostoevsky_addr);
    let mut producer_node = producer_node::ProducerNode::new(port,dostoevsky_addr,args[3].clone())?; 
    producer_node.start()?;  

    println!("\nwelcome to the system\n");
    let input_handler = io::stdin(); 
    let mut buffer = String::new();
    loop { 
        buffer.clear(); 
        let _ = input_handler.read_line(&mut buffer);
        producer_node.send(&buffer)?; 
    }   
}    
