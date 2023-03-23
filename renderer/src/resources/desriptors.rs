use std::mem::size_of;

use ash::vk;

use crate::utils::MAX_FRAME_DRAWS;

use super::{
    buffer::{Buffer, UniformBuffer},
    image::Image,
};

pub fn create_descriptor_sets(
    device: &ash::Device,
    descriptor_pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
    texture: &Image,
    sampler: vk::Sampler,
    uniform_buffer: &UniformBuffer,
) -> Result<Vec<vk::DescriptorSet>, String> {
    let layouts = [layout; MAX_FRAME_DRAWS];
    let desc_alloc_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(descriptor_pool)
        .set_layouts(&layouts);
    let descriptor_sets = unsafe {
        device
            .allocate_descriptor_sets(&desc_alloc_info)
            .map_err(|err| format!("{err}"))?
    };

    descriptor_sets.iter().for_each(|set| {
        let tex_descriptor = vk::DescriptorImageInfo {
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            image_view: texture.view,
            sampler,
        };

        let uniform_buffer_descriptor = vk::DescriptorBufferInfo {
            buffer: uniform_buffer.buf,
            offset: 0,
            range: uniform_buffer.size,
        };

        let write_desc_sets = [
            vk::WriteDescriptorSet {
                dst_set: *set,
                dst_binding: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                p_buffer_info: &uniform_buffer_descriptor,
                ..Default::default()
            },
            vk::WriteDescriptorSet {
                dst_set: *set,
                dst_binding: 1,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                p_image_info: &tex_descriptor,
                ..Default::default()
            },
        ];
        unsafe { device.update_descriptor_sets(&write_desc_sets, &[]) };
    });

    Ok(descriptor_sets)
}

pub fn create_descriptor_set_layout(
    device: &ash::Device,
) -> Result<vk::DescriptorSetLayout, String> {
    let layout_bindings = [
        vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
            ..Default::default()
        },
        vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        },
    ];

    let layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(&layout_bindings)
        .build();

    let layout = unsafe {
        device
            .create_descriptor_set_layout(&layout_create_info, None)
            .map_err(|err| format!("{err}"))?
    };

    Ok(layout)
}

pub fn create_descriptor_pool(device: &ash::Device) -> Result<vk::DescriptorPool, String> {
    let pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: MAX_FRAME_DRAWS as u32,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: MAX_FRAME_DRAWS as u32,
        },
    ];

    let create_info = vk::DescriptorPoolCreateInfo::builder()
        .max_sets(MAX_FRAME_DRAWS as u32)
        .pool_sizes(&pool_sizes)
        .build();

    let descriptor_pool = unsafe {
        device
            .create_descriptor_pool(&create_info, None)
            .map_err(|err| format!("{err}"))?
    };

    Ok(descriptor_pool)
}
