use ash::vk;

use crate::utils::MAX_FRAME_DRAWS;

use super::buffer::{DynamicUniformBuffer, UniformBuffer};

pub fn update_descriptor_sets(device: &ash::Device, descriptor_sets: &[vk::DescriptorSet], write_desc_sets: &mut [vk::WriteDescriptorSet]) {
    descriptor_sets.iter().for_each(|set| {
        write_desc_sets.iter_mut().for_each(|write| {
            write.dst_set = *set;
        });
        unsafe { device.update_descriptor_sets(&write_desc_sets, &[]) };
    });
}

pub fn create_descriptor_sets(
    device: &ash::Device,
    descriptor_pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
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

    Ok(descriptor_sets)
}

pub fn create_descriptor_set_layout(
    device: &ash::Device,
    layout_bindings: &[vk::DescriptorSetLayoutBinding]
) -> Result<vk::DescriptorSetLayout, String> {


    let layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(layout_bindings)
        .build();

    let layout = unsafe {
        device
            .create_descriptor_set_layout(&layout_create_info, None)
            .map_err(|err| format!("{err}"))?
    };

    Ok(layout)
}

pub fn create_descriptor_pool(device: &ash::Device, pool_sizes: &[vk::DescriptorPoolSize]) -> Result<vk::DescriptorPool, String> {
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
