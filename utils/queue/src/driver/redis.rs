#![allow(unused_variables)]
use super::*;
use ::redis::*;

pub struct RedisDriver {
    pub client: Client,
    pub queue_name: String,
}

impl RedisDriver {
    #[allow(dead_code)]
    pub fn new(url: impl AsRef<str>, queue_name: String) -> Result<Self> {
        let client = Client::open(url.as_ref())?;
        Ok(Self { client, queue_name })
    }

    #[allow(dead_code)]
    pub fn connection(&self) -> Result<Connection> {
        Ok(self.client.get_connection()?)
    }
}

impl DodoQueueDriver for RedisDriver {
    fn push(&self, task: Task) -> Result<()> {
        todo!()
    }

    fn pull(&self, task: Task) -> Result<String> {
        todo!()
    }
}
