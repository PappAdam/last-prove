use std::mem::size_of;

use crate::{create_shader, resources::buffer::Buffer, utils::buffer_data::Vertex};
use ash::{util, vk};

use super::{
    base::RenderBase,
    resources::{self},
    utils::MAX_FRAME_DRAWS,
};

pub struct RenderData {
    pub pipeline_layout: vk::PipelineLayout,
    pub render_pass: vk::RenderPass,
    pub pipeline: vk::Pipeline,
    pub viewport: vk::Viewport,
    pub scissor: vk::Rect2D,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub img_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub fences: Vec<vk::Fence>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    //Buffers
    pub vertex_buffer: Buffer,
    pub instance_count: u32,
    pub index_buffer: Buffer,
    pub index_count: u32,
}

impl RenderData {
    pub fn new(base: &mut RenderBase) -> Result<Self, String> {
        let vertex_shader_module = create_shader!("../.compiled_shaders/vert.spv", base.device);
        let fragment_shader_module = create_shader!("../.compiled_shaders/frag.spv", base.device);

        let pipeline_layout = resources::create_pipeline_layout(&base.device)?;

        let render_pass = resources::create_render_pass(&base.device, base.surface_format.format)?;

        let pipeline = resources::create_pipelines(
            &base.device,
            vertex_shader_module,
            fragment_shader_module,
            pipeline_layout,
            render_pass,
        )?;

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: base.surface_extent.width as f32,
            height: base.surface_extent.height as f32,
            min_depth: 0.0f32,
            max_depth: 1.0f32,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: base.surface_extent.width,
                height: base.surface_extent.height,
            },
        };

        let framebuffers = resources::create_framebuffers(
            &base.device,
            &base.swapchain_image_views,
            render_pass,
            base.surface_extent,
        )?;

        let img_available_semaphores =
            resources::create_semaphore(&base.device, "img available semaphore")?;

        let render_finished_semaphores =
            resources::create_semaphore(&base.device, "rendering finished semaphore")?;

        let fences = resources::create_fences(&base.device)?;
        let command_pool = resources::create_command_pool(&base.device, base.queue_family)?;

        let command_buffers = {
            unsafe {
                let cb_info = vk::CommandBufferAllocateInfo::builder()
                    .command_pool(command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(MAX_FRAME_DRAWS as u32)
                    .build();
                base.device
                    .allocate_command_buffers(&cb_info)
                    .map_err(|err| format!("{}", err))?
            }
        };

        let vertecies = crate::utils::buffer_data::quad();
        let indicies = [0u16, 1, 3, 3, 2, 0];

        let vertex_buffer = Buffer::device_local(
            &base.device,
            vertecies.as_ptr() as *const _,
            size_of::<Vertex>() as u64 * vertecies.len() as u64,
            base.physical_device_memory_properties,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            base.queue,
            command_pool,
        )?;

        let index_buffer = Buffer::device_local(
            &base.device,
            indicies.as_ptr() as *const _,
            size_of::<u16>() as u64 * 6,
            base.physical_device_memory_properties,
            vk::BufferUsageFlags::INDEX_BUFFER,
            base.queue,
            command_pool,
        )?;

        Ok(Self {
            pipeline_layout,
            render_pass,
            pipeline,
            viewport,
            scissor,
            framebuffers,
            img_available_semaphores,
            render_finished_semaphores,
            fences,
            command_pool,
            command_buffers,

            //Buffers
            vertex_buffer,
            index_buffer,
            instance_count: vertecies.len() as u32,
            index_count: indicies.len() as u32,
        })
    }

    #[inline]
    pub fn resize(&mut self, vulkan_base: &RenderBase) -> Result<(), String> {
        unsafe {
            for &framebuffer in &self.framebuffers {
                vulkan_base.device.destroy_framebuffer(framebuffer, None);
            }
        }

        self.framebuffers = resources::create_framebuffers(
            &vulkan_base.device,
            &vulkan_base.swapchain_image_views,
            self.render_pass,
            vulkan_base.surface_extent,
        )?;

        Ok(())
    }

    pub fn clean_up(&self, device: &ash::Device) {
        unsafe {
            self.vertex_buffer.free(device);
            self.index_buffer.free(device);

            device.destroy_pipeline_layout(self.pipeline_layout, None);

            device.destroy_render_pass(self.render_pass, None);

            device.destroy_pipeline(self.pipeline, None);

            for &framebuffer in &self.framebuffers {
                device.destroy_framebuffer(framebuffer, None);
            }

            for i in 0..MAX_FRAME_DRAWS {
                device.destroy_semaphore(self.img_available_semaphores[i], None);
                device.destroy_semaphore(self.render_finished_semaphores[i], None);
                device.destroy_fence(self.fences[i], None);
            }
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
