use std::mem::size_of;

use crate::{
    create_shader,
    resources::{
        self,
        buffer::{Buffer, UniformBuffer},
        desriptors::{
            create_descriptor_pool, create_descriptor_set_layout, create_descriptor_sets,
        },
        image::Image,
    },
    setup,
    utils::buffer_data::{create_cube, BufferObject, Side, Transform, Vertex},
};
use ash::vk;

use nalgebra_glm::vec3;

use super::{base::RenderBase, utils::MAX_FRAME_DRAWS};

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
    pub depth_img: Image,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub descriptor_set_layout: vk::DescriptorSetLayout,

    //Buffer content
    pub transform: Transform,

    //Buffers
    pub vertex_buffer: Buffer,
    pub instance_count: u32,
    // pub index_buffer: Buffer,
    // pub index_count: u32,
    pub uniform_buffer: UniformBuffer,
}

impl RenderData {
    pub fn new(base: &mut RenderBase) -> Result<Self, String> {
        let vertex_shader_module = create_shader!("../.compiled_shaders/vert.spv", base.device);
        let fragment_shader_module = create_shader!("../.compiled_shaders/frag.spv", base.device);

        let descriptor_set_layout = create_descriptor_set_layout(&base.device)?;
        let pipeline_layout = setup::create_pipeline_layout(&base.device, descriptor_set_layout)?;

        let render_pass =
            setup::create_render_pass(&base.device, base.surface_format.format, base.depth_format)?;

        let pipeline = setup::create_pipelines(
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

        let depth_img = Image::new(
            &base.device,
            base.surface_extent.into(),
            base.depth_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::ImageAspectFlags::DEPTH,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            base.physical_device_memory_properties,
        )?;

        let framebuffers = resources::create_framebuffers(
            &base.device,
            &base.swapchain_image_views,
            render_pass,
            base.surface_extent,
            depth_img.view,
        )?;

        let img_available_semaphores =
            resources::create_semaphore(&base.device, "img available semaphore")?;

        let render_finished_semaphores =
            resources::create_semaphore(&base.device, "rendering finished semaphore")?;

        let fences = resources::create_fences(&base.device)?;
        let command_pool = setup::create_command_pool(&base.device, base.queue_family)?;

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

        let descriptor_pool = create_descriptor_pool(&base.device)?;

        let transform = Transform::new();
        let uniform_buffer = Buffer::uniform_buffer::<Transform>(
            &base.device,
            transform.as_void_ptr(),
            base.physical_device_memory_properties,
            0,
        )?;

        let descriptor_sets = create_descriptor_sets(
            &base.device,
            descriptor_pool,
            descriptor_set_layout,
            &uniform_buffer,
        )?;

        uniform_buffer.update(&base.device, transform.as_void_ptr(), &descriptor_sets);

        let mut vertecies = Vec::<Vertex>::new();
        vertecies.append(&mut create_cube(
            Side::LEFT | Side::FRONT | Side::TOP | Side::BOTTOM,
            vec3(-1., -0.5, -1.),
        ));
        vertecies.append(&mut create_cube(
            Side::RIGHT | Side::FRONT | Side::TOP | Side::BOTTOM,
            vec3(0., -0.5, -1.),
        ));
        vertecies.append(&mut create_cube(
            Side::LEFT | Side::BACK | Side::BOTTOM,
            vec3(-1., -0.5, 0.),
        ));
        vertecies.append(&mut create_cube(
            Side::LEFT | Side::BACK | Side::TOP | Side::FRONT | Side::RIGHT,
            vec3(-1., -1.5, 0.),
        ));
        vertecies.append(&mut create_cube(
            Side::RIGHT | Side::BACK | Side::TOP | Side::BOTTOM,
            vec3(0., -0.5, 0.),
        ));

        let vertex_buffer = Buffer::device_local(
            &base.device,
            vertecies.as_ptr() as *const _,
            size_of::<Vertex>() as u64 * vertecies.len() as u64,
            base.physical_device_memory_properties,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            base.queue,
            command_pool,
        )?;

        // let index_buffer = Buffer::device_local(
        //     &base.device,
        //     indicies.as_ptr() as *const _,
        //     size_of::<u16>() as u64 * indicies.len() as u64,
        //     base.physical_device_memory_properties,
        //     vk::BufferUsageFlags::INDEX_BUFFER,
        //     base.queue,
        //     command_pool,
        // )?;

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
            depth_img,
            descriptor_pool,
            descriptor_set_layout,
            descriptor_sets,

            transform,

            //Buffers
            vertex_buffer,
            // index_buffer,
            instance_count: vertecies.len() as u32,
            // index_count: indicies.len() as u32,
            uniform_buffer,
        })
    }

    #[inline]
    pub fn resize(&mut self, base: &RenderBase) -> Result<(), String> {
        unsafe {
            for &framebuffer in &self.framebuffers {
                base.device.destroy_framebuffer(framebuffer, None);
            }
        }

        self.depth_img.free(&base.device);

        self.depth_img = Image::new(
            &base.device,
            base.surface_extent.into(),
            base.depth_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::ImageAspectFlags::DEPTH,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            base.physical_device_memory_properties,
        )?;

        self.framebuffers = resources::create_framebuffers(
            &base.device,
            &base.swapchain_image_views,
            self.render_pass,
            base.surface_extent,
            self.depth_img.view,
        )?;

        Ok(())
    }

    pub fn clean_up(&self, device: &ash::Device) {
        unsafe {
            self.vertex_buffer.free(device);
            // self.index_buffer.free(device);
            self.uniform_buffer.free(device);

            self.depth_img.free(device);

            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

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
