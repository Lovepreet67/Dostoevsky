use std::env;
use dostoevsky::DostoevskyNode;

mod dostoevsky;  
mod message; 
fn main()->Result<(),std::io::Error> {  
    let args:Vec<String> = env::args().collect();
    let port = if args.len()>1{
        args[1].clone()
    } else {
        "3370".to_string()  
    };   
    println!("creating the dostoevsky broker node"); 
    println!("Starting the dostoevsky node on port : {}",port);
    let mut dostoevsky_node = DostoevskyNode::new(&port).expect("error while creating the dostoevsky");   
    dostoevsky_node.read_all();   
    Ok(())
} 
