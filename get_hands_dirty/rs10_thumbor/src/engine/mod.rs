use crate::pb::Spec;
use image::ImageFormat;

mod photon;
pub use photon::Photon;

// Engine trait: 未来可以添加更多的 engine 实现
pub trait Engine {
    // 对 engine 按照 specs 进行一系列的处理
    fn apply(&mut self, specs: &[Spec]);
    // 从 engine 中生成目标图片
    fn generate(self, format: ImageFormat) -> Vec<u8>;
}

// SpecTransformer trait: 未来可以添加更多的 spec 实现
pub trait SpecTransformer<T> {
    // 对图片使用 op 做 transform 操作
    fn transform(&mut self, op: T);
}

