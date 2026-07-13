
use fast_image_resize;

pub struct HadalythFrameResizer {
    resizer: fast_image_resize::Resizer,
    dst: fast_image_resize::images::Image<'static>,
}

impl HadalythFrameResizer {
    pub fn new() -> Self {
        Self {
            resizer: fast_image_resize::Resizer::new(),
            dst: fast_image_resize::images::Image::new(
                2560, 
                1440, 
                fast_image_resize::PixelType::U8x4
            ),
        }
    }

    pub fn resize<'a>(
        &'a mut self,
        width: i64,
        height: i64,
        data: &'a [u8],
    ) -> (i64, i64, Option<&'a [u8]>) {
        if width <= 2560 && height <= 1440 {
            return (width, height, Some(data));
        }

        let src = fast_image_resize::images::ImageRef::new(
            width as u32,
            height as u32,
            data,
            fast_image_resize::PixelType::U8x4,
        );
        let Ok(src) = src else {
            return (width, height, Some(data));
        };

        let options = fast_image_resize::ResizeOptions::new()
            .resize_alg(fast_image_resize::ResizeAlg::Nearest)
            .use_alpha(false)
            .fit_into_destination(Some((0.5,0.5)));

        match self.resizer.resize(&src, &mut self.dst, &options) {
            Ok(()) => {}
            Err(_err) => {
                return (width, height, Some(data));    
            }
        }

        (2560, 1440, Some(self.dst.buffer()))
    }
}