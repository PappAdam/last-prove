pub mod buffer;
pub mod desriptors;
pub mod image;
pub mod texture;

use ash::vk;

use super::utils::MAX_FRAME_DRAWS;

pub fn create_framebuffers(
    device: &ash::Device,
    swapchain_image_views: &Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    framebuffer_extent: vk::Extent2D,
    depth_img_view: vk::ImageView,
) -> Result<Vec<vk::Framebuffer>, String> {
    let mut framebuffers = Vec::with_capacity(swapchain_image_views.len());

    for (i, &view) in swapchain_image_views.iter().enumerate() {
        let attachments = [view, depth_img_view];

        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(framebuffer_extent.width)
            .height(framebuffer_extent.height)
            .layers(1)
            .build();

        let framebuffer = unsafe {
            device.create_framebuffer(&create_info, None).map_err(|_| {
                for &fb in &framebuffers {
                    device.destroy_framebuffer(fb, None);
                }
                format!("failed to create framebuffer {}", i)
            })?
        };

        framebuffers.push(framebuffer);
    }

    Ok(framebuffers)
}

pub fn create_semaphore(
    device: &ash::Device,
    object_name: &str,
) -> Result<Vec<vk::Semaphore>, String> {
    let mut semaphores = Vec::<vk::Semaphore>::with_capacity(MAX_FRAME_DRAWS);

    for _ in 0..MAX_FRAME_DRAWS {
        let create_info = vk::SemaphoreCreateInfo::default();

        let semaphore = unsafe {
            device
                .create_semaphore(&create_info, None)
                .map_err(|_| format!("failed to create {}", object_name))?
        };

        semaphores.push(semaphore);
    }

    Ok(semaphores)
}

pub fn create_fences(device: &ash::Device) -> Result<Vec<vk::Fence>, String> {
    let create_info = vk::FenceCreateInfo::builder()
        .flags(vk::FenceCreateFlags::SIGNALED)
        .build();

    let mut fences = Vec::with_capacity(MAX_FRAME_DRAWS as usize);

    for i in 0..MAX_FRAME_DRAWS {
        let fence = unsafe {
            device.create_fence(&create_info, None).map_err(|_| {
                for &f in &fences {
                    device.destroy_fence(f, None);
                }

                format!("failed to create fence {}", i)
            })?
        };

        fences.push(fence);
    }

    Ok(fences)
}

#[inline]
pub fn end_and_submit_command_buffer(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    queue: vk::Queue,
) -> Result<(), String> {
    let submit_info = vk::SubmitInfo::builder()
        .command_buffers(&[command_buffer])
        .build();

    unsafe {
        device
            .end_command_buffer(command_buffer)
            .map_err(|msg| format!("{}", msg))?;

        device
            .queue_submit(queue, &[submit_info], vk::Fence::null())
            .map_err(|msg| format!("{}", msg))?;
        device
            .queue_wait_idle(queue)
            .map_err(|msg| format!("{}", msg))?;
        device.free_command_buffers(command_pool, &[command_buffer]);
    }

    Ok(())
}

#[inline]
pub fn create_and_begin_command_buffer(
    device: &ash::Device,
    command_pool: vk::CommandPool,
) -> Result<vk::CommandBuffer, String> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .command_buffer_count(1)
        .level(vk::CommandBufferLevel::PRIMARY)
        .build();

    let command_buffer = unsafe {
        device
            .allocate_command_buffers(&allocate_info)
            .map_err(|msg| format!("{}", msg))?[0]
    };

    let begin_info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
        .build();

    unsafe {
        device
            .begin_command_buffer(command_buffer, &begin_info)
            .map_err(|msg| format!("{}", msg))?;
    }

    Ok(command_buffer)
}
