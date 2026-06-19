use std::time::SystemTime;

use godot::classes::{INode, Node};
use godot::prelude::*;
use renet::{ConnectionConfig, ServerEvent};

enum NetworkingEvent {
    MessageReceived {
        client_id: i64,
        message: PackedArray<u8>,
    },
}

enum NetworkingConfig {
    NotConnected {},
    NetcodeServerConnected {
        server: renet::RenetServer,
        transport: renet_netcode::NetcodeServerTransport,
    },
    NetcodeClientConnected {
        client: renet::RenetClient,
        transport: renet_netcode::NetcodeClientTransport,
    },
}

#[derive(GodotClass)]
#[class(base=Node)]
struct HadalythNetworking {
    networking_config: NetworkingConfig,

    tx: std::sync::mpsc::Sender<NetworkingEvent>,
    rx: std::sync::mpsc::Receiver<NetworkingEvent>,

    base: Base<Node>,
}

#[godot_api]
impl INode for HadalythNetworking {
    fn init(base: Base<Node>) -> Self {
        let networking_config = NetworkingConfig::NotConnected {};

        let (tx, rx) = std::sync::mpsc::channel::<NetworkingEvent>();

        Self {
            networking_config,

            tx,
            rx,

            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let networking_config = &mut self.networking_config;
        match networking_config {
            NetworkingConfig::NotConnected {} => {
                return;
            }

            NetworkingConfig::NetcodeServerConnected { server, transport } => {
                let duration = std::time::Duration::from_secs_f64(delta);
                server.update(duration);
                let err = transport.update(duration, server);
                if err.is_err() {
                    self.networking_config = NetworkingConfig::NotConnected {};
                    godot_error!("{:?}", err.unwrap_err());
                    return;
                }

                while let Some(event) = server.get_event() {
                    match event {
                        ServerEvent::ClientConnected { client_id } => {
                            godot_print!(
                                "HadalythNetworking::ServerEvent::ClientConnected {}",
                                client_id
                            );
                        }
                        ServerEvent::ClientDisconnected { client_id, reason } => {
                            godot_print!(
                                "HadalythNetworking::ServerEvent::ClientDisconnected {} {}",
                                client_id,
                                reason
                            );
                        }
                    }
                }
                for client_id in server.clients_id() {
                    while let Some(message) =
                        server.receive_message(client_id, renet::DefaultChannel::ReliableOrdered)
                    {
                        let message = zstd::bulk::decompress(&message, 1024 * 1024);
                        let Ok(message) = message else {
                            godot_error!("Decompression Error");
                            continue;
                        };
                        let message = PackedArray::<u8>::from(message);

                        let tx = self.tx.clone();
                        let _ = tx.send(NetworkingEvent::MessageReceived {
                            client_id: client_id as i64,
                            message,
                        });
                    }
                }
            }

            NetworkingConfig::NetcodeClientConnected { client, transport } => {
                let duration = std::time::Duration::from_secs_f64(delta);
                client.update(duration);
                let err = transport.update(duration, client);
                if err.is_err() {
                    self.networking_config = NetworkingConfig::NotConnected {};
                    godot_error!("{:?}", err.unwrap_err());
                    return;
                }

                if !client.is_connected() {
                    return;
                }

                while let Some(message) =
                    client.receive_message(renet::DefaultChannel::ReliableOrdered)
                {
                    let message = zstd::bulk::decompress(&message, 1024 * 1024);
                    let Ok(message) = message else {
                        godot_error!("Decompression Error");
                        continue;
                    };
                    let message = PackedArray::<u8>::from(message);

                    let tx = self.tx.clone();
                    let _ = tx.send(NetworkingEvent::MessageReceived {
                        client_id: 0,
                        message,
                    });
                }
            }
        }

        while let Ok(event) = self.rx.try_recv() {
            match event {
                NetworkingEvent::MessageReceived { client_id, message } => {
                    self.signals()
                        .recv_networking_message()
                        .emit(client_id, &message);
                }
            }
        }
    }
}

#[godot_api]
impl HadalythNetworking {
    #[signal]
    fn recv_networking_message(client_id: i64, message: PackedArray<u8>);

    #[func]
    fn connect_server_netcode(&mut self, port: i64, max_clients: i64) {
        let connection_config = renet::ConnectionConfig::default();

        let server = renet::RenetServer::new(connection_config);

        // TODO : This needs to be the public IP address at some point

        let socket_addr =
            std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), port as u16);
        let socket = match std::net::UdpSocket::bind(socket_addr) {
            Ok(socket) => socket,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };

        let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(current_time) => current_time,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };

        // NOTE : You need a server for a secure authentication
        let authentication = renet_netcode::ServerAuthentication::Unsecure {};

        let server_config = renet_netcode::ServerConfig {
            current_time: current_time,
            max_clients: max_clients as usize,
            protocol_id: 0,
            public_addresses: vec![socket_addr],
            authentication: authentication,
        };

        let transport = match renet_netcode::NetcodeServerTransport::new(server_config, socket) {
            Ok(transport) => transport,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };

        let networking_config = NetworkingConfig::NetcodeServerConnected { server, transport };
        self.networking_config = networking_config;
    }

    #[func]
    fn connect_client_netcode(&mut self, client_id: i64, ip_addr: String, port: i64) {
        let connection_config = ConnectionConfig::default();

        let client = renet::RenetClient::new(connection_config);

        let ip_addr: std::net::IpAddr = match ip_addr.parse() {
            Ok(ip_addr) => ip_addr,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };
        let server_addr = std::net::SocketAddr::new(ip_addr, port as u16);
        let socket_addr = std::net::SocketAddr::new(ip_addr, 0);
        let socket = match std::net::UdpSocket::bind(socket_addr) {
            Ok(socket) => socket,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };

        let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(current_time) => current_time,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };

        let authentication = renet_netcode::ClientAuthentication::Unsecure {
            protocol_id: 0,
            client_id: client_id as u64,
            server_addr,
            user_data: None,
        };

        let transport = match renet_netcode::NetcodeClientTransport::new(
            current_time,
            authentication,
            socket,
        ) {
            Ok(transport) => transport,
            Err(err) => {
                godot_error!("{:?}", err);
                return;
            }
        };

        let networking_config = NetworkingConfig::NetcodeClientConnected { client, transport };
        self.networking_config = networking_config;
    }

    #[func]
    fn broadcast_message(&mut self, message: PackedArray<u8>) {
        let message = zstd::bulk::compress(&message.to_vec(), 1);
        let Ok(message) = message else {
            godot_error!("Compression failed");
            return;
        };

        let networking_config = &mut self.networking_config;
        match networking_config {
            NetworkingConfig::NotConnected {} => {
                return;
            }
            NetworkingConfig::NetcodeServerConnected { server, .. } => {
                let _ = server.broadcast_message(renet::DefaultChannel::ReliableOrdered, message);
            }
            NetworkingConfig::NetcodeClientConnected { .. } => {
                return;
            }
        }
    }

    #[func]
    fn broadcast_message_except(&mut self, client_id: i64, message: PackedArray<u8>) {
        let message = zstd::bulk::compress(&message.to_vec(), 1);
        let Ok(message) = message else {
            godot_error!("Compression failed");
            return;
        };

        let networking_config = &mut self.networking_config;
        match networking_config {
            NetworkingConfig::NotConnected {} => {
                return;
            }
            NetworkingConfig::NetcodeServerConnected { server, .. } => {
                let _ = server.broadcast_message_except(
                    client_id as u64,
                    renet::DefaultChannel::ReliableOrdered,
                    message,
                );
            }
            NetworkingConfig::NetcodeClientConnected { .. } => {
                return;
            }
        }
    }

    #[func]
    fn send_message_server(&mut self, client_id: i64, message: PackedArray<u8>) {
        let message = zstd::bulk::compress(&message.to_vec(), 1);
        let Ok(message) = message else {
            godot_error!("Compression failed");
            return;
        };

        let networking_config = &mut self.networking_config;
        match networking_config {
            NetworkingConfig::NotConnected {} => {
                return;
            }
            NetworkingConfig::NetcodeServerConnected { server, .. } => {
                let _ = server.send_message(
                    client_id as u64,
                    renet::DefaultChannel::ReliableOrdered,
                    message,
                );
            }
            NetworkingConfig::NetcodeClientConnected { .. } => {
                return;
            }
        }
    }

    #[func]
    fn send_message_client(&mut self, message: PackedArray<u8>) {
        let message = zstd::bulk::compress(&message.to_vec(), 1);
        let Ok(message) = message else {
            godot_error!("Compression failed");
            return;
        };

        let networking_config = &mut self.networking_config;
        match networking_config {
            NetworkingConfig::NotConnected {} => {
                return;
            }
            NetworkingConfig::NetcodeServerConnected { .. } => {
                return;
            }
            NetworkingConfig::NetcodeClientConnected { client, .. } => {
                let _ = client.send_message(renet::DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}
