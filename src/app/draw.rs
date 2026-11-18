use std::ptr::slice_from_raw_parts;

use vulkanalia::vk::{self, CommandBuffer, DeviceV1_0};

use super::App;
use crate::utils::color::RGBA;
use crate::utils::math::Matrix4;

impl App {
    pub unsafe fn draw(&self, command_buffer: CommandBuffer, transform: &Matrix4, color: &RGBA) {
        let transform_bytes = &*slice_from_raw_parts(
            transform as *const Matrix4 as *const u8,
            size_of::<Matrix4>(),
        );

        let color_bytes =
            &*slice_from_raw_parts(color as *const RGBA as *const u8, size_of::<RGBA>());

        self.device.cmd_push_constants(
            command_buffer,
            self.data.pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            transform_bytes,
        );

        self.device.cmd_push_constants(
            command_buffer,
            self.data.pipeline_layout,
            vk::ShaderStageFlags::FRAGMENT,
            64,
            color_bytes,
        );
        self.device
            .cmd_draw_indexed(command_buffer, self.data.indices.len() as u32, 1, 0, 0, 0);
    }
}
