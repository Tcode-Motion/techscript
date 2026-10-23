// =============================================================================
// TechScript AppForge — EGL/OpenGL ES 2.0 Native Renderer
// runtime/android_runtime/src/renderer.rs
//
// Renders the TechScript AppForge Liquid Glass UI and real 3D interactive
// scenes natively using OpenGL ES 2.0.
//
// Architecture:
//   - Hardware-accelerated EGL 1.4 context on ANativeWindow
//   - Pass 1: 3D Procedural Mesh with Blinn-Phong lighting & glass Fresnel
//   - Pass 2: 2D Liquid Glass UI (SDF rounded panels, specular highlights, text quads)
//   - Persistent bottom liquid-glass navigation dock with 5 primary destinations:
//     Home, Explorer, Labs, Projects, Docs
//   - Zero Java/Kotlin code, zero WebView
// =============================================================================

use android_activity::AndroidApp;
use crate::theme::Theme;
use crate::txs_runtime::TechScriptRuntime;
use crate::font8x8;
use crate::geometry3d::{self, MeshType, MaterialPreset};
use crate::glass_ui::{GlassBatch, UI_VS_SOURCE, UI_FS_SOURCE, MESH3D_VS_SOURCE, MESH3D_FS_SOURCE};
use crate::typography::{self, FontAtlas};

// ─────────────────────────────────────────────────────────────────────────────
// Interactive Hit-Test Regions
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Action {
    Navigate(String),
    ToggleTheme,
    Cycle3DMesh,
    Cycle3DMaterial,
    ToggleAutoRotate,
    Reset3DCamera,
    AdjustZoom(f32),
    ToggleSpeed,
    SelectExplorerCategory(usize),
    IncrementCounter,
    ToggleSwitch,
    ToggleExplorerCode,
    SelectProject(usize),
    ToggleCodeView,
    ValidateProject,
    SelectDocTopic(usize),
    OpenAbout,
    OpenLink(String),
}

#[derive(Debug, Clone)]
pub struct HitRegion {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub action: Action,
}

impl HitRegion {
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EGL Raw Types & Functions
// ─────────────────────────────────────────────────────────────────────────────

mod egl {
    pub type Display = *mut std::ffi::c_void;
    pub type Surface = *mut std::ffi::c_void;
    pub type Context = *mut std::ffi::c_void;
    pub type Config  = *mut std::ffi::c_void;

    pub const NO_DISPLAY: Display = std::ptr::null_mut();
    pub const NO_SURFACE: Surface = std::ptr::null_mut();
    pub const NO_CONTEXT: Context = std::ptr::null_mut();

    pub const EGL_DEFAULT_DISPLAY: isize   = 0;
    pub const EGL_OPENGL_ES2_BIT: i32      = 0x0004;
    pub const EGL_SURFACE_TYPE: i32        = 0x3033;
    pub const EGL_WINDOW_BIT: i32          = 0x0004;
    pub const EGL_RENDERABLE_TYPE: i32     = 0x3040;
    pub const EGL_RED_SIZE: i32            = 0x3024;
    pub const EGL_GREEN_SIZE: i32          = 0x3023;
    pub const EGL_BLUE_SIZE: i32           = 0x3022;
    pub const EGL_ALPHA_SIZE: i32          = 0x3021;
    pub const EGL_DEPTH_SIZE: i32          = 0x3025;
    pub const EGL_NONE: i32                = 0x3038;
    pub const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;
    pub const EGL_WIDTH: i32               = 0x3057;
    pub const EGL_HEIGHT: i32              = 0x3056;

    extern "C" {
        pub fn eglGetDisplay(display_id: isize) -> Display;
        pub fn eglInitialize(display: Display, major: *mut i32, minor: *mut i32) -> u32;
        pub fn eglChooseConfig(display: Display, attrib_list: *const i32, configs: *mut Config, config_size: i32, num_config: *mut i32) -> u32;
        pub fn eglCreateWindowSurface(display: Display, config: Config, window: *mut std::ffi::c_void, attrib_list: *const i32) -> Surface;
        pub fn eglCreateContext(display: Display, config: Config, share_context: Context, attrib_list: *const i32) -> Context;
        pub fn eglMakeCurrent(display: Display, draw: Surface, read: Surface, context: Context) -> u32;
        pub fn eglSwapBuffers(display: Display, surface: Surface) -> u32;
        pub fn eglDestroyContext(display: Display, context: Context) -> u32;
        pub fn eglDestroySurface(display: Display, surface: Surface) -> u32;
        pub fn eglTerminate(display: Display) -> u32;
        pub fn eglGetError() -> i32;
        pub fn eglQuerySurface(display: Display, surface: Surface, attribute: i32, value: *mut i32) -> u32;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OpenGL ES 2.0 Types & Functions
// ─────────────────────────────────────────────────────────────────────────────

const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
const GL_TRIANGLES: u32        = 0x0004;
const GL_UNSIGNED_SHORT: u32   = 0x1403;
const GL_FLOAT: u32            = 0x1406;
const GL_ARRAY_BUFFER: u32     = 0x8892;
const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
const GL_STATIC_DRAW: u32      = 0x88E4;
const GL_DYNAMIC_DRAW: u32     = 0x88E8;
const GL_FRAGMENT_SHADER: u32  = 0x8B30;
const GL_VERTEX_SHADER: u32    = 0x8B31;
const GL_COMPILE_STATUS: u32   = 0x8B81;
const GL_LINK_STATUS: u32      = 0x8B82;
const GL_BLEND: u32            = 0x0BE2;
const GL_SRC_ALPHA: u32        = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
const GL_DEPTH_TEST: u32       = 0x0B71;
const GL_LEQUAL: u32           = 0x0203;
const GL_TEXTURE_2D: u32       = 0x0DE1;
const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
const GL_NEAREST: i32          = 0x2600;
const GL_LINEAR: i32           = 0x2601;
const GL_RGBA: u32             = 0x1908;
const GL_UNSIGNED_BYTE: u32    = 0x1401;
const GL_UNPACK_ALIGNMENT: u32 = 0x0CF5;
const GL_SCISSOR_TEST: u32     = 0x0C11;

extern "C" {
    fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    fn glClear(mask: u32);
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glScissor(x: i32, y: i32, width: i32, height: i32);
    fn glEnable(cap: u32);
    fn glDisable(cap: u32);
    fn glBlendFunc(sfactor: u32, dfactor: u32);
    fn glDepthFunc(func: u32);
    fn glCreateShader(shader_type: u32) -> u32;
    fn glShaderSource(shader: u32, count: i32, string: *const *const u8, length: *const i32);
    fn glCompileShader(shader: u32);
    fn glGetShaderiv(shader: u32, pname: u32, params: *mut i32);
    fn glGetShaderInfoLog(shader: u32, max_length: i32, length: *mut i32, info_log: *mut u8);
    fn glCreateProgram() -> u32;
    fn glAttachShader(program: u32, shader: u32);
    fn glLinkProgram(program: u32);
    fn glGetProgramiv(program: u32, pname: u32, params: *mut i32);
    fn glGetProgramInfoLog(program: u32, max_length: i32, length: *mut i32, info_log: *mut u8);
    fn glUseProgram(program: u32);
    fn glDeleteShader(shader: u32);
    fn glGenBuffers(n: i32, buffers: *mut u32);
    fn glBindBuffer(target: u32, buffer: u32);
    fn glBufferData(target: u32, size: isize, data: *const std::ffi::c_void, usage: u32);
    fn glGenTextures(n: i32, textures: *mut u32);
    fn glBindTexture(target: u32, texture: u32);
    fn glTexParameteri(target: u32, pname: u32, param: i32);
    fn glTexImage2D(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, image_type: u32, pixels: *const std::ffi::c_void);
    fn glPixelStorei(pname: u32, param: i32);
    fn glGetAttribLocation(program: u32, name: *const u8) -> i32;
    fn glGetUniformLocation(program: u32, name: *const u8) -> i32;
    fn glEnableVertexAttribArray(index: u32);
    fn glVertexAttribPointer(index: u32, size: i32, pointer_type: u32, normalized: u8, stride: i32, pointer: *const std::ffi::c_void);
    fn glUniform2f(location: i32, v0: f32, v1: f32);
    fn glUniform3f(location: i32, v0: f32, v1: f32, v2: f32);
    fn glUniform1i(location: i32, v0: i32);
    fn glUniformMatrix4fv(location: i32, count: i32, transpose: u8, value: *const f32);
    fn glDrawElements(mode: u32, count: i32, element_type: u32, indices: *const std::ffi::c_void);
}

// ─────────────────────────────────────────────────────────────────────────────
// Native Renderer Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub struct NativeRenderer {
    display: egl::Display,
    surface: egl::Surface,
    context: egl::Context,
    pub width: i32,
    pub height: i32,

    // 2D Liquid Glass UI Shader & Buffers
    ui_program: u32,
    ui_vbo: u32,
    ui_ibo: u32,
    ui_batch: GlassBatch,
    font_texture: u32,
    pub font_atlas: FontAtlas,

    // 3D Procedural Mesh Shader & Buffers
    mesh3d_program: u32,
    mesh3d_vbo: u32,
    mesh3d_ibo: u32,
    mesh3d_index_count: i32,
    current_mesh: MeshType,
    current_material: MaterialPreset,

    // Interactive Hit Regions
    pub hit_regions: Vec<HitRegion>,

    // Animation Clock
    anim_time: f32,
}

impl NativeRenderer {
    pub fn new(app: &AndroidApp, _theme: &Theme) -> Result<Self, String> {
        let window = app.native_window()
            .ok_or_else(|| "No native window available".to_string())?;
        let window_ptr = window.ptr().as_ptr() as *mut std::ffi::c_void;

        unsafe {
            // 1. EGL Initialization
            let display = egl::eglGetDisplay(egl::EGL_DEFAULT_DISPLAY);
            if display == egl::NO_DISPLAY {
                return Err(format!("eglGetDisplay failed: {}", egl::eglGetError()));
            }

            let mut major = 0i32;
            let mut minor = 0i32;
            if egl::eglInitialize(display, &mut major, &mut minor) == 0 {
                return Err(format!("eglInitialize failed: {}", egl::eglGetError()));
            }
            log::info!("[TechScript EGL] Initialized EGL {major}.{minor}");

            let attribs = [
                egl::EGL_SURFACE_TYPE,    egl::EGL_WINDOW_BIT,
                egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_ES2_BIT,
                egl::EGL_RED_SIZE,        8,
                egl::EGL_GREEN_SIZE,      8,
                egl::EGL_BLUE_SIZE,       8,
                egl::EGL_ALPHA_SIZE,      8,
                egl::EGL_DEPTH_SIZE,      16,
                egl::EGL_NONE,
            ];
            let mut config: egl::Config = std::ptr::null_mut();
            let mut num_configs = 0i32;
            if egl::eglChooseConfig(display, attribs.as_ptr(), &mut config, 1, &mut num_configs) == 0 || num_configs == 0 {
                return Err(format!("eglChooseConfig failed: {}", egl::eglGetError()));
            }

            let surface = egl::eglCreateWindowSurface(display, config, window_ptr, std::ptr::null());
            if surface == egl::NO_SURFACE {
                return Err(format!("eglCreateWindowSurface failed: {}", egl::eglGetError()));
            }

            let ctx_attribs = [egl::EGL_CONTEXT_CLIENT_VERSION, 2, egl::EGL_NONE];
            let context = egl::eglCreateContext(display, config, egl::NO_CONTEXT, ctx_attribs.as_ptr());
            if context == egl::NO_CONTEXT {
                return Err(format!("eglCreateContext failed: {}", egl::eglGetError()));
            }

            if egl::eglMakeCurrent(display, surface, surface, context) == 0 {
                return Err(format!("eglMakeCurrent failed: {}", egl::eglGetError()));
            }

            let mut width = 0i32;
            let mut height = 0i32;
            egl::eglQuerySurface(display, surface, egl::EGL_WIDTH,  &mut width);
            egl::eglQuerySurface(display, surface, egl::EGL_HEIGHT, &mut height);
            log::info!("[TechScript EGL] Viewport resolution: {width}x{height}");

            // 2. Compile 2D Liquid Glass UI Shader Program
            let ui_program = create_program(UI_VS_SOURCE, UI_FS_SOURCE)?;

            // 3. Compile 3D Mesh Shader Program
            let mesh3d_program = create_program(MESH3D_VS_SOURCE, MESH3D_FS_SOURCE)?;

            // 4. Generate & Upload 32-bit RGBA TrueType Font Atlas Texture (512x512)
            let mut font_texture = 0u32;
            glGenTextures(1, &mut font_texture);
            glBindTexture(GL_TEXTURE_2D, font_texture);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            let font_atlas = FontAtlas::create_system_atlas();
            glTexImage2D(
                GL_TEXTURE_2D, 0, GL_RGBA as i32,
                typography::ATLAS_WIDTH as i32, typography::ATLAS_HEIGHT as i32, 0,
                GL_RGBA, GL_UNSIGNED_BYTE,
                font_atlas.atlas_rgba.as_ptr() as *const std::ffi::c_void,
            );
            log::info!("[TechScript Font] Uploaded TrueType font atlas (512x512)");

            // 5. Create VBO/IBO Buffers
            let mut ui_buffers = [0u32; 2];
            glGenBuffers(2, ui_buffers.as_mut_ptr());

            let mut mesh_buffers = [0u32; 2];
            glGenBuffers(2, mesh_buffers.as_mut_ptr());

            // 6. Upload Initial 3D Geometry
            let initial_mesh = MeshType::Crystal;
            let initial_mat = MaterialPreset::TechScriptIndigo;
            let (v3d, i3d) = geometry3d::generate_mesh(initial_mesh, initial_mat);
            let mesh3d_index_count = i3d.len() as i32;

            glBindBuffer(GL_ARRAY_BUFFER, mesh_buffers[0]);
            glBufferData(
                GL_ARRAY_BUFFER,
                (v3d.len() * std::mem::size_of::<geometry3d::Vertex3D>()) as isize,
                v3d.as_ptr() as *const std::ffi::c_void,
                GL_STATIC_DRAW,
            );

            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, mesh_buffers[1]);
            glBufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (i3d.len() * std::mem::size_of::<u16>()) as isize,
                i3d.as_ptr() as *const std::ffi::c_void,
                GL_STATIC_DRAW,
            );

            Ok(Self {
                display,
                surface,
                context,
                width,
                height,
                ui_program,
                ui_vbo: ui_buffers[0],
                ui_ibo: ui_buffers[1],
                ui_batch: GlassBatch::new(),
                font_texture,
                font_atlas,
                mesh3d_program,
                mesh3d_vbo: mesh_buffers[0],
                mesh3d_ibo: mesh_buffers[1],
                mesh3d_index_count,
                current_mesh: initial_mesh,
                current_material: initial_mat,
                hit_regions: Vec::new(),
                anim_time: 0.0,
            })
        }
    }

    /// Render modern high-DPI text with proportional font metrics
    pub fn draw_text(&mut self, x: f32, y: f32, text: &str, font_size_px: f32, color: [f32; 4]) -> f32 {
        self.ui_batch.push_text_ttf(&self.font_atlas, x, y, text, font_size_px, color)
    }

    /// Measures exact visual pixel width of a string at the specified font size
    pub fn measure_text(&self, text: &str, font_size_px: f32) -> f32 {
        self.font_atlas.measure_text(text, font_size_px)
    }

    /// Render word-wrapped multiline text ensuring it never overflows max_w
    pub fn draw_text_wrapped(&mut self, x: f32, y: f32, max_w: f32, text: &str, font_size_px: f32, color: [f32; 4], line_gap: f32) -> f32 {
        let lines = self.font_atlas.wrap_text(text, max_w, font_size_px);
        let line_h = font_size_px + line_gap;
        for (i, line) in lines.iter().enumerate() {
            self.draw_text(x, y + (i as f32) * line_h, line, font_size_px, color);
        }
        lines.len() as f32 * line_h
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        unsafe {
            glViewport(0, 0, width, height);
        }
        log::info!("[TechScript EGL] Surface resized to {width}x{height}");
    }

    /// Renders a single frame:
    ///   - Checks for mesh/material updates and re-uploads geometry
    ///   - Pass 1: 3D interactive procedural mesh (on Home and Labs screens)
    ///   - Pass 2: 2D Liquid Glass UI with persistent bottom dock
    ///   - Swaps hardware EGL buffers
    pub fn render_frame(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme) -> Result<(), String> {
        self.anim_time += 0.016;

        // Auto-rotation physics
        if runtime.auto_rotate {
            runtime.rot_y += 0.015 * runtime.anim_speed;
        }

        unsafe {
            // Dynamic 3D Mesh / Material update
            if runtime.mesh_type != self.current_mesh || runtime.material != self.current_material {
                let (v3d, i3d) = geometry3d::generate_mesh(runtime.mesh_type, runtime.material);
                self.mesh3d_index_count = i3d.len() as i32;

                glBindBuffer(GL_ARRAY_BUFFER, self.mesh3d_vbo);
                glBufferData(
                    GL_ARRAY_BUFFER,
                    (v3d.len() * std::mem::size_of::<geometry3d::Vertex3D>()) as isize,
                    v3d.as_ptr() as *const std::ffi::c_void,
                    GL_STATIC_DRAW,
                );

                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.mesh3d_ibo);
                glBufferData(
                    GL_ELEMENT_ARRAY_BUFFER,
                    (i3d.len() * std::mem::size_of::<u16>()) as isize,
                    i3d.as_ptr() as *const std::ffi::c_void,
                    GL_STATIC_DRAW,
                );

                self.current_mesh = runtime.mesh_type;
                self.current_material = runtime.material;
            }

            // Clear Screen
            let [bg_r, bg_g, bg_b, bg_a] = Theme::to_f32_color(theme.bg);
            glClearColor(bg_r, bg_g, bg_b, bg_a);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            // Responsive scale factor (relative to 390dp reference)
            let scale = (self.width as f32 / 390.0).clamp(1.0, 3.5);
            let pad = 16.0 * scale;
            let top_inset = 40.0 * scale;

            // ─────────────────────────────────────────────────────────────────
            // Pass 1: 2D Liquid Glass UI
            // ─────────────────────────────────────────────────────────────────
            self.build_ui_batch(runtime, theme, scale);
            self.draw_ui_batch();

            // ─────────────────────────────────────────────────────────────────
            // Pass 2: 3D Procedural Mesh (Rendered within scissored 3D card viewport)
            // ─────────────────────────────────────────────────────────────────
            let screen = runtime.active_screen();
            if screen == "home" {
                let hero_y = top_inset + 42.0 * scale + 8.0 * scale + 52.0 * scale + 10.0 * scale;
                let vp_y = hero_y + 40.0 * scale;
                let vp_h = 138.0 * scale;
                let vp_w = (self.width as f32 - pad * 2.0).min(480.0 * scale);
                let vp_x = (self.width as f32 - vp_w) * 0.5;
                self.render_3d_viewport(runtime, vp_x, vp_y, vp_w, vp_h);
            } else if screen == "labs" {
                let content_y = top_inset + 42.0 * scale + 8.0 * scale;
                let vp_y = content_y + 42.0 * scale + 40.0 * scale;
                let vp_h = 160.0 * scale;
                let vp_w = (self.width as f32 - pad * 2.0).min(480.0 * scale);
                let vp_x = (self.width as f32 - vp_w) * 0.5;
                self.render_3d_viewport(runtime, vp_x, vp_y, vp_w, vp_h);
            }

            // Swap buffers to display frame
            if egl::eglSwapBuffers(self.display, self.surface) == 0 {
                return Err(format!("eglSwapBuffers failed: {}", egl::eglGetError()));
            }
        }

        runtime.frame_count += 1;
        Ok(())
    }

    /// Render 3D mesh in a specified viewport rectangle
    unsafe fn render_3d_viewport(&self, runtime: &TechScriptRuntime, vp_x: f32, vp_y: f32, vp_w: f32, vp_h: f32) {
        let sh = self.height as f32;

        // OpenGL viewport Y starts from bottom of screen
        let gl_y = (sh - vp_y - vp_h) as i32;
        glViewport(vp_x as i32, gl_y, vp_w as i32, vp_h as i32);
        glEnable(GL_SCISSOR_TEST);
        glScissor(vp_x as i32, gl_y, vp_w as i32, vp_h as i32);

        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LEQUAL);
        glClear(GL_DEPTH_BUFFER_BIT);

        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        glUseProgram(self.mesh3d_program);

        // Compute Camera MVP matrix with zoom
        let aspect = vp_w / vp_h;
        let proj = geometry3d::mat4_perspective(45.0f32.to_radians(), aspect, 0.1, 100.0);
        let cam_dist = -3.2 / runtime.zoom;
        let view = geometry3d::mat4_translate(0.0, 0.0, cam_dist);

        let rot_x = geometry3d::mat4_rotate_x(runtime.rot_x);
        let rot_y = geometry3d::mat4_rotate_y(runtime.rot_y);
        let model = geometry3d::mat4_multiply(&rot_x, &rot_y);

        let mv = geometry3d::mat4_multiply(&view, &model);
        let mvp = geometry3d::mat4_multiply(&proj, &mv);

        let u_mvp = glGetUniformLocation(self.mesh3d_program, b"u_mvp\0".as_ptr());
        let u_model = glGetUniformLocation(self.mesh3d_program, b"u_model\0".as_ptr());
        let u_light = glGetUniformLocation(self.mesh3d_program, b"u_light_dir\0".as_ptr());
        let u_spec = glGetUniformLocation(self.mesh3d_program, b"u_specular_col\0".as_ptr());
        let u_view = glGetUniformLocation(self.mesh3d_program, b"u_view_pos\0".as_ptr());

        glUniformMatrix4fv(u_mvp, 1, 0, mvp.as_ptr());
        glUniformMatrix4fv(u_model, 1, 0, model.as_ptr());
        glUniform3f(u_light, 0.6, 1.0, 0.8);

        let spec_col = runtime.material.specular_color();
        glUniform3f(u_spec, spec_col[0], spec_col[1], spec_col[2]);
        glUniform3f(u_view, 0.0, 0.0, -cam_dist);

        // Bind 3D mesh buffers
        glBindBuffer(GL_ARRAY_BUFFER, self.mesh3d_vbo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.mesh3d_ibo);

        let stride = std::mem::size_of::<geometry3d::Vertex3D>() as i32;
        let a_pos = glGetAttribLocation(self.mesh3d_program, b"a_pos\0".as_ptr()) as u32;
        let a_normal = glGetAttribLocation(self.mesh3d_program, b"a_normal\0".as_ptr()) as u32;
        let a_color = glGetAttribLocation(self.mesh3d_program, b"a_color\0".as_ptr()) as u32;

        glEnableVertexAttribArray(a_pos);
        glVertexAttribPointer(a_pos, 3, GL_FLOAT, 0, stride, std::ptr::null());

        glEnableVertexAttribArray(a_normal);
        glVertexAttribPointer(a_normal, 3, GL_FLOAT, 0, stride, 12 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_color);
        glVertexAttribPointer(a_color, 4, GL_FLOAT, 0, stride, 24 as *const std::ffi::c_void);

        glDrawElements(GL_TRIANGLES, self.mesh3d_index_count, GL_UNSIGNED_SHORT, std::ptr::null());

        glDisable(GL_SCISSOR_TEST);
        glDisable(GL_DEPTH_TEST);
        glViewport(0, 0, self.width, self.height);
    }

    /// Build the 2D UI batch for the current screen
    fn build_ui_batch(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32) {
        self.ui_batch.clear();
        self.hit_regions.clear();

        let sw = self.width as f32;
        let sh = self.height as f32;
        let pad = 16.0 * scale;
        let content_w = sw - pad * 2.0;
        let top_inset = 40.0 * scale;

        // ── Atmospheric Spatial Lighting Glows (Subtle, Restrained) ────────
        self.ui_batch.push_rect(-20.0 * scale, -20.0 * scale, sw * 0.65, 200.0 * scale, 100.0 * scale, [0.388, 0.400, 0.945, 0.04], 0.0);
        self.ui_batch.push_rect(sw * 0.45, -10.0 * scale, sw * 0.6, 220.0 * scale, 110.0 * scale, [0.545, 0.361, 0.965, 0.035], 0.0);
        self.ui_batch.push_rect(sw * 0.25, 200.0 * scale, sw * 0.5, 140.0 * scale, 70.0 * scale, [0.133, 0.827, 0.933, 0.025], 0.0);
        self.ui_batch.push_rect(sw * 0.1, sh - 120.0 * scale, sw * 0.8, 80.0 * scale, 40.0 * scale, [0.851, 0.275, 0.937, 0.03], 0.0);

        // ── 1. Top Persistent Brand Header ───────────────────────────────────
        let header_y = top_inset;
        let header_h = 38.0 * scale;

        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);
        let col_surface = Theme::to_f32_color(theme.surface);

        // Official TechScript Brand Logo Mark
        self.ui_batch.draw_techscript_logo_mark(pad + 12.0 * scale, header_y + 16.0 * scale, 24.0 * scale);

        // Brand Title with Proportional TrueType Typography
        self.draw_text(pad + 32.0 * scale, header_y + 4.0 * scale, "TechScript", 16.0 * scale, col_text);
        self.draw_text(pad + 32.0 * scale, header_y + 20.0 * scale, "AppForge v2.0", 10.5 * scale, col_accent);

        // Header Action: Theme toggle pill [DARK / LIGHT]
        let badge_h = 24.0 * scale;
        let theme_btn_w = 56.0 * scale;
        let theme_btn_x = sw - pad - theme_btn_w;
        let theme_label = match runtime.theme_mode() {
            "dark"  => "DARK",
            "light" => "LIGHT",
            _       => "AUTO",
        };
        self.ui_batch.push_rect(theme_btn_x, header_y + 4.0 * scale, theme_btn_w, badge_h, 12.0 * scale, col_surface, 1.0);
        let t_w = self.measure_text(theme_label, 10.5 * scale);
        self.draw_text(theme_btn_x + (theme_btn_w - t_w) * 0.5, header_y + 10.0 * scale, theme_label, 10.5 * scale, col_text);
        self.hit_regions.push(HitRegion {
            x: theme_btn_x,
            y: header_y + 4.0 * scale,
            w: theme_btn_w,
            h: badge_h,
            action: Action::ToggleTheme,
        });

        // Header Action: Diagnostics Info [?]
        let info_btn_w = 26.0 * scale;
        let info_btn_x = theme_btn_x - 8.0 * scale - info_btn_w;
        self.ui_batch.push_rect(info_btn_x, header_y + 4.0 * scale, info_btn_w, badge_h, 12.0 * scale, col_surface, 1.0);
        let q_w = self.measure_text("?", 11.5 * scale);
        self.draw_text(info_btn_x + (info_btn_w - q_w) * 0.5, header_y + 10.0 * scale, "?", 11.5 * scale, col_accent);
        self.hit_regions.push(HitRegion {
            x: info_btn_x,
            y: header_y + 4.0 * scale,
            w: info_btn_w,
            h: badge_h,
            action: Action::OpenAbout,
        });

        // ── 2. Screen Content Dispatch ───────────────────────────────────────
        let content_y = header_y + header_h + 8.0 * scale;
        match runtime.active_screen() {
            "home"     => self.render_home_content(runtime, theme, scale, pad, content_w, content_y),
            "explorer" => self.render_explorer_content(runtime, theme, scale, pad, content_w, content_y),
            "labs"     => self.render_labs_content(runtime, theme, scale, pad, content_w, content_y),
            "projects" => self.render_projects_content(runtime, theme, scale, pad, content_w, content_y),
            "docs"     => self.render_docs_content(runtime, theme, scale, pad, content_w, content_y),
            "about"    => self.render_about_content(runtime, theme, scale, pad, content_w, content_y),
            _ => {}
        }

        // ── 3. Persistent Bottom Navigation Dock (5 Destinations) ────────────
        self.render_navigation_dock(runtime, theme, scale);
    }

    /// Destination 1: Home Screen
    fn render_home_content(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32, pad: f32, cw: f32, start_y: f32) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        // 1. Welcome Landing Banner
        let banner_h = 44.0 * scale;
        self.ui_batch.push_rect(pad, start_y, cw, banner_h, 12.0 * scale, [0.39, 0.40, 0.95, 0.20], 1.0);
        self.draw_text(pad + 12.0 * scale, start_y + 8.0 * scale, "Next-Gen Native Application Engine", 13.5 * scale, col_text);
        self.draw_text(pad + 12.0 * scale, start_y + 25.0 * scale, "0% Java • Direct NativeActivity • Hardware EGL/GLES", 10.5 * scale, col_accent);

        // 2. 3D Interactive Hero Object Viewport Card
        let hero_y = start_y + banner_h + 8.0 * scale;
        let hero_h = 240.0 * scale;
        self.ui_batch.push_rect(pad, hero_y, cw, hero_h, 16.0 * scale, col_card, 1.2);

        // Hero 3D Card Header Badges
        self.draw_text(pad + 14.0 * scale, hero_y + 12.0 * scale, "3D Hero Object", 13.5 * scale, col_text);
        let mat_badge = format!("[ {} ]", runtime.material.name());
        let mat_w = self.measure_text(&mat_badge, 11.0 * scale);
        self.draw_text(pad + cw - mat_w - 14.0 * scale, hero_y + 12.0 * scale, &mat_badge, 11.0 * scale, col_accent);

        let mesh_label = format!("Procedural: {}", runtime.mesh_type.name());
        self.draw_text(pad + 14.0 * scale, hero_y + 28.0 * scale, &mesh_label, 10.5 * scale, col_muted);

        // Hero 3D Card Footer Hints
        let hint_y = hero_y + hero_h - 22.0 * scale;
        self.draw_text(pad + 14.0 * scale, hint_y, "Touch drag to rotate • Tap crystal for material", 10.5 * scale, col_muted);

        // Register Hero Touch Region
        self.hit_regions.push(HitRegion {
            x: pad,
            y: hero_y,
            w: cw,
            h: hero_h,
            action: Action::Cycle3DMaterial,
        });

        // 3. Navigation Shortcuts (4 Direct Destinations)
        let grid_y = hero_y + hero_h + 8.0 * scale;
        let cell_w = (cw - 8.0 * scale) * 0.5;
        let cell_h = 58.0 * scale;

        // Cell 1: Explorer
        self.ui_batch.push_rect(pad, grid_y, cell_w, cell_h, 12.0 * scale, col_card, 1.0);
        self.draw_text(pad + 12.0 * scale, grid_y + 12.0 * scale, "Explorer Gallery", 12.5 * scale, col_primary);
        self.draw_text(pad + 12.0 * scale, grid_y + 32.0 * scale, "Native components ->", 10.0 * scale, col_muted);
        self.hit_regions.push(HitRegion {
            x: pad,
            y: grid_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("explorer".into()),
        });

        // Cell 2: Labs
        let cell2_x = pad + cell_w + 8.0 * scale;
        self.ui_batch.push_rect(cell2_x, grid_y, cell_w, cell_h, 12.0 * scale, col_card, 1.0);
        self.draw_text(cell2_x + 12.0 * scale, grid_y + 12.0 * scale, "3D & Motion Lab", 12.5 * scale, col_accent);
        self.draw_text(cell2_x + 12.0 * scale, grid_y + 32.0 * scale, "Procedural mesh ->", 10.0 * scale, col_muted);
        self.hit_regions.push(HitRegion {
            x: cell2_x,
            y: grid_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("labs".into()),
        });

        // Cell 3: Projects
        let row2_y = grid_y + cell_h + 8.0 * scale;
        self.ui_batch.push_rect(pad, row2_y, cell_w, cell_h, 12.0 * scale, col_card, 1.0);
        self.draw_text(pad + 12.0 * scale, row2_y + 12.0 * scale, "Real Projects", 12.5 * scale, [0.10, 0.90, 0.60, 1.0]);
        self.draw_text(pad + 12.0 * scale, row2_y + 32.0 * scale, "Ecosystem source ->", 10.0 * scale, col_muted);
        self.hit_regions.push(HitRegion {
            x: pad,
            y: row2_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("projects".into()),
        });

        // Cell 4: Docs
        self.ui_batch.push_rect(cell2_x, row2_y, cell_w, cell_h, 12.0 * scale, col_card, 1.0);
        self.draw_text(cell2_x + 12.0 * scale, row2_y + 12.0 * scale, "Documentation", 12.5 * scale, [0.95, 0.70, 0.20, 1.0]);
        self.draw_text(cell2_x + 12.0 * scale, row2_y + 32.0 * scale, "Architecture specs ->", 10.0 * scale, col_muted);
        self.hit_regions.push(HitRegion {
            x: cell2_x,
            y: row2_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("docs".into()),
        });

        // 4. Engine Metrics Pill Row
        let stats_y = row2_y + cell_h + 8.0 * scale;
        let stats_h = 46.0 * scale;
        self.ui_batch.push_rect(pad, stats_y, cw, stats_h, 12.0 * scale, col_card, 1.0);
        let col_w = cw / 4.0;
        let metrics = [
            ("v2.0.0", "Compiler"),
            ("Rust NDK", "Runtime"),
            ("60 FPS", "EGL Canvas"),
            ("0% Java", "Codebase"),
        ];
        for (i, (val, lbl)) in metrics.iter().enumerate() {
            let cx = pad + (i as f32) * col_w;
            let v_w = self.measure_text(val, 12.0 * scale);
            let l_w = self.measure_text(lbl, 9.5 * scale);
            self.draw_text(cx + (col_w - v_w) * 0.5, stats_y + 8.0 * scale, val, 12.0 * scale, col_primary);
            self.draw_text(cx + (col_w - l_w) * 0.5, stats_y + 26.0 * scale, lbl, 9.5 * scale, col_muted);
        }
    }

    /// Destination 2: Explorer Screen
    fn render_explorer_content(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32, pad: f32, cw: f32, start_y: f32) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        self.draw_text(pad, start_y, "TechScript Explorer", 18.0 * scale, col_primary);
        self.draw_text_wrapped(
            pad,
            start_y + 22.0 * scale,
            cw,
            "Interactive gallery of verified native capabilities & reactive state",
            11.0 * scale,
            col_muted,
            3.0 * scale,
        );

        // Category Filter Pills in 2 Balanced Rows (3 per row)
        let cat_y = start_y + 44.0 * scale;
        let cat_h = 26.0 * scale;
        let cats = [
            "ALL", "UI BUTTONS", "CONTROLS",
            "SURFACES", "TELEMETRY", "NATIVE APIS",
        ];
        let pill_w = (cw - 2.0 * 8.0 * scale) / 3.0;

        for (i, cat) in cats.iter().enumerate() {
            let row = i / 3;
            let col = i % 3;
            let px = pad + (col as f32) * (pill_w + 8.0 * scale);
            let py = cat_y + (row as f32) * (cat_h + 6.0 * scale);
            let is_sel = runtime.explorer_category == i;
            let bg = if is_sel { col_primary } else { col_card };
            let fg = if is_sel { [1.0, 1.0, 1.0, 1.0] } else { col_muted };
            self.ui_batch.push_rect(px, py, pill_w, cat_h, 13.0 * scale, bg, if is_sel { 1.5 } else { 1.0 });
            let t_w = self.measure_text(cat, 10.0 * scale);
            self.draw_text(px + (pill_w - t_w) * 0.5, py + 8.0 * scale, cat, 10.0 * scale, fg);
            self.hit_regions.push(HitRegion {
                x: px,
                y: py,
                w: pill_w,
                h: cat_h,
                action: Action::SelectExplorerCategory(i),
            });
        }

        // Active Demonstration Card
        let demo_y = cat_y + 2.0 * (cat_h + 6.0 * scale) + 6.0 * scale;
        let demo_h = 240.0 * scale;
        self.ui_batch.push_rect(pad, demo_y, cw, demo_h, 16.0 * scale, col_card, 1.0);

        match runtime.explorer_category {
            0 | 1 => {
                // UI Components Demonstration
                self.draw_text(pad + 16.0 * scale, demo_y + 12.0 * scale, "Interactive Button Styles & States", 13.5 * scale, col_text);

                // Button 1: Glowing Primary with Counter
                let btn1_y = demo_y + 36.0 * scale;
                let btn_h = 38.0 * scale;
                self.ui_batch.push_rect(pad + 16.0 * scale, btn1_y, cw - 32.0 * scale, btn_h, 12.0 * scale, col_primary, 1.0);
                let btn1_lbl = format!("Primary Glow Button [ Taps: {} ]", runtime.demo_counter);
                self.draw_text(pad + 28.0 * scale, btn1_y + 11.0 * scale, &btn1_lbl, 12.0 * scale, [1.0, 1.0, 1.0, 1.0]);
                self.hit_regions.push(HitRegion {
                    x: pad + 16.0 * scale,
                    y: btn1_y,
                    w: cw - 32.0 * scale,
                    h: btn_h,
                    action: Action::IncrementCounter,
                });

                // Button 2: Liquid Glass Translucent
                let btn2_y = btn1_y + btn_h + 8.0 * scale;
                self.ui_batch.push_rect(pad + 16.0 * scale, btn2_y, cw - 32.0 * scale, btn_h, 12.0 * scale, [0.18, 0.22, 0.32, 0.85], 1.2);
                self.draw_text(pad + 28.0 * scale, btn2_y + 11.0 * scale, "Liquid Glass Translucent Surface", 12.0 * scale, col_accent);

                // Button 3: Cyan Outline Pill
                let btn3_y = btn2_y + btn_h + 8.0 * scale;
                self.ui_batch.push_rect(pad + 16.0 * scale, btn3_y, cw - 32.0 * scale, btn_h, 12.0 * scale, [0.06, 0.72, 0.50, 0.20], 1.2);
                self.draw_text(pad + 28.0 * scale, btn3_y + 11.0 * scale, "Cyan Rim Outline Pill", 12.0 * scale, [0.10, 0.90, 0.60, 1.0]);

                // Source Toggle Button
                let code_btn_y = btn3_y + btn_h + 10.0 * scale;
                self.draw_text(pad + 16.0 * scale, code_btn_y, "View .txs code: button(onClick: increment_counter)", 10.5 * scale, col_muted);
            }
            2 => {
                // Controls & Toggles
                self.draw_text(pad + 16.0 * scale, demo_y + 12.0 * scale, "Hardware Switches & Precision Sliders", 13.5 * scale, col_text);

                // Switch Row
                let sw_y = demo_y + 44.0 * scale;
                let is_on = runtime.demo_toggle;
                let sw_bg = if is_on { col_primary } else { [0.3, 0.3, 0.3, 0.6] };
                self.draw_text(pad + 16.0 * scale, sw_y + 6.0 * scale, "Hardware Acceleration:", 12.5 * scale, col_text);

                let sw_w = 52.0 * scale;
                let sw_h = 26.0 * scale;
                let sw_x = pad + cw - sw_w - 20.0 * scale;
                self.ui_batch.push_rect(sw_x, sw_y, sw_w, sw_h, 13.0 * scale, sw_bg, 1.0);
                let knob_x = if is_on { sw_x + sw_w - 24.0 * scale } else { sw_x + 2.0 * scale };
                self.ui_batch.push_rect(knob_x, sw_y + 2.0 * scale, 22.0 * scale, 22.0 * scale, 11.0 * scale, [1.0, 1.0, 1.0, 1.0], 0.0);
                self.hit_regions.push(HitRegion {
                    x: sw_x,
                    y: sw_y,
                    w: sw_w,
                    h: sw_h,
                    action: Action::ToggleSwitch,
                });

                // Slider Row
                let sl_y = sw_y + 46.0 * scale;
                self.draw_text(pad + 16.0 * scale, sl_y, "Glass Blur Intensity: 65%", 12.0 * scale, col_text);
                let track_y = sl_y + 20.0 * scale;
                let track_w = cw - 32.0 * scale;
                self.ui_batch.push_rect(pad + 16.0 * scale, track_y, track_w, 8.0 * scale, 4.0 * scale, [0.2, 0.25, 0.35, 0.7], 0.0);
                self.ui_batch.push_rect(pad + 16.0 * scale, track_y, track_w * runtime.demo_slider, 8.0 * scale, 4.0 * scale, col_accent, 0.0);

                let hint_y = track_y + 22.0 * scale;
                self.draw_text(pad + 16.0 * scale, hint_y, "SDF-evaluated realtime shader blurring", 10.5 * scale, col_muted);
            }
            3 => {
                // Surfaces & Depth
                self.draw_text(pad + 16.0 * scale, demo_y + 12.0 * scale, "Spatial Glass Depth & Elevation", 13.5 * scale, col_text);
                self.draw_text_wrapped(
                    pad + 16.0 * scale,
                    demo_y + 34.0 * scale,
                    cw - 32.0 * scale,
                    "SDF-evaluated rounded surfaces with specular light refraction and antialiased edges.",
                    11.0 * scale,
                    col_muted,
                    3.0 * scale,
                );

                let layer_y = demo_y + 78.0 * scale;
                self.ui_batch.push_rect(pad + 16.0 * scale, layer_y, cw - 32.0 * scale, 76.0 * scale, 12.0 * scale, [0.39, 0.40, 0.95, 0.25], 1.2);
                self.draw_text(pad + 28.0 * scale, layer_y + 16.0 * scale, "Inner Glass Layer", 13.0 * scale, col_accent);
                self.draw_text(pad + 28.0 * scale, layer_y + 36.0 * scale, "Depth Z = +2.4dp • Ambient Occlusion Rim", 10.5 * scale, col_muted);
            }
            4 => {
                // Data Visualization
                self.draw_text(pad + 16.0 * scale, demo_y + 12.0 * scale, "Realtime Telemetry & Data Viz", 13.5 * scale, col_text);

                let chart_y = demo_y + 38.0 * scale;
                let bars = [45.0, 78.0, 62.0, 95.0, 52.0, 88.0, 105.0];
                let bar_w = (cw - 48.0 * scale) / bars.len() as f32;
                for (bi, bh) in bars.iter().enumerate() {
                    let bx = pad + 16.0 * scale + (bi as f32) * bar_w;
                    let norm_h = (bh / 110.0) * 90.0 * scale;
                    self.ui_batch.push_rect(bx, chart_y + 100.0 * scale - norm_h, bar_w - 6.0 * scale, norm_h, 4.0 * scale, col_primary, 0.0);
                }
                self.draw_text(pad + 16.0 * scale, chart_y + 116.0 * scale, "Frame Render Time: 16.6ms (Deterministic 60 FPS)", 11.0 * scale, col_accent);
            }
            _ => {
                // Platform & Architecture
                self.draw_text(pad + 16.0 * scale, demo_y + 12.0 * scale, "Android Platform Integration", 13.5 * scale, col_text);
                let info = "Lifecycle: RESUMED • NativeActivity Active\nMemory: Zero GC • Deterministic RAII Rust\nRenderer: Hardware EGL 1.4 + OpenGL ES 2.0\nJava/Kotlin: 0% in Application Codebase";
                self.draw_text_wrapped(pad + 16.0 * scale, demo_y + 36.0 * scale, cw - 32.0 * scale, info, 11.5 * scale, col_muted, 4.0 * scale);
            }
        }

        // Secondary Card: Reactive State Architecture Preview
        let card2_y = demo_y + demo_h + 10.0 * scale;
        let card2_h = 130.0 * scale;
        self.ui_batch.push_rect(pad, card2_y, cw, card2_h, 14.0 * scale, col_card, 1.0);
        self.draw_text(pad + 16.0 * scale, card2_y + 12.0 * scale, "Reactive State & Compiler Integration", 13.0 * scale, col_accent);
        let arch_desc = "TechScript compiles `.txs` components into deterministic native state machines running directly on Android NativeActivity without web runtimes or JavaScript interpreters.";
        self.draw_text_wrapped(pad + 16.0 * scale, card2_y + 32.0 * scale, cw - 32.0 * scale, arch_desc, 10.5 * scale, col_muted, 3.0 * scale);

        let inc_btn_w = 170.0 * scale;
        let inc_btn_h = 28.0 * scale;
        let inc_btn_y = card2_y + card2_h - inc_btn_h - 10.0 * scale;
        self.ui_batch.push_rect(pad + 16.0 * scale, inc_btn_y, inc_btn_w, inc_btn_h, 14.0 * scale, [0.39, 0.40, 0.95, 0.30], 1.0);
        self.draw_text(pad + 28.0 * scale, inc_btn_y + 8.0 * scale, "[ TAP TO INCREMENT STATE ]", 10.5 * scale, col_primary);
        self.hit_regions.push(HitRegion {
            x: pad + 16.0 * scale,
            y: inc_btn_y,
            w: inc_btn_w,
            h: inc_btn_h,
            action: Action::IncrementCounter,
        });
    }

    /// Destination 3: Labs Screen (3D & Motion Playground)
    fn render_labs_content(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32, pad: f32, cw: f32, start_y: f32) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        self.draw_text(pad, start_y, "Animation & 3D Lab", 18.0 * scale, col_accent);
        self.draw_text_wrapped(
            pad,
            start_y + 22.0 * scale,
            cw,
            "Real procedural geometry, Blinn-Phong lighting, & camera physics",
            11.0 * scale,
            col_muted,
            3.0 * scale,
        );

        // 3D Viewport Card
        let vp_y = start_y + 44.0 * scale;
        let vp_h = 230.0 * scale;
        self.ui_batch.push_rect(pad, vp_y, cw, vp_h, 16.0 * scale, col_card, 1.2);

        // Viewport Header Badges
        self.draw_text(pad + 14.0 * scale, vp_y + 12.0 * scale, "3D Playground Viewport", 13.0 * scale, col_text);
        let mat_lbl = format!("[ {} ]", runtime.material.name());
        let mat_w = self.measure_text(&mat_lbl, 11.0 * scale);
        self.draw_text(pad + cw - mat_w - 14.0 * scale, vp_y + 12.0 * scale, &mat_lbl, 11.0 * scale, col_primary);

        let mesh_lbl = format!("Procedural: {}", runtime.mesh_type.name());
        self.draw_text(pad + 14.0 * scale, vp_y + 28.0 * scale, &mesh_lbl, 10.5 * scale, col_muted);

        // Mesh Switcher Pills Row
        let mesh_y = vp_y + vp_h + 8.0 * scale;
        let meshes = [
            (MeshType::Crystal, "Crystal"),
            (MeshType::Octahedron, "Octahedron"),
            (MeshType::HexPrism, "Prism"),
            (MeshType::StarGem, "StarGem"),
        ];
        let pill_w = (cw - 3.0 * 6.0 * scale) / 4.0;
        let pill_h = 28.0 * scale;

        for (i, (m_type, label)) in meshes.iter().enumerate() {
            let px = pad + (i as f32) * (pill_w + 6.0 * scale);
            let is_sel = runtime.mesh_type == *m_type;
            let bg = if is_sel { col_primary } else { col_card };
            let fg = if is_sel { [1.0, 1.0, 1.0, 1.0] } else { col_muted };
            self.ui_batch.push_rect(px, mesh_y, pill_w, pill_h, 12.0 * scale, bg, 1.0);
            let t_w = self.measure_text(label, 10.5 * scale);
            self.draw_text(px + (pill_w - t_w) * 0.5, mesh_y + 8.0 * scale, label, 10.5 * scale, fg);
            self.hit_regions.push(HitRegion {
                x: px,
                y: mesh_y,
                w: pill_w,
                h: pill_h,
                action: Action::Cycle3DMesh,
            });
        }

        // Material Switcher Pills Row
        let mat_y = mesh_y + pill_h + 6.0 * scale;
        let materials = [
            (MaterialPreset::TechScriptIndigo, "Indigo"),
            (MaterialPreset::LiquidCyan, "Cyan"),
            (MaterialPreset::AmethystPurple, "Amethyst"),
            (MaterialPreset::EmeraldChrome, "Emerald"),
        ];
        for (i, (mat_preset, label)) in materials.iter().enumerate() {
            let px = pad + (i as f32) * (pill_w + 6.0 * scale);
            let is_sel = runtime.material == *mat_preset;
            let bg = if is_sel { col_accent } else { col_card };
            let fg = if is_sel { [1.0, 1.0, 1.0, 1.0] } else { col_muted };
            self.ui_batch.push_rect(px, mat_y, pill_w, pill_h, 12.0 * scale, bg, 1.0);
            let t_w = self.measure_text(label, 10.5 * scale);
            self.draw_text(px + (pill_w - t_w) * 0.5, mat_y + 8.0 * scale, label, 10.5 * scale, fg);
            self.hit_regions.push(HitRegion {
                x: px,
                y: mat_y,
                w: pill_w,
                h: pill_h,
                action: Action::Cycle3DMaterial,
            });
        }

        // Camera & Physics Controls Row
        let ctrl_y = mat_y + pill_h + 6.0 * scale;
        let ctrl_w = (cw - 3.0 * 6.0 * scale) / 4.0;
        let rot_lbl = if runtime.auto_rotate { "Auto: ON" } else { "Auto: OFF" };

        // Button 1: Auto Rotate Toggle
        self.ui_batch.push_rect(pad, ctrl_y, ctrl_w, pill_h, 12.0 * scale, col_card, 1.0);
        let r_w = self.measure_text(rot_lbl, 10.0 * scale);
        self.draw_text(pad + (ctrl_w - r_w) * 0.5, ctrl_y + 8.0 * scale, rot_lbl, 10.0 * scale, col_text);
        self.hit_regions.push(HitRegion {
            x: pad,
            y: ctrl_y,
            w: ctrl_w,
            h: pill_h,
            action: Action::ToggleAutoRotate,
        });

        // Button 2: Zoom +
        let z1_x = pad + ctrl_w + 6.0 * scale;
        self.ui_batch.push_rect(z1_x, ctrl_y, ctrl_w, pill_h, 12.0 * scale, col_card, 1.0);
        let z1_w = self.measure_text("Zoom +", 10.0 * scale);
        self.draw_text(z1_x + (ctrl_w - z1_w) * 0.5, ctrl_y + 8.0 * scale, "Zoom +", 10.0 * scale, col_text);
        self.hit_regions.push(HitRegion {
            x: z1_x,
            y: ctrl_y,
            w: ctrl_w,
            h: pill_h,
            action: Action::AdjustZoom(0.2),
        });

        // Button 3: Zoom -
        let z2_x = z1_x + ctrl_w + 6.0 * scale;
        self.ui_batch.push_rect(z2_x, ctrl_y, ctrl_w, pill_h, 12.0 * scale, col_card, 1.0);
        let z2_w = self.measure_text("Zoom -", 10.0 * scale);
        self.draw_text(z2_x + (ctrl_w - z2_w) * 0.5, ctrl_y + 8.0 * scale, "Zoom -", 10.0 * scale, col_text);
        self.hit_regions.push(HitRegion {
            x: z2_x,
            y: ctrl_y,
            w: ctrl_w,
            h: pill_h,
            action: Action::AdjustZoom(-0.2),
        });

        // Button 4: Reset Camera
        let rst_x = z2_x + ctrl_w + 6.0 * scale;
        self.ui_batch.push_rect(rst_x, ctrl_y, ctrl_w, pill_h, 12.0 * scale, col_card, 1.0);
        let rst_w = self.measure_text("Reset", 10.0 * scale);
        self.draw_text(rst_x + (ctrl_w - rst_w) * 0.5, ctrl_y + 8.0 * scale, "Reset", 10.0 * scale, col_accent);
        self.hit_regions.push(HitRegion {
            x: rst_x,
            y: ctrl_y,
            w: ctrl_w,
            h: pill_h,
            action: Action::Reset3DCamera,
        });

        // Geometry Telemetry Info Card
        let info_y = ctrl_y + pill_h + 8.0 * scale;
        self.ui_batch.push_rect(pad, info_y, cw, 40.0 * scale, 12.0 * scale, [0.18, 0.22, 0.32, 0.7], 1.0);
        let geom_info = format!("Indices: {} • Specular: Blinn-Phong • Shading: Glass Fresnel", self.mesh3d_index_count);
        self.draw_text(pad + 12.0 * scale, info_y + 13.0 * scale, &geom_info, 10.5 * scale, col_muted);
    }

    /// Destination 4: Projects Screen
    fn render_projects_content(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32, pad: f32, cw: f32, start_y: f32) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        if runtime.show_source {
            // Source Code Inspector View
            let prj = &runtime.projects[runtime.selected_project];
            self.draw_text(pad, start_y, prj.name, 18.0 * scale, col_primary);
            self.draw_text(pad, start_y + 22.0 * scale, prj.path, 11.0 * scale, col_muted);

            // Back button
            let back_btn_w = 80.0 * scale;
            let back_btn_x = pad + cw - back_btn_w;
            self.ui_batch.push_rect(back_btn_x, start_y, back_btn_w, 28.0 * scale, 14.0 * scale, col_card, 1.0);
            self.draw_text(back_btn_x + 12.0 * scale, start_y + 8.0 * scale, "< BACK", 11.0 * scale, col_accent);
            self.hit_regions.push(HitRegion {
                x: back_btn_x,
                y: start_y,
                w: back_btn_w,
                h: 28.0 * scale,
                action: Action::ToggleCodeView,
            });

            // Action: Validate Project
            let val_y = start_y + 42.0 * scale;
            let val_btn_w = 160.0 * scale;
            let (val_lbl, val_col) = if runtime.project_validated {
                ("[✓ VALID AST (0 ERRORS)]", [0.10, 0.90, 0.60, 1.0])
            } else {
                ("[ VALIDATE PROJECT ]", col_primary)
            };
            self.ui_batch.push_rect(pad, val_y, val_btn_w, 28.0 * scale, 14.0 * scale, col_card, 1.0);
            self.draw_text(pad + 12.0 * scale, val_y + 7.0 * scale, val_lbl, 10.5 * scale, val_col);
            self.hit_regions.push(HitRegion {
                x: pad,
                y: val_y,
                w: val_btn_w,
                h: 28.0 * scale,
                action: Action::ValidateProject,
            });

            // Code Terminal Card
            let code_y = val_y + 36.0 * scale;
            let code_h = 240.0 * scale;
            self.ui_batch.push_rect(pad, code_y, cw, code_h, 14.0 * scale, [0.08, 0.10, 0.16, 0.95], 1.2);

            let mut line_y = code_y + 14.0 * scale;
            for (idx, line) in prj.code_sample.lines().enumerate() {
                let num_str = format!("{:2} | ", idx + 1);
                self.draw_text(pad + 14.0 * scale, line_y, &num_str, 11.0 * scale, col_muted);
                self.draw_text(pad + 50.0 * scale, line_y, line, 11.0 * scale, col_text);
                line_y += 20.0 * scale;
            }
        } else {
            // Projects List View
            self.draw_text(pad, start_y, "TechScript Ecosystem", 18.0 * scale, col_primary);
            self.draw_text(pad, start_y + 22.0 * scale, "Real projects from the active repository", 11.0 * scale, col_muted);

            let mut list_y = start_y + 44.0 * scale;
            let item_h = 58.0 * scale;

            for (i, prj) in runtime.projects.iter().take(5).enumerate() {
                self.ui_batch.push_rect(pad, list_y, cw, item_h, 12.0 * scale, col_card, 1.0);
                self.draw_text(pad + 14.0 * scale, list_y + 10.0 * scale, prj.name, 13.0 * scale, col_text);
                self.draw_text(pad + 14.0 * scale, list_y + 32.0 * scale, prj.path, 10.5 * scale, col_muted);

                let cat_badge = format!("[ {} ]", prj.category);
                let cat_w = self.measure_text(&cat_badge, 10.5 * scale);
                self.draw_text(pad + cw - cat_w - 36.0 * scale, list_y + 10.0 * scale, &cat_badge, 10.5 * scale, col_accent);
                self.draw_text(pad + cw - 24.0 * scale, list_y + 32.0 * scale, "->", 11.5 * scale, col_primary);

                self.hit_regions.push(HitRegion {
                    x: pad,
                    y: list_y,
                    w: cw,
                    h: item_h,
                    action: Action::SelectProject(i),
                });

                list_y += item_h + 8.0 * scale;
            }
        }
    }

    /// Destination 5: Docs Screen
    fn render_docs_content(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32, pad: f32, cw: f32, start_y: f32) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        self.draw_text(pad, start_y, "Documentation & Guides", 18.0 * scale, col_primary);
        self.draw_text(pad, start_y + 22.0 * scale, "Verified specifications from TechScript codebase", 11.0 * scale, col_muted);

        // Topic Selector Pills
        let topic_y = start_y + 44.0 * scale;
        let pill_h = 26.0 * scale;
        let topics = ["Start", "Arch", "3D", "Web", "Syntax"];
        let pill_w = (cw - 4.0 * 6.0 * scale) / 5.0;

        for (i, t_name) in topics.iter().enumerate() {
            let px = pad + (i as f32) * (pill_w + 6.0 * scale);
            let is_sel = runtime.selected_doc_topic == i;
            let bg = if is_sel { col_primary } else { col_card };
            let fg = if is_sel { [1.0, 1.0, 1.0, 1.0] } else { col_muted };
            self.ui_batch.push_rect(px, topic_y, pill_w, pill_h, 13.0 * scale, bg, 1.0);
            let t_w = self.measure_text(t_name, 10.5 * scale);
            self.draw_text(px + (pill_w - t_w) * 0.5, topic_y + 7.0 * scale, t_name, 10.5 * scale, fg);
            self.hit_regions.push(HitRegion {
                x: px,
                y: topic_y,
                w: pill_w,
                h: pill_h,
                action: Action::SelectDocTopic(i),
            });
        }

        // Active Topic Content Card
        let doc_card_y = topic_y + pill_h + 10.0 * scale;
        let doc_card_h = 240.0 * scale;
        self.ui_batch.push_rect(pad, doc_card_y, cw, doc_card_h, 16.0 * scale, col_card, 1.0);

        let topic = &runtime.doc_topics[runtime.selected_doc_topic];
        self.draw_text(pad + 16.0 * scale, doc_card_y + 12.0 * scale, topic.title, 13.5 * scale, col_text);
        let cat_badge = format!("[ {} ]", topic.category);
        let cat_w = self.measure_text(&cat_badge, 10.5 * scale);
        self.draw_text(pad + cw - cat_w - 16.0 * scale, doc_card_y + 12.0 * scale, &cat_badge, 10.5 * scale, col_accent);

        // Content Body (wrapped)
        self.draw_text_wrapped(pad + 16.0 * scale, doc_card_y + 36.0 * scale, cw - 32.0 * scale, topic.content, 11.0 * scale, col_muted, 3.5 * scale);

        // Code Snippet Terminal if available
        if let Some(snippet) = topic.code_snippet {
            let snip_y = doc_card_y + 130.0 * scale;
            let snip_h = 92.0 * scale;
            self.ui_batch.push_rect(pad + 14.0 * scale, snip_y, cw - 28.0 * scale, snip_h, 10.0 * scale, [0.08, 0.10, 0.16, 0.95], 1.0);
            self.draw_text(pad + 22.0 * scale, snip_y + 12.0 * scale, snippet, 10.5 * scale, col_text);
        }

        // Verified Official Links Row
        let links_y = doc_card_y + doc_card_h + 10.0 * scale;
        let link_w = (cw - 8.0 * scale) * 0.5;
        let link_h = 42.0 * scale;

        // Link 1: Official Site
        self.ui_batch.push_rect(pad, links_y, link_w, link_h, 12.0 * scale, [0.39, 0.40, 0.95, 0.25], 1.0);
        self.draw_text(pad + 12.0 * scale, links_y + 13.0 * scale, "techscript.is-a.dev", 11.5 * scale, col_text);
        self.draw_text(pad + link_w - 24.0 * scale, links_y + 13.0 * scale, "->", 11.5 * scale, col_primary);
        self.hit_regions.push(HitRegion {
            x: pad,
            y: links_y,
            w: link_w,
            h: link_h,
            action: Action::OpenLink("https://techscript.is-a.dev/techscript/".into()),
        });

        // Link 2: GitHub Repository
        let link2_x = pad + link_w + 8.0 * scale;
        self.ui_batch.push_rect(link2_x, links_y, link_w, link_h, 12.0 * scale, [0.12, 0.16, 0.24, 0.85], 1.0);
        self.draw_text(link2_x + 12.0 * scale, links_y + 13.0 * scale, "GitHub Repository", 11.5 * scale, col_accent);
        self.draw_text(link2_x + link_w - 24.0 * scale, links_y + 13.0 * scale, "->", 11.5 * scale, col_accent);
        self.hit_regions.push(HitRegion {
            x: link2_x,
            y: links_y,
            w: link_w,
            h: link_h,
            action: Action::OpenLink("https://github.com/Tcode-Motion/techscript".into()),
        });
    }

    /// Secondary Screen: About / Diagnostics Screen
    fn render_about_content(&mut self, runtime: &mut TechScriptRuntime, theme: &Theme, scale: f32, pad: f32, cw: f32, start_y: f32) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        self.draw_text(pad, start_y, "Diagnostics & About", 18.0 * scale, col_primary);
        self.draw_text(pad, start_y + 22.0 * scale, "Native engine state and device telemetry", 11.0 * scale, col_muted);

        // Back button
        let back_btn_w = 80.0 * scale;
        let back_btn_x = pad + cw - back_btn_w;
        self.ui_batch.push_rect(back_btn_x, start_y, back_btn_w, 28.0 * scale, 14.0 * scale, col_card, 1.0);
        self.draw_text(back_btn_x + 12.0 * scale, start_y + 8.0 * scale, "< CLOSE", 11.0 * scale, col_accent);
        self.hit_regions.push(HitRegion {
            x: back_btn_x,
            y: start_y,
            w: back_btn_w,
            h: 28.0 * scale,
            action: Action::Navigate("home".into()),
        });

        // Telemetry Card
        let card_y = start_y + 42.0 * scale;
        let card_h = 240.0 * scale;
        self.ui_batch.push_rect(pad, card_y, cw, card_h, 16.0 * scale, col_card, 1.0);

        let diag_text = format!(
            "Resolution: {}x{} (Scale: {:.2})\nRenderer: Hardware EGL 1.4 + OpenGL ES 2.0\nEntry Point: Android OS NativeActivity\nAuthored Java/Kotlin: 0% (Completely Independent)\nArchitecture: arm64-v8a (Release stripped)\nLanguage Spec: TechScript (.txs) v2.0\nRender Frames: {}\nDeveloper: Tanmoy Majumder (Tcode-Motion)\nPackage: com.techscript.appforge",
            self.width, self.height, scale, runtime.frame_count
        );
        self.draw_text_wrapped(pad + 16.0 * scale, card_y + 16.0 * scale, cw - 32.0 * scale, &diag_text, 11.0 * scale, col_text, 4.0 * scale);
    }

    /// Persistent Bottom Floating Liquid Glass Navigation Dock (5 Destinations)
    fn render_navigation_dock(&mut self, runtime: &TechScriptRuntime, theme: &Theme, scale: f32) {
        let sw = self.width as f32;
        let sh = self.height as f32;

        let dock_w = (sw - 24.0 * scale).min(520.0 * scale);
        let dock_h = 50.0 * scale;
        let dock_x = (sw - dock_w) * 0.5;
        let dock_y = sh - dock_h - 12.0 * scale;

        let col_dock = Theme::to_f32_color(theme.dock_bg);
        let col_active = Theme::to_f32_color(theme.dock_active);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_cyan = Theme::to_f32_color(theme.accent);
        let col_magenta = Theme::to_f32_color(theme.magenta);
        let col_violet = Theme::to_f32_color(theme.violet);

        // Dock background pill with specular border highlight
        self.ui_batch.push_rect(dock_x, dock_y, dock_w, dock_h, 25.0 * scale, col_dock, 1.5 * scale);

        let tabs = [
            ("home",     "HOME"),
            ("explorer", "EXPLORE"),
            ("labs",     "LABS"),
            ("projects", "PROJECTS"),
            ("docs",     "DOCS"),
        ];

        let tab_w = dock_w / 5.0;
        let active_screen = runtime.active_screen();

        for (i, (screen_id, label)) in tabs.iter().enumerate() {
            let tx = dock_x + (i as f32) * tab_w;
            let is_active = *screen_id == active_screen;

            if is_active {
                // Active refined glowing pill highlight (compact, balanced)
                let active_pad_x = 3.0 * scale;
                let active_pad_y = 4.0 * scale;
                self.ui_batch.push_rect(
                    tx + active_pad_x,
                    dock_y + active_pad_y,
                    tab_w - active_pad_x * 2.0,
                    dock_h - active_pad_y * 2.0,
                    21.0 * scale,
                    col_active,
                    1.2 * scale,
                );
            }

            let fg = if is_active { col_primary } else { col_muted };
            let icon_cx = tx + tab_w * 0.5;
            let icon_cy = dock_y + 16.0 * scale;
            let icon_sz = 16.0 * scale;

            // Render crisp procedural vector icon
            match *screen_id {
                "home" => self.ui_batch.draw_icon_home(icon_cx, icon_cy, icon_sz, fg),
                "explorer" => self.ui_batch.draw_icon_explorer(icon_cx, icon_cy, icon_sz, fg, if is_active { col_cyan } else { fg }),
                "labs" => self.ui_batch.draw_icon_labs(icon_cx, icon_cy, icon_sz, fg, if is_active { col_magenta } else { fg }),
                "projects" => self.ui_batch.draw_icon_projects(icon_cx, icon_cy, icon_sz, fg, if is_active { col_violet } else { fg }),
                "docs" => self.ui_batch.draw_icon_docs(icon_cx, icon_cy, icon_sz, fg),
                _ => {}
            }

            let l_w = self.measure_text(label, 8.5 * scale);
            self.draw_text(tx + (tab_w - l_w) * 0.5, dock_y + 34.0 * scale, label, 8.5 * scale, fg);

            // Register dock tap region
            self.hit_regions.push(HitRegion {
                x: tx,
                y: dock_y,
                w: tab_w,
                h: dock_h,
                action: Action::Navigate(screen_id.to_string()),
            });
        }
    }

    /// Upload UI batch to GPU and draw in a single OpenGL call
    unsafe fn draw_ui_batch(&self) {
        if self.ui_batch.indices.is_empty() {
            return;
        }

        glUseProgram(self.ui_program);

        let u_screen_size = glGetUniformLocation(self.ui_program, b"u_screen_size\0".as_ptr());
        glUniform2f(u_screen_size, self.width as f32, self.height as f32);

        let u_font_texture = glGetUniformLocation(self.ui_program, b"u_font_texture\0".as_ptr());
        glUniform1i(u_font_texture, 0);

        glBindTexture(GL_TEXTURE_2D, self.font_texture);

        // Upload vertex & index data
        glBindBuffer(GL_ARRAY_BUFFER, self.ui_vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            (self.ui_batch.vertices.len() * std::mem::size_of::<crate::glass_ui::UIVertex>()) as isize,
            self.ui_batch.vertices.as_ptr() as *const std::ffi::c_void,
            GL_DYNAMIC_DRAW,
        );

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ui_ibo);
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            (self.ui_batch.indices.len() * std::mem::size_of::<u16>()) as isize,
            self.ui_batch.indices.as_ptr() as *const std::ffi::c_void,
            GL_DYNAMIC_DRAW,
        );

        let stride = std::mem::size_of::<crate::glass_ui::UIVertex>() as i32;

        let a_pos          = glGetAttribLocation(self.ui_program, b"a_pos\0".as_ptr()) as u32;
        let a_uv           = glGetAttribLocation(self.ui_program, b"a_uv\0".as_ptr()) as u32;
        let a_color        = glGetAttribLocation(self.ui_program, b"a_color\0".as_ptr()) as u32;
        let a_local_pos    = glGetAttribLocation(self.ui_program, b"a_local_pos\0".as_ptr()) as u32;
        let a_size         = glGetAttribLocation(self.ui_program, b"a_size\0".as_ptr()) as u32;
        let a_radius       = glGetAttribLocation(self.ui_program, b"a_radius\0".as_ptr()) as u32;
        let a_border_width = glGetAttribLocation(self.ui_program, b"a_border_width\0".as_ptr()) as u32;
        let a_is_text      = glGetAttribLocation(self.ui_program, b"a_is_text\0".as_ptr()) as u32;

        glEnableVertexAttribArray(a_pos);
        glVertexAttribPointer(a_pos, 2, GL_FLOAT, 0, stride, std::ptr::null());

        glEnableVertexAttribArray(a_uv);
        glVertexAttribPointer(a_uv, 2, GL_FLOAT, 0, stride, 8 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_color);
        glVertexAttribPointer(a_color, 4, GL_FLOAT, 0, stride, 16 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_local_pos);
        glVertexAttribPointer(a_local_pos, 2, GL_FLOAT, 0, stride, 32 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_size);
        glVertexAttribPointer(a_size, 2, GL_FLOAT, 0, stride, 40 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_radius);
        glVertexAttribPointer(a_radius, 1, GL_FLOAT, 0, stride, 48 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_border_width);
        glVertexAttribPointer(a_border_width, 1, GL_FLOAT, 0, stride, 52 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_is_text);
        glVertexAttribPointer(a_is_text, 1, GL_FLOAT, 0, stride, 56 as *const std::ffi::c_void);

        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glDisable(GL_DEPTH_TEST);

        glDrawElements(GL_TRIANGLES, self.ui_batch.indices.len() as i32, GL_UNSIGNED_SHORT, std::ptr::null());
    }
}

impl Drop for NativeRenderer {
    fn drop(&mut self) {
        unsafe {
            if self.display != egl::NO_DISPLAY {
                egl::eglMakeCurrent(self.display, egl::NO_SURFACE, egl::NO_SURFACE, egl::NO_CONTEXT);
                if self.context != egl::NO_CONTEXT {
                    egl::eglDestroyContext(self.display, self.context);
                }
                if self.surface != egl::NO_SURFACE {
                    egl::eglDestroySurface(self.display, self.surface);
                }
                egl::eglTerminate(self.display);
            }
        }
    }
}

unsafe fn create_program(vs_src: &str, fs_src: &str) -> Result<u32, String> {
    let vs = compile_shader(GL_VERTEX_SHADER, vs_src)?;
    let fs = compile_shader(GL_FRAGMENT_SHADER, fs_src)?;

    let program = glCreateProgram();
    glAttachShader(program, vs);
    glAttachShader(program, fs);
    glLinkProgram(program);

    let mut linked = 0i32;
    glGetProgramiv(program, GL_LINK_STATUS, &mut linked);
    if linked == 0 {
        let mut log_len = 0i32;
        glGetProgramiv(program, 0x8B84, &mut log_len);
        let mut log_buf = vec![0u8; log_len as usize];
        glGetProgramInfoLog(program, log_len, std::ptr::null_mut(), log_buf.as_mut_ptr());
        let err = String::from_utf8_lossy(&log_buf).to_string();
        return Err(format!("Program link failed: {err}"));
    }

    glDeleteShader(vs);
    glDeleteShader(fs);
    Ok(program)
}

unsafe fn compile_shader(shader_type: u32, source: &str) -> Result<u32, String> {
    let shader = glCreateShader(shader_type);
    let src_ptr = source.as_ptr();
    let src_len = source.len() as i32;
    glShaderSource(shader, 1, &src_ptr, &src_len);
    glCompileShader(shader);

    let mut compiled = 0i32;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut compiled);
    if compiled == 0 {
        let mut log_len = 0i32;
        glGetShaderiv(shader, 0x8B84, &mut log_len);
        let mut log_buf = vec![0u8; log_len as usize];
        glGetShaderInfoLog(shader, log_len, std::ptr::null_mut(), log_buf.as_mut_ptr());
        let err = String::from_utf8_lossy(&log_buf).to_string();
        return Err(format!("Shader compilation failed: {err}"));
    }

    Ok(shader)
}
