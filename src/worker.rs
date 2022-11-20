use cuid::cuid;
use crate::cluster::Cluster;
use anyhow::Result;

#[derive(PartialEq, Eq, Hash)]
pub struct Worker {
    pub id: String,
    pub arguments: Vec<String>,
    pub cluster: Cluster
}

impl Worker {
    pub fn new(cluster: Cluster, arguments: Vec<String>) -> Result<Worker> {
        Ok(Worker {
            id: cuid()?,    
            arguments,
            cluster
        })
        
    }

    pub fn start(self) {
        dbg!("ya");
        tokio::task::Builder::new().name(self.id.as_str()).spawn(async move {
            crate::spawn::invoke()
        }).unwrap();
    }
}