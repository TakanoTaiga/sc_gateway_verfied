// Copyright 2023 Hakoroboken
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fmt::{Display , Formatter}, net::SocketAddr};


pub enum NodeConnectionKey {
    SearchNode,
    PingRequest,
    PingResponse,
    IpResponse,
    NodeInfoRequest,
    NodeInfoResponse,
    GamepadValueRequest,
    UnknownKey,
    MissingKey
}

impl Display for NodeConnectionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<() , std::fmt::Error> {
        match self {
            NodeConnectionKey::SearchNode => write!(f , "NC::SearchNode"),
            NodeConnectionKey::PingRequest => write!(f , "NC::PingRequest"),
            NodeConnectionKey::PingResponse => write!(f , "NC::PingResponse"),
            NodeConnectionKey::IpResponse => write!(f , "NC::IpResponse"),
            NodeConnectionKey::NodeInfoRequest => write!(f , "NC::NodeInfoRequest"),
            NodeConnectionKey::NodeInfoResponse => write!(f , "NC::NodeInfoResponse"),
            NodeConnectionKey::GamepadValueRequest => write!(f , "NC::GamepadValueRequest"),
            _ => write!(f , "NC::Unknown Key"),
        }
    }
}

pub trait EnumKeyUtil {
    fn convert_to_u8key(&self)->u8;
}
impl EnumKeyUtil for NodeConnectionKey {
    fn convert_to_u8key(&self)->u8{
        match self {
            NodeConnectionKey::SearchNode => 47,
            NodeConnectionKey::PingRequest => 98,
            NodeConnectionKey::PingResponse => 87,
            NodeConnectionKey::IpResponse => 75,
            NodeConnectionKey::NodeInfoRequest => 23,
            NodeConnectionKey::NodeInfoResponse => 25,
            NodeConnectionKey::GamepadValueRequest => 12,
            _ => 0,
        }
    }
}

pub trait U8KeyUtil {
    fn convert_to_enumkey(&self) -> NodeConnectionKey;
}
impl U8KeyUtil for u8 {
    fn convert_to_enumkey(&self) -> NodeConnectionKey{
        match self {
            47 => NodeConnectionKey::SearchNode,
            98 => NodeConnectionKey::PingRequest,
            87 => NodeConnectionKey::PingResponse,
            75 => NodeConnectionKey::IpResponse,
            23 => NodeConnectionKey::NodeInfoRequest,
            25 => NodeConnectionKey::NodeInfoResponse,
            12 => NodeConnectionKey::GamepadValueRequest,
            _ => NodeConnectionKey::UnknownKey,
        }
    }
}

pub struct ConnectionBuffer {
    pub connection_key: NodeConnectionKey,
    pub raw_buffer: [u8; 2048],
    pub rcv_size: usize,
    pub taget_address: SocketAddr
}