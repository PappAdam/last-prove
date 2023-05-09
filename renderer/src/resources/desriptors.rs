use std::mem::size_of;

use ash::vk;

use crate::utils::MAX_FRAME_DRAWS;

use super::{
    buffer::{Buffer, DynamicUniformBuffer, UniformBuffer},
    image::Image,
};

pub fn create_descriptor_sets(
    device: &ash::Device,
    descriptor_pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
    dynamic_uniform_buffer: &DynamicUniformBuffer,
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
        let uniform_buffer_descriptor = vk::DescriptorBufferInfo {
            buffer: uniform_buffer.buf,
            offset: 0,
            range: uniform_buffer.size,
        };

        let dynamic_uniform_buffer_descriptor = vk::DescriptorBufferInfo {
            buffer: dynamic_uniform_buffer.buf,
            offset: 0,
            range: dynamic_uniform_buffer.size,
        };

        let write_desc_sets = [
            vk::WriteDescriptorSet {
                dst_set: *set,
                dst_binding: uniform_buffer.binding,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                p_buffer_info: &uniform_buffer_descriptor,
                ..Default::default()
            },
            vk::WriteDescriptorSet {
                dst_set: *set,
                dst_binding: dynamic_uniform_buffer.binding,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                p_buffer_info: &dynamic_uniform_buffer_descriptor,
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
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
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
            ty: vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
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
