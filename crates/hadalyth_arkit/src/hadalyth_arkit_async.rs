use crate::{
    custom_events::arkit_event::ArkitEvent,
    custom_resources::live_link_face_payload::LiveLinkFacePayload,
};

pub async fn get_local_ip_address_async(tx: std::sync::mpsc::Sender<ArkitEvent>) {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await;
    let Ok(socket) = socket else {
        let _ = tx.send(ArkitEvent::LocalIpAddressStatus(None));
        return;
    };

    let connect_status = socket.connect("8.8.8.8:80").await;
    if !connect_status.is_ok() {
        let _ = tx.send(ArkitEvent::LocalIpAddressStatus(None));
        return;
    };

    let socket_addr = socket.local_addr();
    let Ok(socket_addr) = socket_addr else {
        let _ = tx.send(ArkitEvent::LocalIpAddressStatus(None));
        return;
    };

    let local_ip = socket_addr.ip();
    let _ = tx.send(ArkitEvent::LocalIpAddressStatus(Some(local_ip.to_string())));
}

pub async fn open_udp_socket_async(addr: String, tx: std::sync::mpsc::Sender<ArkitEvent>) {
    let socket = tokio::net::UdpSocket::bind(addr.clone()).await;
    let Ok(socket) = socket else {
        let _ = tx.send(ArkitEvent::UdpSocketClosed());
        return;
    };

    let mut buffer: Vec<u8> = vec![0u8; 65535];

    loop {
        let result = socket.recv_from(&mut buffer).await;
        let Ok(result) = result else {
            break;
        };

        let message_size = result.0;

        let message = &buffer[..message_size];

        let live_link_face_payload = LiveLinkFacePayload::from_raw(message);
        let Ok(live_link_face_payload) = live_link_face_payload else {
            break;
        };

        let _ = tx.send(ArkitEvent::ArkitPayload(live_link_face_payload));
    }

    let _ = tx.send(ArkitEvent::UdpSocketClosed());
    return;
}
