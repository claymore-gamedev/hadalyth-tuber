use godot::prelude::*;
use godot::classes::Node;
use godot::classes::INode;

use grafton_ndi;

use crate::hadalyth_ndi_enums::NdiEvent;
use crate::hadalyth_frame_resizer::HadalythFrameResizer;


#[derive(GodotClass)]
#[class(base=Node)]
struct HadalythNdi {

    resizer : HadalythFrameResizer,
    
    tx : std::sync::mpsc::Sender<NdiEvent>,
    rx : std::sync::mpsc::Receiver<NdiEvent>,

    ndi : Result<grafton_ndi::NDI, grafton_ndi::Error>,
  
    sources : Vec<grafton_ndi::Source>,

    video_framesync : Option<grafton_ndi::FrameSync>,
    video_framesync_timer : f32,
    video_framesync_last_recv_timecode : i64,

    audio_framesync : Option<grafton_ndi::FrameSync>,
    audio_framesync_timer : f32,

    base : Base<Node>

}


#[godot_api]
impl INode for HadalythNdi {

    fn init(base : Base<Node>) -> Self {

        let resizer = HadalythFrameResizer::new();

        let (tx, rx) = std::sync::mpsc::channel::<NdiEvent>();

        let ndi = grafton_ndi::NDI::new();
      
        let sources = vec![];

        let video_framesync : Option<grafton_ndi::FrameSync> = None;
        let video_framesync_timer = 0.0;
        let video_framesync_last_recv_timecode = 0;
        
        let audio_framesync : Option<grafton_ndi::FrameSync> = None;
        let audio_framesync_timer = 0.0;
        
        return Self {

            resizer,

            tx,
            rx,

            ndi,

            sources,

            video_framesync,
            video_framesync_timer,
            video_framesync_last_recv_timecode,
            
            audio_framesync,
            audio_framesync_timer,
            
            base
        }
    }


    fn ready(&mut self) {

    }


    fn process(&mut self, delta : f32) {
        // Check for available sources request responses        
        while let Ok(event) = self.rx.try_recv() {
            match event {
                NdiEvent::SourcesFound{ sources } => {
                    self.sources = sources;

                    // Emit a signal here with their string names
                    let source_names : Vec<GString> = self.sources.iter().map(|x|{x.name.to_godot()}).collect();
                    let source_names : PackedStringArray = PackedStringArray::from(source_names);
                    self.signals().recv_source_list().emit(
                        &source_names
                    );
                }
            }
        }


        self._poll_video(delta);
        self._poll_audio(delta);


    }
}


#[godot_api]
impl HadalythNdi {


    #[signal]
    fn recv_source_list(source_names : PackedStringArray);


    #[signal]
    fn recv_source_video_data(width : i64, height : i64, video_data : PackedByteArray);


    #[signal]
    fn recv_source_audio_data(audio_data : PackedVector2Array);


    #[func]
    fn _poll_video(&mut self, delta : f32) {
        // Gate after for video
        self.video_framesync_timer += delta;
        if self.video_framesync_timer < 1.0 / 30.0 {
            return;
        }
        self.video_framesync_timer -= 1.0 / 30.0;
        
        // Grab frames, this only works if the framesync is active
        let frame = {
            let Some(ref framesync) = self.video_framesync else {return};
            let Ok(video) = framesync.capture_video(
                grafton_ndi::ScanType::Progressive
            ) else {return};
            let Some(video) = video else {return};
            
            let timecode = video.timecode();
            if self.video_framesync_last_recv_timecode == timecode {
                return;
            }
            self.video_framesync_last_recv_timecode = timecode;
            
            let height = video.height();
            let width = video.width();
            let data = video.data();

            let (width, height, data) = self.resizer.resize(
                width as i64, 
                height as i64,
                data
            );

            let Some(data) = data else {
                return;
            };
            (width, height, PackedByteArray::from(data))
        };

        let (width, height, data) = frame;

        self.signals().recv_source_video_data().emit(
            width, 
            height,
            &PackedByteArray::from(data)
        );
        
    }


    #[func]
    fn _poll_audio(&mut self, delta : f32) {

        // Gate before for audio
        self.audio_framesync_timer += delta;
        if self.audio_framesync_timer < 0.02 {
            return;
        }

        self.audio_framesync_timer -= 0.02;
        let data = {
            let Some(ref framesync) = self.audio_framesync else {return};
            let audio = framesync.capture_audio(
                grafton_ndi::FrameSyncAudioRequest::Capture{ 
                    sample_rate: Some(std::num::NonZero::new(48000).unwrap()), 
                    channels: Some(std::num::NonZero::new(1).unwrap()), 
                    samples: std::num::NonZero::new(960).unwrap(), 
                }
            );
            let Ok(audio) = audio else {return};           
            let data = audio.data();

            let data : Vec<Vector2> = data.iter().map(
                |x| {
                    let s = (*x).clamp(-1.0, 1.0);
                    Vector2::new(s, s)
                }
            ).collect();

            let data = PackedVector2Array::from(data);

            data
        };

        self.signals().recv_source_audio_data().emit(
            &data
        );

    }


    #[func]
    fn request_source_list(&mut self) {
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
                let _ = source_list_tx.send(NdiEvent::SourcesFound{sources:vec![]});
                return;
            };
            let _ = source_list_tx.send(NdiEvent::SourcesFound{sources});
        });

    }


    #[func]
    fn connect_to_source_video(&mut self, source_name : String) -> bool {
        let Ok(ref ndi) = self.ndi else {return false};

        // Clear out old frame sync and data
        self.video_framesync = None;
        self.video_framesync_timer = 0.0;
        self.video_framesync_last_recv_timecode = 0;
        
        // Get the source by name if it's currently in the map
        let source = self.sources.iter().find(|x| {return x.name == source_name});
        let Some(source) = source else {return false};      

        // Create a receiver
        let receiver_options = grafton_ndi::ReceiverOptions::builder(source.clone())
            .color(grafton_ndi::ReceiverColorFormat::RGBX_RGBA)
            .bandwidth(grafton_ndi::ReceiverBandwidth::Highest)
            .build();
        let receiver = grafton_ndi::Receiver::new(ndi, &receiver_options);
        let Ok(receiver) = receiver else {return false;};

        // Create a framesync from the receiver
        let framesync = grafton_ndi::FrameSync::new(receiver);
        let Ok(framesync) = framesync else {self.video_framesync = None; return false};
        self.video_framesync = Some(framesync);       

        return true;
    }


    #[func]
    fn connect_to_source_audio(&mut self, source_name : String) -> bool {
        let Ok(ref ndi) = self.ndi else {return false};

        // Clear out old frame sync and data
        self.audio_framesync = None;
        self.audio_framesync_timer = 0.0;

        // Get the source by name if it's currently in the map
        let source = self.sources.iter().find(|x| {return x.name == source_name});
        let Some(source) = source else {return false};      

        // Create a receiver
        let receiver_options = grafton_ndi::ReceiverOptions::builder(source.clone())
            .color(grafton_ndi::ReceiverColorFormat::RGBX_RGBA)
            .bandwidth(grafton_ndi::ReceiverBandwidth::AudioOnly)
            .build();
        let receiver = grafton_ndi::Receiver::new(ndi, &receiver_options);
        let Ok(receiver) = receiver else {return false;};

        // Create a framesync from the receiver
        let framesync = grafton_ndi::FrameSync::new(receiver);
        let Ok(framesync) = framesync else {self.audio_framesync = None; return false};
        self.audio_framesync = Some(framesync);       

        return true;

    }


    #[func]
    fn disconnect(&mut self) {

        self.video_framesync = None;
        self.video_framesync_timer = 0.0;
        self.video_framesync_last_recv_timecode = 0;

        self.audio_framesync = None;
        self.audio_framesync_timer = 0.0;

    }

}