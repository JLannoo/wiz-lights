use crate::{Light, Messenger::MessageConstructor};

use std::{net,time, ops::RangeInclusive};
use local_ip_address::local_ip;
use serde_json;
use serde::{Deserialize};

const DISCOVERY_PORT: u16 = 38899;
const SOCKET_TIMEOUT_MS: u64 = 500;

#[derive(Deserialize, Debug)]
pub struct GetSystemConfigResponse {
    method: String,
    env: String,
    result: GetSystemConfigResult,
}

#[derive(Deserialize, Debug)]
pub struct GetSystemConfigResult {
    mac: String,
    homeId: u32,
    roomId: u32,
    rgn: String,
    moduleName: String,
    fwVersion: String,
    groupId: u32,
    ping: u8,
}

#[derive(Deserialize, Debug)]
pub struct GetPilotResponse {
    method: String,
    env: String,
    result: GetPilotResult,
}

/// The response from the getPilot command
/// Color can be expressed as r,g,b or temp or sceneId
/// But they are mutually exclusive
#[derive(Deserialize, Debug)]
pub struct GetPilotResult {
    mac: String,
    rssi: i8,
    src: String,
    state: bool,
    dimming: u8,
    sceneId: u8,
    temp: u16,
}

pub struct Wiz {
    socket: net::UdpSocket,
    port: u16,
    pub lights: Vec<Light::Light>,
    pub rooms: Vec<u32>,
    pub homes: Vec<u32>,
}

impl Wiz {
    pub fn new() -> Self {
        let socket = net::UdpSocket::bind("0.0.0.0:0").expect("Could not bind to UDP socket");
        socket.set_read_timeout(Some(time::Duration::from_millis(SOCKET_TIMEOUT_MS))).expect("Could not set UDP socket read timeout");

        Self {
            socket,
            port: DISCOVERY_PORT,
            lights: Vec::new(),
            rooms: Vec::new(),
            homes: Vec::new(),
        }
    }

    pub fn find_lights(&mut self){
        const RANGE: RangeInclusive<u8> = 18..=25;

        let ip = local_ip().expect("Could not get local IP address");

        for i in RANGE {
            // Replace the last octet of the IP address with the current value of i
            let ip = ip.to_string().split('.').take(3).collect::<Vec<&str>>().join(".") + &format!(".{}", i);

            // Send UDP packet
            let msg = &MessageConstructor::get_system_config();
            
            let msg = match self.send_message(msg, &ip) {
                Some(string) => string,
                None => continue,
            };
            let msg: GetSystemConfigResponse = serde_json::from_str(&msg).expect("Could not parse JSON");

            if !self.rooms.contains(&msg.result.roomId) {
                self.rooms.push(msg.result.roomId);
            }

            if !self.homes.contains(&msg.result.homeId) {
                self.homes.push(msg.result.homeId);
            }
            
            self.lights.push(Light::Light::new(ip, self.port, msg.result.mac));
        }

        println!("Found {} lights", self.lights.len());
    }

    pub fn send_message(&self, message: &str, ip: &String) -> Option<String> {
        let ip: net::IpAddr = ip.parse().expect("Could not parse IP address");
        let msg = message.as_bytes();

        println!("Sending message to {}: {}", ip, message);

        self.socket.send_to(msg, (ip, self.port)).expect("Could not send UDP packet");

        let buffer = &mut [0; 1024];
        let (amt, _) = match self.socket.recv_from(buffer) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let buffer = &mut buffer[..amt];
        let msg = String::from_utf8_lossy(buffer);

        Some(msg.to_string())
    }

    pub fn get_pilot(&self, light: &Light::Light) -> Option<GetPilotResult> {
        let msg = &MessageConstructor::get_pilot_state();
        let msg = match self.send_message(msg, &light.ip) {
            Some(string) => string,
            None => return None,
        };

        let msg: GetPilotResponse = serde_json::from_str(&msg).expect("Could not parse JSON");

        Some(msg.result)
    }
}

impl std::fmt::Debug for Wiz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lights: {:#?}\nRooms: {:#?}\nHomes: {:#?}", self.lights, self.rooms, self.homes)
    }
}