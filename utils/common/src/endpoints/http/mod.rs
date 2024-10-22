pub mod middleware;

pub trait HttpStateT: Clone + Sync + Send + 'static {}

pub trait HttpUserT: Clone + Sync + Send + 'static {
    fn role(&self) -> &[String];
}
