use crate::custom_resources::live_link_face_payload::LiveLinkFacePayload;

pub enum ArkitEvent {
    LocalIpAddressStatus(Option<String>),

    UdpSocketClosed(),

    ArkitPayload(LiveLinkFacePayload),
}
