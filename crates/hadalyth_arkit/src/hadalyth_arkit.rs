use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;

use crate::custom_config::hadalyth_arkit_connection::HadalythArkitConnection;
use crate::custom_events::arkit_event::ArkitEvent;
use crate::custom_resources::live_link_face_blend_shapes::LiveLinkFaceBlendShapes;
use crate::hadalyth_arkit_async::{get_local_ip_address_async, open_udp_socket_async};

#[derive(GodotClass)]
#[class(base=Node)]
struct HadalythArkit {
    #[export]
    hadalyth_arkit_connection: Option<Gd<HadalythArkitConnection>>,

    local_ip_address: Option<String>,
    local_ip_address_timer: f64,

    runtime: Option<tokio::runtime::Runtime>,

    tx: std::sync::mpsc::Sender<ArkitEvent>,
    rx: std::sync::mpsc::Receiver<ArkitEvent>,

    udp_socket_open: bool,
    udp_socket_open_timer: f64,

    base: Base<Node>,
}

#[godot_api]
impl INode for HadalythArkit {
    fn init(base: Base<Node>) -> Self {
        let hadalyth_arkit_connection = None;

        let local_ip_address = None;
        let local_ip_address_timer = 0.0;

        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => Some(runtime),
            Err(err) => {
                godot_error!("{:?}", err);
                None
            }
        };

        let (tx, rx) = std::sync::mpsc::channel::<ArkitEvent>();

        let udp_socket_open = false;
        let udp_socket_open_timer = 0.0;

        return Self {
            hadalyth_arkit_connection,
            local_ip_address,
            local_ip_address_timer,
            runtime,
            tx,
            rx,
            udp_socket_open,
            udp_socket_open_timer,
            base,
        };
    }

    fn ready(&mut self) {}

    fn process(&mut self, delta: f64) {
        // Parse callbacks from rx
        while let Ok(event) = self.rx.try_recv() {
            match event {
                // Startup
                ArkitEvent::LocalIpAddressStatus(local_ip_address) => {
                    godot_print!("ArkitEvent::LocalIpAddressStatus: {:?}", local_ip_address);
                    self.local_ip_address = local_ip_address;
                }

                // Udp Socket status
                ArkitEvent::UdpSocketClosed() => {
                    godot_print!("ArkitEvent::UdpSocketClosed");
                    self.udp_socket_open = false;
                }

                // ARKit packets parsed into useful parameters
                ArkitEvent::ArkitPayload(payload) => {
                    if godot::classes::Os::singleton().has_feature("arkit_debug") {
                        godot_print!("ArkitEvent::ArkitPayload:");
                        godot_print!("\tversion:{}", payload.version);
                        godot_print!("\tuuid:{}", payload.uuid);
                        godot_print!("\tname:{}", payload.name);
                        godot_print!("\tframe_number:{}", payload.frame_number);
                        godot_print!("\tsub_frame:{}", payload.sub_frame);
                        godot_print!("\tfps:{}", payload.fps);
                        godot_print!("\tdenominator:{}", payload.denominator);
                        godot_print!("\tblend_shapes:{:?}", payload.blend_shapes);
                    }
                    
                    let blend_shapes = PackedArray::<f32>::from(payload.blend_shapes[0..52].to_vec());
        
                    let head_yaw : f32 = payload.blend_shapes[52];
                    let head_pitch : f32 = payload.blend_shapes[53];
                    let head_roll : f32 = payload.blend_shapes[54];
                    let left_eye_yaw : f32 = payload.blend_shapes[55];
                    let left_eye_pitch : f32 = payload.blend_shapes[56];
                    let left_eye_roll : f32 = payload.blend_shapes[57];
                    let right_eye_yaw : f32 = payload.blend_shapes[58];
                    let right_eye_pitch : f32 = payload.blend_shapes[59];
                    let right_eye_roll : f32 = payload.blend_shapes[60];

                    // Convert this into a ref counted and emit it to godot
                    let live_link_face_blend_shapes =
                        LiveLinkFaceBlendShapes::create(
                            blend_shapes,
                            head_yaw,
                            head_pitch,
                            head_roll,
                            left_eye_yaw,
                            left_eye_pitch,
                            left_eye_roll,
                            right_eye_yaw,
                            right_eye_pitch,
                            right_eye_roll
                        );

                    self.signals()
                        .recv_live_link_face_blend_shapes()
                        .emit(&live_link_face_blend_shapes);
                }
            }
        }

        // See if the runtime was spawned correctly
        let Some(ref runtime) = self.runtime else {
            return;
        };

        // See if the connection resource is set
        let Some(ref hadalyth_arkit_connection) = self.hadalyth_arkit_connection else {
            return;
        };
        let port = hadalyth_arkit_connection.bind().port;

        // Attempt to get the local ip address on a timer if it's not available
        let Some(ref local_ip_address) = self.local_ip_address else {
            self.local_ip_address_timer -= delta;
            self.local_ip_address_timer = godot::global::maxf(0.0, self.local_ip_address_timer);
            if self.local_ip_address_timer > 0.0 {
                return;
            }
            self.local_ip_address_timer = 30.0;

            let tx = self.tx.clone();

            runtime.spawn(get_local_ip_address_async(tx));

            return;
        };

        // Attempt to open a socket if it isn't already open
        self.udp_socket_open_timer -= delta;
        self.udp_socket_open_timer = godot::global::maxf(0.0, self.udp_socket_open_timer);
        if self.udp_socket_open_timer > 0.0 {
            return;
        }
        self.udp_socket_open_timer = 30.0;

        if self.udp_socket_open {
            return;
        }
        self.udp_socket_open = true;

        let addr: String = format!("{}:{}", local_ip_address, port);
        godot_print!("{}", addr);

        let tx = self.tx.clone();
        runtime.spawn(open_udp_socket_async(addr, tx));
    }
}

#[godot_api]
impl HadalythArkit {
    #[signal]
    fn recv_live_link_face_blend_shapes(live_link_face_blend_shapes: Gd<LiveLinkFaceBlendShapes>);

    #[func]
    fn set_connection(&mut self, hadalyth_arkit_connection : Option<Gd<HadalythArkitConnection>>) {
        self.hadalyth_arkit_connection = hadalyth_arkit_connection;
    }
}
