use ash::vk;
use imgui::{Context, Ui};
use imgui_ash_renderer::Renderer;

pub struct ImGuiRenderer {
    pub context: Context,
    pub renderer: Renderer,
}

impl ImGuiRenderer {
    pub unsafe fn new(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        device: &ash::Device,
        queue: vk::Queue,
        queue_family: u32,
        render_pass: vk::RenderPass,
    ) -> Self {
        let mut context = Context::create();
        let renderer = Renderer::new(
            &mut context,
            instance,
            physical_device,
            device,
            queue,
            queue_family,
            render_pass,
        ).expect("Failed to create ImGui renderer");
        
        Self { context, renderer }
    }

    pub fn render_frame(&mut self, device: &ash::Device, command_buffer: vk::CommandBuffer) {
        let ui = self.context.frame();
        ui.show_demo_window(&mut true);
        
        self.renderer
            .cmd_draw(device, command_buffer, ui)
            .expect("ImGui rendering failed");
    }
}
