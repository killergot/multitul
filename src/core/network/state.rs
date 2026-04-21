use std::sync::Arc;
use std::time::Duration;

pub enum NetworkState {
    High,
    Normal,
    Low
}

impl NetworkState {
    pub fn check_network() -> Option<NetworkState> {
        let addr = "8.8.8.8".parse().unwrap();
        let data = [1,2,3,4];  // ping data
        let data_arc = Arc::new(&data[..]);
        let timeout = Duration::from_secs(1);
        let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
        let future = ping_rs::send_ping_async(&addr, timeout, data_arc, Some(&options));
        let result = futures::executor::block_on(future);
        match result {
            Ok(reply) => {
                println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, data.len(), reply.rtt, options.ttl);
                match reply.rtt{
                    ..100 => Some(NetworkState::High),
                    100..400 => Some(NetworkState::Normal),
                    _ => Some(NetworkState::Low)
                }
            },
            Err(e) =>
                {
                    println!("Error while check network");
                    None
                }
        }
    }
}