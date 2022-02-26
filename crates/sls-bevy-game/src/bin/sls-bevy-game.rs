use env_logger::env_logger;

fn main(){
  env_logger::init();
  log::info!("hello!")
}