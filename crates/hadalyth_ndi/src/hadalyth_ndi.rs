use godot::prelude::*;
use godot::classes::Image;
use godot::classes::ImageTexture;
use godot::classes::RenderingServer;
use godot::classes::image::Format;
use godot::classes::TextureRect;
use godot::classes::ITextureRect;

use grafton_ndi;

use crate::hadalyth_ndi_enums::NdiEvent;

#[derive(GodotClass)]
#[class(base=TextureRect)]
struct HadalythNdi {

    tx : std::sync::mpsc::Sender<NdiEvent>,
    rx : std::sync::mpsc::Receiver<NdiEvent>,

    ndi : Result<grafton_ndi::NDI, grafton_ndi::Error>,
    framesync : Option<grafton_ndi::FrameSync>,
    
    last_recv_timecode : i64,
    recv_frames : i64,
    
    image_rid : Rid,
    image : Option<Gd<Image>>,
    image_buffer : PackedByteArray,

    pending_request : bool,

    base : Base<TextureRect>
}

#[godot_api]
impl ITextureRect for HadalythNdi {
    fn init(base : Base<TextureRect>) -> Self {

        let (tx, rx) = std::sync::mpsc::channel::<NdiEvent>();

        let ndi = grafton_ndi::NDI::new();
        let framesync : Option<grafton_ndi::FrameSync> = None;

        let last_recv_timecode = 0;
        let recv_frames = 0;

        let image_rid = Rid::Invalid;
        let image : Option<Gd<Image>> = None;
        let image_buffer = PackedByteArray::new();

        let pending_request : bool = false;

        return Self {
            tx,
            rx,
            ndi,
            framesync,
            last_recv_timecode,
            recv_frames,
            image_rid,
            image,
            image_buffer,
            pending_request,
            base
        }
    }

    fn ready(&mut self) {

    }


    fn process(&mut self, _delta : f32) {
        // Check for available sources request responses        
        while let Ok(event) = self.rx.try_recv() {
            match event {
                NdiEvent::SourcesFound(sources) => {
                    self.pending_request = false;        

                    let Ok(ref ndi) = self.ndi else {return};

                    // Assume the first one for now
                    let source = sources.first();
                    let Some(source) = source else {return};

                    // Create a receiver
                    let receiver_options = grafton_ndi::ReceiverOptions::builder(source.clone())
                        .color(grafton_ndi::ReceiverColorFormat::RGBX_RGBA)
                        .bandwidth(grafton_ndi::ReceiverBandwidth::Highest)
                        .build();
                    let receiver = grafton_ndi::Receiver::new(ndi, &receiver_options);
                    let Ok(receiver) = receiver else {return;};
                    let framesync = grafton_ndi::FrameSync::new(receiver);
                    let Ok(framesync) = framesync else {self.framesync = None; return};
                    self.framesync = Some(framesync);
                    
                    self.last_recv_timecode = 0;
                    self.recv_frames = 0;

                }
            }
        }

        // Do nothing if there is a pending request still
        if self.pending_request {
            return;
        }

        // Verify that we have an active source
        // try to get one if we do not
        {
            // No framesync, request a new list
            let Some(ref framesync) = self.framesync else {
                self._request_source_list();
                self.pending_request = true;
                return
            };
            // Framesync not connected, request a new list
            if !framesync.receiver().is_connected() {            
                self._request_source_list();
                self.pending_request = true;
                return;
            }
        };

        // Grab frames
        let (timecode, height, width, data, data_len) = {
            let Some(ref framesync) = self.framesync else {return};
            let Ok(video) = framesync.capture_video(grafton_ndi::ScanType::Progressive) else {return};
            let Some(video) = video else {return};

            let timecode = video.timecode();
            let height = video.height();
            let width = video.width();
            //let frame_rate_n = video.frame_rate_n();
            //let frame_rate_d = video.frame_rate_d();
            //let pixel_format = video.pixel_format();
            let data = video.data().as_ptr();
            let data_len = video.data().len();

            (timecode, height, width, data, data_len)
        };
    
        // Check if this is the same frame as last time (FrameSync may repeat frames)
        if timecode == self.last_recv_timecode && self.recv_frames > 0 {
            return;
        }
        self.last_recv_timecode = timecode;

        // Create the necessary image texture
        if self.recv_frames == 0 {
            godot_print!("First video frame received:");
            godot_print!("  Resolution: {}x{}", width, height);
            godot_print!("  Data size: {} bytes", data_len);
            godot_print!("");

            self.image_buffer = PackedByteArray::from(unsafe{std::slice::from_raw_parts(data, data_len)});

            // Create an image texture
            let image = Image::create_from_data(
                width, 
                height, 
                false, 
                Format::RGBA8, 
                &self.image_buffer
            );
            let Some(image) = image else {return};
            let image_texture = ImageTexture::create_from_image(&image);
            let Some(image_texture) = image_texture else {return};
            self.base_mut().set_texture(&image_texture);

            self.image_rid = image_texture.get_rid();
            self.image = image_texture.get_image();

        }

        // Update the current image texture
        else {
            let Some(ref mut image) = self.image else {return};
            //let image_slice = self.image_buffer.as_mut_slice();
            unsafe {
                std::ptr::copy_nonoverlapping(data, self.image_buffer.as_mut_slice().as_mut_ptr(), data_len);
            }
            image.set_data(width, height, false, Format::RGBA8, &self.image_buffer);
            let image : &Gd<Image> = image;

            RenderingServer::singleton().texture_2d_update(
                self.image_rid, 
                image,
                0
            );
        }
        self.recv_frames += 1;
    }
}

#[godot_api]
impl HadalythNdi {
    #[func]
    fn _request_source_list(&mut self) {
        let Ok(ref ndi) = self.ndi else {return};
        let source_list_tx = self.tx.clone();

        // Create a finder
        let finder_options = grafton_ndi::FinderOptions::builder()
            .show_local_sources(true)
            .groups("Public,Private")
            .build();
        let finder = grafton_ndi::Finder::new(ndi, &finder_options);
        let Ok(finder) = finder else {return};
        
        std::thread::spawn(move || {
            // Search for sources
            let sources = finder.find_sources(std::time::Duration::from_secs(5));
            let Ok(sources) = sources else {
                let _ = source_list_tx.send(NdiEvent::SourcesFound(vec![]));
                return;
            };
            let _ = source_list_tx.send(NdiEvent::SourcesFound(sources));
        });
    }
}