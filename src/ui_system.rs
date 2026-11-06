use imgui::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use winit::window::{Window, WindowAttributes};

use vulkanalia::prelude::v1_0::*;

use crate::vulkan::buffer::{create_buffer};
use crate::app::AppData;

pub struct UiSystem {
    pub imgui: Context,
    pub platform: WinitPlatform,

    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,

    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub vertex_buffer_size: vk::DeviceSize,

    pub index_buffer: vk::Buffer,
    pub index_buffer_memory: vk::DeviceMemory,
    pub index_buffer_size: vk::DeviceSize,

    pub font_image: vk::Image,
    pub font_image_memory: vk::DeviceMemory,
    pub font_view: vk::ImageView,
    pub font_sampler: vk::Sampler,
    pub descriptor_pool: vk::DescriptorPool,
    pub font_descriptor_set: vk::DescriptorSet,
}

impl UiSystem {
    pub unsafe fn new(
        instance: &Instance,
        window: &Window,
        device: &Device,
        data: &mut AppData,
        ) -> Result<Self> {
        // Link with window and imgui
        let mut imgui = Context::create();

        let mut platform = WinitPlatform::new(&mut imgui);
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);

        let fonts = imgui.fonts();
        let font_atlas = fonts.build_rgba32_texture();

        let initial_vtx_size = (5000 * std::mem::size_of::<imgui::DrawVert>()) as vk::DeviceSize;
        let initial_idx_size = (10000 * std::mem::size_of::<u16>()) as vk::DeviceSize;

        let (vertex_buffer, vertex_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            initial_vtx_size,
        vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE, 
        )?;

        let (index_buffer, index_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            initial_idx_size,
        vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE, 
        )?;

        Ok(Self {
            imgui,
            platform,
            pipeline: vk::Pipeline::null(),
            pipeline_layout: vk::PipelineLayout::null(),
            vertex_buffer,
            vertex_buffer_memory,
            vertex_buffer_size: initial_vtx_size,
            index_buffer,
            index_buffer_memory,
            index_buffer_size: initial_idx_size,
            font_image: vk::Image::null(),
            font_image_memory: vk::DeviceMemory::null(),
            font_view: vk::ImageView::null(),
            font_sampler: vk::Sampler::null(),
            descriptor_pool: vk::DescriptorPool::null(),
            font_descriptor_set: vk::DescriptorSet::null()
        })
    }

    pub fn prepare_frame(&mut self, window: &Window) -> Result<()> {
        self.platform.prepare_frame(self.imgui.io_mut(), window)?;
        return Ok(());
    }

    pub unsafe fn commands(
        &mut self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        ui_data: &imgui::DrawData
    ) -> Result<()> {
        if ui_data.display_size[0] <= 0.0 || ui_data.display_size[1] <= 0.0 {
            return Ok(());
        }

        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline);

        device.cmd_bind_vertex_buffers(command_buffer, 0, &[self.vertex_buffer], &[0]);
        device.cmd_bind_index_buffer(command_buffer, self.index_buffer, 0, vk::IndexType::UINT32);

        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline_layout,
            0,
            &[self.font_descriptor_set],
            &[],
        );


        Ok(())
    }

    pub unsafe fn destroy(&mut self, device: &Device) {
        device.destroy_pipeline(self.pipeline, None);
        device.destroy_pipeline_layout(self.pipeline_layout, None);
        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None);
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.index_buffer_memory, None);
    }
}
