use iced::{
    Element, Length, Task, Theme,
    widget::{button, column, row, svg, text},
};
use crate::core::network::state::NetworkState;

#[derive(Debug, Clone)]
struct Icons {
    high: svg::Handle,
    normal: svg::Handle,
    low: svg::Handle,
    none: svg::Handle
}

impl Icons {
    pub fn load() -> Self {
        Self{
            high: svg::Handle::from_path("assets/icons/wifi_high.svg"),
            normal: svg::Handle::from_path("assets/icons/wifi_medium.svg"),
            low: svg::Handle::from_path("assets/icons/wifi_low.svg"),
            none: svg::Handle::from_path("assets/icons/wifi_none.svg")
        }
    }
}


pub struct Network {
    state: Option<NetworkState>,
    icons: Icons
}



impl Network {
    pub fn new(state: Option<NetworkState>) -> Self {
        Self{
            state,
            icons: Icons::load()
        }
    }

    pub fn set_state(&mut self, state: Option<NetworkState>) {
        self.state= state;
    }
    pub fn state(&self) -> &Option<NetworkState> {
        &self.state
    }

    pub fn get_icon(&self) -> &svg::Handle {
        if let Some(state) = &self.state {
            match state {
                NetworkState::High => &self.icons.high,
                NetworkState::Normal => &self.icons.normal,
                NetworkState::Low => &self.icons.low,
            }
        } else {
            &self.icons.none
        }
    }

    pub fn check_network(&mut self){
        self.state = NetworkState::check_network();
    }
}