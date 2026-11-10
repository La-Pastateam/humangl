use imgui::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::window::Window;
use imgui::internal::RawWrapper;

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use std::ptr::copy_nonoverlapping as memcpy;

use crate::vulkan::buffer::{create_buffer};
use crate::app::AppData;
use crate::vulkan::image::{create_image, create_image_view, transition_image_layout, copy_buffer_to_image};
use crate::vulkan::pipeline::{create_shader_module};

#[derive(Debug)]
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
        // -------------------General Init----------------------
        // Link with window and imgui
        let mut imgui = Context::create();

        let mut platform = WinitPlatform::new(&mut imgui);
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);


        // -------------------Font setup----------------------
        // Get font pixels from gui
        let fonts = imgui.fonts();
        let font_atlas = fonts.build_rgba32_texture();

        // Staging buffer to store data in GPU later
        let image_size = (font_atlas.data.len() * std::mem::size_of::<u8>()) as vk::DeviceSize;
        let (staging_buffer, staging_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            image_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
        )?;

        // Put pixels in staging buffer
        let memory = device.map_memory(staging_buffer_memory, 0, image_size, vk::MemoryMapFlags::empty())?;

        memcpy(font_atlas.data.as_ptr(), memory.cast(), font_atlas.data.len());

        device.unmap_memory(staging_buffer_memory);

        // Create vk::image on cpu
        let (font_image, font_image_memory) = create_image(
            instance,
            device,
            data,
            font_atlas.width,
            font_atlas.height,
            1,
            vk::SampleCountFlags::_1,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        // Copy image from staging buffer to GPU
        transition_image_layout(
            device,
            data,
            font_image,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            1,
        )?;

        copy_buffer_to_image(
            device,
            data,
            staging_buffer,
            font_image,
            font_atlas.width,
            font_atlas.height
        )?;

        transition_image_layout(
            device,
            data,
            font_image,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            1,
        )?;


        // Create image view
        let font_image_view = create_image_view(
            device,
            font_image,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageAspectFlags::COLOR,
            1,
        )?;

        // Create Sampler
        let sampler_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);


        let font_sampler = device.create_sampler(&sampler_info, None)?;

        // Create Descriptor pool
        let pool_sizes = &[vk::DescriptorPoolSize {
            type_: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
        }];

        let pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(pool_sizes)
            .max_sets(1);

        let descriptor_pool = device.create_descriptor_pool(&pool_info, None)?;

        // Define Descriptor Layout
        let bindings = &[vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .build()];

        let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);

        let descriptor_set_layout = device.create_descriptor_set_layout(&layout_info, None)?;

        // Allocate Descriptor Set
        let layouts = &[descriptor_set_layout];

        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(descriptor_pool)
            .set_layouts(layouts);

        let font_descriptor_set = device.allocate_descriptor_sets(&alloc_info)?[0];

        // Connect Image View and Sampler to Descriptor Set
        let image_info = &[vk::DescriptorImageInfo::builder()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(font_image_view)
            .sampler(font_sampler)
            .build()];

        let descriptor_write = vk::WriteDescriptorSet::builder()
            .dst_set(font_descriptor_set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(image_info);

        let descriptor_copies: &[vk::CopyDescriptorSet] = &[];
        device.update_descriptor_sets(&[descriptor_write], descriptor_copies);


        // Destroy staging buffer
        device.destroy_buffer(staging_buffer, None);
        device.free_memory(staging_buffer_memory, None);


        // -------------------Pipeline setup----------------------
        // Define Push Constants
        let push_constant_ranges = &[vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size(16)
            .build()];
        
        // Create pipeline layout and bind push constants
        let set_layouts = &[descriptor_set_layout];
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(set_layouts)
            .push_constant_ranges(push_constant_ranges);

        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_info, None)?;

        // Describe struct imgui::DrawVert
        let binding_descriptions = &[vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<imgui::DrawVert>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()];

        let attribute_descriptions = &[
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(8)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                .format(vk::Format::R8G8B8A8_UNORM)
                .offset(16)
                .build(),
        ];

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(binding_descriptions)
            .vertex_attribute_descriptions(attribute_descriptions);


        // Shaders loading and config
        let vert_shader_code = include_bytes!("../shaders/imgui.vert.spv");
        let frag_shader_code = include_bytes!("../shaders/imgui.frag.spv");

        let vert_shader_module = create_shader_module(device, vert_shader_code)?;
        let frag_shader_module = create_shader_module(device, frag_shader_code)?;

        let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(b"main\0");

        let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(b"main\0");

        let shader_stages = &[vert_stage.build(), frag_stage.build()];


        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let dynamic_states = &[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(dynamic_states);

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(0)
            .scissor_count(0);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::CLOCKWISE);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::_1);

        let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(false)
            .depth_write_enable(false)
            .depth_compare_op(vk::CompareOp::ALWAYS);

        let color_blend_attachment = &[vk::PipelineColorBlendAttachmentState::builder()
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .alpha_blend_op(vk::BlendOp::ADD)
            .color_write_mask(
                vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A)
            .build()
        ];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(color_blend_attachment);

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .depth_stencil_state(&depth_stencil)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout)
            .render_pass(data.render_pass)
            .subpass(0);

        let pipeline = device.create_graphics_pipelines(
            vk::PipelineCache::null(),
            &[pipeline_info],
            None
        )?.0[0];

        device.destroy_shader_module(vert_shader_module, None);
        device.destroy_shader_module(frag_shader_module, None);

        // -------------------Vertex and Index setup----------------------

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
            pipeline,
            pipeline_layout,
            vertex_buffer,
            vertex_buffer_memory,
            vertex_buffer_size: initial_vtx_size,
            index_buffer,
            index_buffer_memory,
            index_buffer_size: initial_idx_size,
            font_image,
            font_image_memory,
            font_view: font_image_view,
            font_sampler,
            descriptor_pool,
            font_descriptor_set
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
        ui_data: &imgui::DrawData,
        swapchain_extent: vk::Extent2D
    ) -> Result<()> {
        if ui_data.display_size[0] <= 0.0 || ui_data.display_size[1] <= 0.0 {
            return Ok(());
        }

        // Statging buffer to store data on GPU
        let vertex_buffer_size = (ui_data.total_vtx_count as usize * std::mem::size_of::<imgui::DrawVert>()) as vk::DeviceSize;
        let index_buffer_size = (ui_data.total_idx_count as usize * std::mem::size_of::<u16>()) as vk::DeviceSize;

        if vertex_buffer_size == 0 || index_buffer_size == 0 {
            return Ok(());
        }

        let vtx_memory = device.map_memory(self.vertex_buffer_memory, 0, vertex_buffer_size, vk::MemoryMapFlags::empty())?;
        let mut vtx_dest = vtx_memory.cast::<imgui::DrawVert>();

        let idx_memory = device.map_memory(self.index_buffer_memory, 0, index_buffer_size, vk::MemoryMapFlags::empty())?;
        let mut idx_dest = idx_memory.cast::<u16>();

        for draw_list in ui_data.draw_lists() {
            let vtx_slice = draw_list.vtx_buffer();
            memcpy(vtx_slice.as_ptr(), vtx_dest, vtx_slice.len());
            vtx_dest = vtx_dest.add(vtx_slice.len());

            let idx_slice = draw_list.idx_buffer();
            memcpy(idx_slice.as_ptr(), idx_dest, idx_slice.len());
            idx_dest = idx_dest.add(idx_slice.len());
        }

        device.unmap_memory(self.vertex_buffer_memory);
        device.unmap_memory(self.index_buffer_memory);

        // Link Pipeline, Buffers, and Descriptors Sets
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline);

        device.cmd_bind_vertex_buffers(command_buffer, 0, &[self.vertex_buffer], &[0]);
        device.cmd_bind_index_buffer(command_buffer, self.index_buffer, 0, vk::IndexType::UINT16);

        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline_layout,
            0,
            &[self.font_descriptor_set],
            &[],
        );

        // Config Dynamic States
        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(swapchain_extent.width as f32)
            .height(swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);
        device.cmd_set_viewport(command_buffer, 0, &[viewport]);

        // Push constant Calculations
        let fb_width = ui_data.display_size[0] * ui_data.framebuffer_scale[0];
        let fb_height = ui_data.display_size[1] * ui_data.framebuffer_scale[1];

        let scale = [
            2.0 / ui_data.display_size[0],
            2.0 / ui_data.display_size[1]
        ];
        let translate = [
            -1.0 - ui_data.display_pos[0] * scale[0],
            -1.0 - ui_data.display_pos[1] * scale[1]
        ];

        // Send Push Constants
        let push_data: [f32; 4] = [scale[0], scale[1], translate[0], translate[1]];
        let push_bytes = unsafe {
            std::slice::from_raw_parts(
                push_data.as_ptr() as *const u8,
                std::mem::size_of_val(&push_data)
            )
        };

        device.cmd_push_constants(
            command_buffer,
            self.pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            push_bytes
        );


        // Loop on draw lists and send Draw Calls
        let clip_off = ui_data.display_pos;
        let clip_scale = ui_data.framebuffer_scale;

        let mut vtx_offset = 0;
        let mut idx_offset = 0;

        for draw_list in ui_data.draw_lists() {
            for cmd in draw_list.commands() {
                match cmd {
                    imgui::DrawCmd::Elements { count, cmd_params } => {
                        // Handle Scissor
                        let clip_rect = cmd_params.clip_rect;
                        let clip_min_x = clip_rect[0] - clip_off[0] * clip_scale[0];
                        let clip_min_y = clip_rect[1] - clip_off[1] * clip_scale[1];
                        let clip_max_x = clip_rect[2] - clip_off[0] * clip_scale[0];
                        let clip_max_y = clip_rect[3] - clip_off[1] * clip_scale[1];

                        if clip_max_x > clip_min_x && clip_max_y > clip_min_y {
                            let x = clip_min_x.clamp(0.0, fb_width) as i32;
                            let y = clip_min_y.clamp(0.0, fb_height) as u32;
                            let width = (clip_max_x - clip_min_x).clamp(0.0, fb_width - x as f32) as u32;
                            let height = (clip_max_y - clip_min_y).clamp(0.0, fb_height - y as f32) as u32;

                            let scissor = vk::Rect2D::builder()
                                .offset(vk::Offset2D { x, y: y as i32 })
                                .extent(vk::Extent2D { width, height });

                            device.cmd_set_scissor(command_buffer, 0, &[scissor]);

                            // Make draw call
                            device.cmd_draw_indexed(
                                command_buffer,
                                count as u32,
                                1,
                                (cmd_params.idx_offset + idx_offset) as u32,
                                (cmd_params.vtx_offset + vtx_offset) as i32,
                                0,
                            );
                        }
                    }
                    imgui::DrawCmd::ResetRenderState => {}
                    imgui::DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd);
                    },
                }
            }

            vtx_offset += draw_list.vtx_buffer().len();
            idx_offset += draw_list.idx_buffer().len();

        }
        Ok(())
    }

    pub unsafe fn destroy(&mut self, device: &Device) {
        device.destroy_pipeline(self.pipeline, None);
        device.destroy_pipeline_layout(self.pipeline_layout, None);
        device.destroy_buffer(self.vertex_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None);
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.index_buffer_memory, None);
        device.destroy_image(self.font_image, None);
        device.free_memory(self.font_image_memory, None);
        device.destroy_image_view(self.font_view, None);
        device.destroy_sampler(self.font_sampler, None);
        device.destroy_descriptor_pool(self.descriptor_pool, None);
    }
}
