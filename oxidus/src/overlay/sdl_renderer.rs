extern crate imgui;
extern crate sdl2_sys;

use image::ImageReader;
use imgui::{Context, DrawCmd};

use std::io::Cursor;

use crate::util::consts::{OXIDUS_LOGO_PNG, OXIDUS_LOGO_TRANS_OUTLINED_PNG, OXIDUS_LOGO_TRANS_PNG};

use super::OxidusResult;

pub struct Texture {
    pub id: imgui::TextureId,
    pub dimensions: (u32, u32),
}

pub struct Textures {
    pub logo: Texture,
    pub logo_trans: Texture,
    pub logo_trans_outlined: Texture,
}

impl Textures {
    pub fn new(renderer: &mut SdlRenderer) -> OxidusResult<Self> {
        let logo = renderer.create_texture_from_bytes(OXIDUS_LOGO_PNG)?;
        let logo_trans = renderer.create_texture_from_bytes(OXIDUS_LOGO_TRANS_PNG)?;
        let logo_trans_outlined =
            renderer.create_texture_from_bytes(OXIDUS_LOGO_TRANS_OUTLINED_PNG)?;

        Ok(Self {
            logo,
            logo_trans,
            logo_trans_outlined,
        })
    }
}

#[derive(Debug)]
#[allow(clippy::struct_field_names)]
pub struct SdlRenderer {
    pub sdl_renderer: *mut sdl2_sys::SDL_Renderer,
    managed_textures: Vec<*mut sdl2_sys::SDL_Texture>,
}

impl SdlRenderer {
    pub fn new(sdl_renderer: *mut sdl2_sys::SDL_Renderer, imgui: &mut Context) -> Self {
        unsafe {
            let atlas = imgui.fonts();
            let texture = atlas.build_rgba32_texture();

            #[allow(clippy::cast_possible_wrap)]
            let font_texture = sdl2_sys::SDL_CreateTexture(
                sdl_renderer,
                sdl2_sys::SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
                sdl2_sys::SDL_TextureAccess::SDL_TEXTUREACCESS_STATIC as i32,
                texture.width as i32,
                texture.height as i32,
            );

            #[allow(clippy::cast_possible_wrap)]
            sdl2_sys::SDL_UpdateTexture(
                font_texture,
                std::ptr::null(),
                texture.data.as_ptr().cast(),
                (texture.width * 4) as i32,
            );

            sdl2_sys::SDL_SetTextureBlendMode(
                font_texture,
                sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
            );

            atlas.tex_id = (font_texture as usize).into();
            sdl2_sys::SDL_SetTextureScaleMode(
                font_texture,
                sdl2_sys::SDL_ScaleMode::SDL_ScaleModeNearest,
            );

            Self {
                sdl_renderer,
                managed_textures: vec![font_texture],
            }
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        let draw_data = ctx.render();
        if draw_data.draw_lists_count() == 0 {
            return;
        }

        unsafe {
            sdl2_sys::SDL_SetRenderDrawBlendMode(
                self.sdl_renderer,
                sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
            );
            for draw_list in draw_data.draw_lists() {
                let vtx_buffer = draw_list.vtx_buffer();
                let idx_buffer = draw_list.idx_buffer();

                let vertices: Vec<sdl2_sys::SDL_Vertex> = vtx_buffer
                    .iter()
                    .map(|v| sdl2_sys::SDL_Vertex {
                        position: sdl2_sys::SDL_FPoint {
                            x: v.pos[0],
                            y: v.pos[1],
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

                let indices: Vec<u32> = idx_buffer.iter().map(|&i| u32::from(i)).collect();

                for cmd in draw_list.commands() {
                    if let DrawCmd::Elements { count, cmd_params } = cmd {
                        let texture = cmd_params.texture_id.id() as *mut sdl2_sys::SDL_Texture;

                        sdl2_sys::SDL_SetTextureBlendMode(
                            texture,
                            sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
                        );

                        let clip_rect = sdl2_sys::SDL_Rect {
                            x: cmd_params.clip_rect[0] as i32,
                            y: cmd_params.clip_rect[1].round() as i32,
                            w: (cmd_params.clip_rect[2] - cmd_params.clip_rect[0]).round() as i32,
                            h: (cmd_params.clip_rect[3] - cmd_params.clip_rect[1]).round() as i32,
                        };
                        sdl2_sys::SDL_RenderSetClipRect(self.sdl_renderer, &clip_rect);

                        let element_offset = cmd_params.idx_offset;
                        let cmd_indices = &indices[element_offset..element_offset + count];

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

    pub fn create_texture_from_bytes(&mut self, bytes: &[u8]) -> OxidusResult<Texture> {
        unsafe {
            let img = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()?
                .decode()?
                .to_rgba8();

            let (width, height) = img.dimensions();
            let pixels = img.into_raw();

            let texture = sdl2_sys::SDL_CreateTexture(
                self.sdl_renderer,
                sdl2_sys::SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
                sdl2_sys::SDL_TextureAccess::SDL_TEXTUREACCESS_STATIC as i32,
                width as i32,
                height as i32,
            );

            sdl2_sys::SDL_UpdateTexture(
                texture,
                std::ptr::null(),
                pixels.as_ptr().cast(),
                (width * 4) as i32,
            );

            self.managed_textures.push(texture);

            sdl2_sys::SDL_SetTextureBlendMode(
                texture,
                sdl2_sys::SDL_BlendMode::SDL_BLENDMODE_BLEND,
            );

            Ok(Texture {
                id: imgui::TextureId::from(texture),
                dimensions: (width, height),
            })
        }
    }
}

impl Drop for SdlRenderer {
    fn drop(&mut self) {
        for &texture in &self.managed_textures {
            unsafe {
                sdl2_sys::SDL_DestroyTexture(texture);
            }
        }
    }
}
