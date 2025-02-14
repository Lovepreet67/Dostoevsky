pub enum ConnectionType{
    ConsumerConnection,
    ProducerConnection  
}
pub enum Request{
    Connect(ConnectionType),
    Consume,
    Produce,
    Terminate
}
 
