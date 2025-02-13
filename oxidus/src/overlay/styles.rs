pub fn set_styles(style: &mut imgui::Style) {
    // Reset to default first to purge inherited settings

    // Razor-sharp geometry
    style.window_rounding = 0.0;
    style.child_rounding = 0.0;
    style.frame_rounding = 0.0;
    style.popup_rounding = 0.0;
    style.scrollbar_rounding = 0.0;
    style.grab_rounding = 0.0;
    style.tab_rounding = 0.0;

    // Surgical border control
    style.window_border_size = 1.0;
    style.child_border_size = 1.0;
    style.popup_border_size = 1.0;
    style.frame_border_size = 1.0;
    style.tab_border_size = 1.0;

    // Compressed layout metrics
    style.window_padding = [6.0, 6.0];
    style.frame_padding = [6.0, 4.0];
    style.item_spacing = [6.0, 4.0];
    style.item_inner_spacing = [4.0, 2.0];
    style.touch_extra_padding = [0.0, 0.0];
    style.indent_spacing = 12.0;
    style.scrollbar_size = 8.0;

    // Absolute monochrome palette (RGB equal values only)
    let black = [0.0, 0.0, 0.0, 1.0];
    let gray_10 = [0.1, 0.1, 0.1, 1.0];
    let gray_20 = [0.2, 0.2, 0.2, 1.0];
    let gray_30 = [0.3, 0.3, 0.3, 1.0];
    let gray_40 = [0.4, 0.4, 0.4, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    // Nuclear option: Set every color explicitly
    let colors = &mut style.colors;

    // Base
    colors[imgui::StyleColor::Text as usize] = white;
    colors[imgui::StyleColor::TextDisabled as usize] = gray_40;
    colors[imgui::StyleColor::WindowBg as usize] = black;
    colors[imgui::StyleColor::ChildBg as usize] = gray_10;
    colors[imgui::StyleColor::PopupBg as usize] = gray_10;

    // Borders
    colors[imgui::StyleColor::Border as usize] = gray_30;
    colors[imgui::StyleColor::BorderShadow as usize] = black;

    // Frame
    colors[imgui::StyleColor::FrameBg as usize] = gray_10;
    colors[imgui::StyleColor::FrameBgHovered as usize] = gray_20;
    colors[imgui::StyleColor::FrameBgActive as usize] = gray_30;

    // Title
    colors[imgui::StyleColor::TitleBg as usize] = black;
    colors[imgui::StyleColor::TitleBgActive as usize] = black;
    colors[imgui::StyleColor::TitleBgCollapsed as usize] = black;

    // Scrollbar
    colors[imgui::StyleColor::ScrollbarBg as usize] = black;
    colors[imgui::StyleColor::ScrollbarGrab as usize] = gray_20;
    colors[imgui::StyleColor::ScrollbarGrabHovered as usize] = gray_30;
    colors[imgui::StyleColor::ScrollbarGrabActive as usize] = gray_40;

    // Checkmarks
    colors[imgui::StyleColor::CheckMark as usize] = white;

    // Sliders
    colors[imgui::StyleColor::SliderGrab as usize] = gray_30;
    colors[imgui::StyleColor::SliderGrabActive as usize] = gray_40;

    // Buttons
    colors[imgui::StyleColor::Button as usize] = gray_10;
    colors[imgui::StyleColor::ButtonHovered as usize] = gray_20;
    colors[imgui::StyleColor::ButtonActive as usize] = gray_30;

    // Headers
    colors[imgui::StyleColor::Header as usize] = gray_10;
    colors[imgui::StyleColor::HeaderHovered as usize] = gray_20;
    colors[imgui::StyleColor::HeaderActive as usize] = gray_30;

    // Separators
    colors[imgui::StyleColor::Separator as usize] = gray_30;
    colors[imgui::StyleColor::SeparatorHovered as usize] = gray_40;
    colors[imgui::StyleColor::SeparatorActive as usize] = white;

    // Resize grips (explicit override)
    colors[imgui::StyleColor::ResizeGrip as usize] = gray_20;
    colors[imgui::StyleColor::ResizeGripHovered as usize] = gray_30;
    colors[imgui::StyleColor::ResizeGripActive as usize] = gray_40;

    // Tabs
    colors[imgui::StyleColor::Tab as usize] = gray_10;
    colors[imgui::StyleColor::TabHovered as usize] = gray_20;
    colors[imgui::StyleColor::TabActive as usize] = gray_30;
    colors[imgui::StyleColor::TabUnfocused as usize] = gray_10;
    colors[imgui::StyleColor::TabUnfocusedActive as usize] = gray_20;

    // Navigation
    colors[imgui::StyleColor::NavHighlight as usize] = gray_40; // Was blue
    colors[imgui::StyleColor::NavWindowingHighlight as usize] = white;
    colors[imgui::StyleColor::NavWindowingDimBg as usize] = [0.0, 0.0, 0.0, 0.5];

    // Disabled state
    style.disabled_alpha = 0.6;

    // Final touch: Kill all possible transparency
    style.alpha = 1.0;
    colors[imgui::StyleColor::ModalWindowDimBg as usize] = [0.0, 0.0, 0.0, 0.85];
}
