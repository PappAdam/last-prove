use std::sync::Arc;

use vulkano::{device::Device, image::{SwapchainImage, ImageAccess, view::ImageView, AttachmentImage}, pipeline::graphics::viewport::Viewport, render_pass::{RenderPass, Framebuffer, FramebufferCreateInfo}, format::Format, swapchain::{SwapchainCreationError, SwapchainCreateInfo, SwapchainAcquireFuture}};
use winit::window::Window;

use super::VulkanApp;

impl VulkanApp {
    pub fn window_size_dependent_setup(
        device: Arc<Device>,
        images: &[Arc<SwapchainImage<Window>>],
        viewport: &mut Viewport,
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        let depth_buffer = ImageView::new_default(
            AttachmentImage::transient(device.clone(), dimensions, Format::D16_UNORM).unwrap(),
        )
        .unwrap();

        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_buffer.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }

    pub fn recreate_swapchain(&mut self) {
        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: self.surface.window().inner_size().into(),
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
        self.swapchain = new_swapchain;
        self.framebuffers = Self::window_size_dependent_setup(
            self.device.clone(),
            &new_images,
            &mut self.viewport,
            self.render_pass.clone(),
        );
        self.recreate_swapchain = false;
    }

    pub fn acquire_swapchain_image(&mut self) -> Option<SwapchainAcquireFuture<Window>> {
        let (image_num, suboptimal, acquire_future) =
            match vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return None;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };
        if suboptimal {
            self.recreate_swapchain = true;
        }
        self.draw_image_index = image_num;
        Some(acquire_future)
    }
}