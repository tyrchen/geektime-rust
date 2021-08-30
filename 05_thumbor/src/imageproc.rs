use crate::pb::*;
use anyhow::Result;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::{
    effects, filters, multiple, native::open_image_from_bytes, transform, PhotonImage,
};
use tracing::instrument;

lazy_static! {
    // 预先把水印文件加载为静态变量
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

#[instrument(level = "info", skip(data))]
pub fn process_image(data: Bytes, specs: &[Spec], format: ImageOutputFormat) -> Result<Vec<u8>> {
    let mut image = open_image_from_bytes(&data)?;
    for spec in specs.iter() {
        // 这个代码还可以用 enum_dispatch 进一步优化掉
        match spec.data {
            Some(spec::Data::Crop(ref v)) => v.transform(&mut image),
            Some(spec::Data::Contrast(ref v)) => v.transform(&mut image),
            Some(spec::Data::Filter(ref v)) => v.transform(&mut image),
            Some(spec::Data::Fliph(ref v)) => v.transform(&mut image),
            Some(spec::Data::Flipv(ref v)) => v.transform(&mut image),
            Some(spec::Data::Resize(ref v)) => v.transform(&mut image),
            Some(spec::Data::Watermark(ref v)) => v.transform(&mut image),
            None => {}
        }
    }

    Ok(image_to_buf(image, format))
}

// 使用 trait 可以统一处理的接口，以后无论增加多少功能，只需要加新的 Spec，然后实现 ImageTransform 接口
pub trait ImageTransform {
    fn transform(&self, image: &mut PhotonImage);
}

impl ImageTransform for Crop {
    fn transform(&self, image: &mut PhotonImage) {
        let img = transform::crop(image, self.x1, self.y1, self.x2, self.y2);
        *image = img;
    }
}

impl ImageTransform for Contrast {
    fn transform(&self, image: &mut PhotonImage) {
        effects::adjust_contrast(image, self.contrast);
    }
}

impl ImageTransform for Flipv {
    fn transform(&self, image: &mut PhotonImage) {
        transform::flipv(image)
    }
}
impl ImageTransform for Fliph {
    fn transform(&self, image: &mut PhotonImage) {
        transform::fliph(image)
    }
}

impl ImageTransform for Filter {
    fn transform(&self, image: &mut PhotonImage) {
        match filter::Filter::from_i32(self.filter) {
            Some(filter::Filter::Unspecified) => {}
            Some(f) => filters::filter(image, f.to_str().unwrap()),
            None => {}
        }
    }
}

impl ImageTransform for Resize {
    fn transform(&self, image: &mut PhotonImage) {
        let img = match resize::ResizeType::from_i32(self.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(
                image,
                self.width,
                self.height,
                resize::SampleFilter::from_i32(self.filter).unwrap().into(),
            ),
            resize::ResizeType::SeamCarve => transform::seam_carve(image, self.width, self.height),
        };
        *image = img;
    }
}

impl ImageTransform for Watermark {
    fn transform(&self, image: &mut PhotonImage) {
        multiple::watermark(image, &WATERMARK, self.x, self.y);
    }
}

// photon 库竟然没有提供在内存中对图片转换格式的方法，只好手工实现
fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = DynamicImage::ImageRgba8(img_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dynimage.write_to(&mut buffer, format).unwrap();
    buffer
}
