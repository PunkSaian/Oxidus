extern crate imgui;
extern crate sdl2_sys;

use imgui::{Context, DrawCmd, DrawIdx};
use std::mem;

#[derive(Debug)]
pub struct Renderer {
    pub sdl_renderer: *mut sdl2_sys::SDL_Renderer,
    font_texture: *mut sdl2_sys::SDL_Texture,
}

impl Renderer {
    pub fn new(sdl_renderer: *mut sdl2_sys::SDL_Renderer, imgui: &mut Context) -> Self {
        unsafe {
            //Create font texture
            let atlas = imgui.fonts();
            let texture = atlas.build_rgba32_texture();

            // Create SDL texture
            #[allow(clippy::cast_possible_wrap)]
            let font_texture = sdl2_sys::SDL_CreateTexture(
                sdl_renderer,
                sdl2_sys::SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
                sdl2_sys::SDL_TextureAccess::SDL_TEXTUREACCESS_STATIC as i32,
                texture.width as i32,
                texture.height as i32,
            );

            //Upload texture data
            #[allow(clippy::cast_possible_wrap)]
            sdl2_sys::SDL_UpdateTexture(
                font_texture,
                std::ptr::null(),
                texture.data.as_ptr().cast(),
                (texture.width * 4) as i32,
            );

            // Set texture parameters
            sdl2_sys::SDL_SetTextureBlendMode(
                font_texture,
                sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
            );

            // Store texture reference in imgui
            atlas.tex_id = (font_texture as usize).into();
            sdl2_sys::SDL_SetTextureScaleMode(
                font_texture,
                sdl2_sys::SDL_ScaleMode::SDL_ScaleModeLinear,
            );
            Self {
                sdl_renderer,
                font_texture,
            }
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        let draw_data = ctx.render();

        unsafe {
            sdl2_sys::SDL_SetRenderDrawBlendMode(
                self.sdl_renderer,
                sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
            );
            for draw_list in draw_data.draw_lists() {
                let vtx_buffer = draw_list.vtx_buffer();
                let idx_buffer = draw_list.idx_buffer();

                // Convert vertices to SDL format (REMOVED SCALING)
                let vertices: Vec<sdl2_sys::SDL_Vertex> = vtx_buffer
                    .iter()
                    .map(|v| sdl2_sys::SDL_Vertex {
                        position: sdl2_sys::SDL_FPoint {
                            x: v.pos[0], // Removed scaling
                            y: v.pos[1], // Removed scaling
                        },
                        color: sdl2_sys::SDL_Color {
                            r: v.col[0],
                            g: v.col[1],
                            b: v.col[2],
                            a: v.col[3],
                        },
                        tex_coord: sdl2_sys::SDL_FPoint {
                            x: v.uv[0],
                            y: v.uv[1],
                        },
                    })
                    .collect();

                // Convert indices to u32 (unchanged)
                let indices: Vec<u32> = idx_buffer.iter().map(|&i| u32::from(i)).collect();

                for cmd in draw_list.commands() {
                    if let DrawCmd::Elements { count, cmd_params } = cmd {
                        let texture = cmd_params.texture_id.id() as *mut sdl2_sys::SDL_Texture;

                        sdl2_sys::SDL_SetTextureBlendMode(
                            texture,
                            sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
                        );

                        // Set clip rect (REMOVED SCALING)
                        let clip_rect = sdl2_sys::SDL_Rect {
                            x: cmd_params.clip_rect[0].round() as i32,
                            y: cmd_params.clip_rect[1].round() as i32,
                            w: (cmd_params.clip_rect[2] - cmd_params.clip_rect[0]).round() as i32,
                            h: (cmd_params.clip_rect[3] - cmd_params.clip_rect[1]).round() as i32,
                        };
                        sdl2_sys::SDL_RenderSetClipRect(self.sdl_renderer, &clip_rect);

                        // Calculate element offset (unchanged)
                        let element_offset = cmd_params.idx_offset / mem::size_of::<DrawIdx>();
                        let cmd_indices = &indices[element_offset..element_offset + count];

                        // Draw command (unchanged)
                        #[allow(clippy::cast_possible_wrap)]
                        sdl2_sys::SDL_RenderGeometry(
                            self.sdl_renderer,
                            texture,
                            vertices.as_ptr(),
                            vertices.len() as i32,
                            cmd_indices.as_ptr().cast(),
                            cmd_indices.len() as i32,
                        );
                    }
                }
            }
            sdl2_sys::SDL_RenderPresent(self.sdl_renderer);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            dbg!("dropping renderer");
            sdl2_sys::SDL_DestroyTexture(self.font_texture);
        }
    }
}
