use super::{Engine, SpecTransformer};
use crate::pb::*;
use anyhow::Result;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageFormat};
use lazy_static::lazy_static;
use photon_rs::{
    effects, filters, native::open_image_from_bytes, transform, PhotonImage, multiple};
use std::convert::TryFrom;
use std::io::Cursor;

lazy_static! {
    // 预加载水印文件
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

// Photon Engine 实现
pub struct Photon(PhotonImage);

// 从 Bytes 转换成 Photon 结构
impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;
    
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[crate::Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                Some(spec::Data::Rotate(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self, format: image::ImageFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpecTransformer<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let img = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = img;
    }
}

impl SpecTransformer<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpecTransformer<&Flipv> for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0)
    }
}

impl SpecTransformer<&Fliph> for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0)
    }
}

impl SpecTransformer<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::try_from(op.filter).unwrap() {
            filter::Filter::Unspecified => {},
            f => filters::filter(&mut self.0, f.to_str().unwrap())
        }
    }
}

impl SpecTransformer<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::ResizeType::try_from(op.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(
                &mut self.0, 
                op.width, 
                op.height, 
                resize::SampleFilter::try_from(op.filter).unwrap().into(),
            ),
            resize::ResizeType::SeamCarve => transform::seam_carve(&mut self.0, op.width, op.height),
        };
        self.0 = img;
    }
}

impl SpecTransformer<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}

impl SpecTransformer<&Rotate> for Photon {
    fn transform(&mut self, op: &Rotate) {
        transform::rotate(&mut self.0, op.angle);
    }
}

/// 内存中图片转换格式
fn image_to_buf(img: PhotonImage, format: ImageFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dyn_img = DynamicImage::ImageRgb8(img_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dyn_img.write_to(&mut Cursor::new(&mut buffer), format).unwrap();
    buffer
}