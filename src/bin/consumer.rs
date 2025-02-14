use std::env;

use consumer_node::ConsumerNode;
mod consumer_node; 
fn main()->Result<(),std::io::Error>{
    let args:Vec<String>  = env::args().collect(); 
    let port = if args.len()>1 {
        args[1].clone()
    }  else {
        "3306".to_string() 
    }; 
    let dostoevsky_addr = if args.len()>2{
        args[2].clone()
    } else {
        "127.0.0.1:3370".to_string()   
    }; 
    println!("trying to connect to the dostoevsky at : {}",dostoevsky_addr);
    //TODO: remove the port clone 
    let mut consumer_node = ConsumerNode::new(port.clone(),&dostoevsky_addr)?;  
    println!("creating a new consumer node on port : {}",port); 
    consumer_node.start()?;     
    println!("\nwelcome to the system\n");
    loop {
        let mut buffer = [0;256]; 
        let read_bytes = consumer_node.recv(&mut buffer)?;   
        if read_bytes>0   
        {
            match std::str::from_utf8(&buffer){
                Ok(value)=>{ 
                    println!("got value : {}",value);
                }
                Err(_error)=>{
                    println!("error while converting the value to string");  
                }
            }
        }
        else {
            continue; 
        }
    }     
} 
