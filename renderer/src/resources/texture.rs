use ash::{util::Align, vk};

use crate::parse_error;

use super::{
    buffer::find_memory_type_index, create_and_begin_command_buffer, end_and_submit_command_buffer,
    image::Image,
};

impl Image {
    pub fn create_sampler(device: &ash::Device) -> vk::Sampler {
        let sampler_info = vk::SamplerCreateInfo {
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
            address_mode_u: vk::SamplerAddressMode::MIRRORED_REPEAT,
            address_mode_v: vk::SamplerAddressMode::MIRRORED_REPEAT,
            address_mode_w: vk::SamplerAddressMode::MIRRORED_REPEAT,
            max_anisotropy: 1.0,
            border_color: vk::BorderColor::FLOAT_OPAQUE_WHITE,
            compare_op: vk::CompareOp::NEVER,
            ..Default::default()
        };

        unsafe { device.create_sampler(&sampler_info, None).unwrap() }
    }

    pub fn texture(
        device: &ash::Device,
        image: &[u8],
        memory_props: vk::PhysicalDeviceMemoryProperties,
        command_pool: vk::CommandPool,
        queue: vk::Queue,
    ) -> Result<Self, String> {
        let image = image::load_from_memory(image).unwrap().to_rgba8();
        let (width, height) = image.dimensions();
        let image_extent = vk::Extent2D { width, height };
        let image_data = image.into_raw();
        let image_buffer_info = vk::BufferCreateInfo {
            size: (std::mem::size_of::<u8>() * image_data.len()) as u64,
            usage: vk::BufferUsageFlags::TRANSFER_SRC,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let image_buffer = unsafe { device.create_buffer(&image_buffer_info, None).unwrap() };
        let image_buffer_memory_req =
            unsafe { device.get_buffer_memory_requirements(image_buffer) };
        let image_buffer_memory_index = find_memory_type_index(
            memory_props,
            image_buffer_memory_req.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        let image_buffer_allocate_info = vk::MemoryAllocateInfo {
            allocation_size: image_buffer_memory_req.size,
            memory_type_index: image_buffer_memory_index,
            ..Default::default()
        };
        let image_buffer_memory = unsafe {
            device
                .allocate_memory(&image_buffer_allocate_info, None)
                .map_err(|err| parse_error!(err))?
        };
        let image_ptr = unsafe {
            device
                .map_memory(
                    image_buffer_memory,
                    0,
                    image_buffer_memory_req.size,
                    vk::MemoryMapFlags::empty(),
                )
                .map_err(|err| parse_error!(err))?
        };
        let mut image_slice = unsafe {
            Align::new(
                image_ptr,
                std::mem::align_of::<u8>() as u64,
                image_buffer_memory_req.size,
            )
        };
        image_slice.copy_from_slice(&image_data);
        unsafe {
            device.unmap_memory(image_buffer_memory);
            device
                .bind_buffer_memory(image_buffer, image_buffer_memory, 0)
                .map_err(|err| parse_error!(err))?;
        }

        let texture_img = Self::create_image(
            device,
            image_extent.into(),
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        )?;

        let texture_mem = Self::create_mem(
            device,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            memory_props,
            texture_img,
        )?;

        let texture_barrier = vk::ImageMemoryBarrier {
            dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            image: texture_img,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: 1,
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let texture_command_buffer = create_and_begin_command_buffer(device, command_pool)?;

        unsafe {
            device.cmd_pipeline_barrier(
                texture_command_buffer,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[texture_barrier],
            );
            let buffer_copy_regions = vk::BufferImageCopy::builder()
                .image_subresource(
                    vk::ImageSubresourceLayers::builder()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1)
                        .build(),
                )
                .image_extent(image_extent.into());

            device.cmd_copy_buffer_to_image(
                texture_command_buffer,
                image_buffer,
                texture_img,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[buffer_copy_regions.build()],
            );

            let texture_barrier_end = vk::ImageMemoryBarrier {
                src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                dst_access_mask: vk::AccessFlags::SHADER_READ,
                old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                image: texture_img,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };

            device.cmd_pipeline_barrier(
                texture_command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[texture_barrier_end],
            );
        }

        end_and_submit_command_buffer(device, command_pool, texture_command_buffer, queue)?;

        let texture_view = Self::create_image_view(
            device,
            texture_img,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageAspectFlags::COLOR,
        )?;

        unsafe {
            device.destroy_buffer(image_buffer, None);
            device.free_memory(image_buffer_memory, None);
        }

        Ok(Self {
            img: texture_img,
            mem: texture_mem,
            view: texture_view,
        })
    }
}
