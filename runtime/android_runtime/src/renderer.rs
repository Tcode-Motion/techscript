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
//   - Real .txs execution engine integration & live code runner
//   - Zero Java/Kotlin code, zero WebView
// =============================================================================

use crate::geometry3d::{self, MaterialPreset, MeshType};
use crate::glass_ui::{GlassBatch, MESH3D_FS_SOURCE, MESH3D_VS_SOURCE, UI_FS_SOURCE, UI_VS_SOURCE};
use crate::theme::Theme;
use crate::txs_runtime::TechScriptRuntime;
use crate::typography::{self, FontAtlas};
use android_activity::AndroidApp;

// ─────────────────────────────────────────────────────────────────────────────
// Interactive Hit-Test Regions
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Action {
    Navigate(String),
    ToggleTheme,
    Cycle3DMesh,
    Prev3DMesh,
    Cycle3DMaterial,
    Set3DMesh(MeshType),
    Set3DMaterial(MaterialPreset),
    ToggleAutoRotate,
    Reset3DCamera,
    Pan3D(f32, f32),
    Reset3DPosition,
    SetCustomHue(f32),
    AdjustZoom(f32),
    ToggleSpeed,
    SelectExplorerCategory(usize),
    IncrementCounter,
    DecrementCounter,
    ResetCounter,
    SetSlider(f32),
    ToggleSwitch,
    ToggleExplorerCode,
    SelectProject(usize),
    ToggleCodeView,
    ValidateProject,
    RunSelectedProject,
    RunQuickAction(String),
    SelectDocTopic(usize),
    OpenAbout,
    OpenLink(String),
    SelectModelCategory(usize),
    ToggleWireframe,
    SelectProjectFilter(usize),
    RunProjectDirect(usize),
    ValidateProjectDirect(usize),
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
// Raw OpenGL ES 2.0 and EGL Type Definitions & Extern FFI
// ─────────────────────────────────────────────────────────────────────────────

type GLenum = u32;
type GLboolean = u8;
type GLbitfield = u32;
type GLint = i32;
type GLsizei = i32;
type GLuint = u32;
type GLfloat = f32;
type GLchar = std::os::raw::c_char;
type GLsizeiptr = isize;

const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
const GL_LINES: GLenum = 0x0001;
const GL_TRIANGLES: GLenum = 0x0004;
const GL_LEQUAL: GLenum = 0x0203;
const GL_SRC_ALPHA: GLenum = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
const GL_BLEND: GLenum = 0x0BE2;
const GL_SCISSOR_TEST: GLenum = 0x0C11;
const GL_DEPTH_TEST: GLenum = 0x0B71;
const GL_TEXTURE_2D: GLenum = 0x0DE1;
const GL_UNSIGNED_SHORT: GLenum = 0x1403;
const GL_FLOAT: GLenum = 0x1406;
const GL_RGBA: GLenum = 0x1908;
const GL_UNSIGNED_BYTE: GLenum = 0x1401;
const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
const GL_VERTEX_SHADER: GLenum = 0x8B31;
const GL_COMPILE_STATUS: GLenum = 0x8B81;
const GL_LINK_STATUS: GLenum = 0x8B82;
const GL_ARRAY_BUFFER: GLenum = 0x8892;
const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
const GL_STATIC_DRAW: GLenum = 0x88E4;
const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
const GL_LINEAR: i32 = 0x2601;
const GL_CLAMP_TO_EDGE: i32 = 0x812F;
const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
const GL_TEXTURE_WRAP_T: GLenum = 0x2803;

extern "C" {
    fn glClearColor(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat);
    fn glClear(mask: GLbitfield);
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glEnable(cap: GLenum);
    fn glDisable(cap: GLenum);
    fn glDepthFunc(func: GLenum);
    fn glDepthMask(flag: GLboolean);
    fn glLineWidth(width: GLfloat);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glCreateShader(type_: GLenum) -> GLuint;
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    fn glCompileShader(shader: GLuint);
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    fn glCreateProgram() -> GLuint;
    fn glAttachShader(program: GLuint, shader: GLuint);
    fn glLinkProgram(program: GLuint);
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    fn glUseProgram(program: GLuint);
    fn glDeleteShader(shader: GLuint);
    fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn glEnableVertexAttribArray(index: GLuint);
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const std::ffi::c_void,
    );
    fn glUniform1i(location: GLint, v0: GLint);
    fn glUniform1f(location: GLint, v0: GLfloat);
    fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
    fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
    fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
    fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    fn glBindBuffer(target: GLenum, buffer: GLuint);
    fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const std::ffi::c_void, usage: GLenum);
    fn glDrawElements(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const std::ffi::c_void,
    );
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const std::ffi::c_void,
    );
}

#[inline]
fn c_str(bytes: &[u8]) -> *const GLchar {
    bytes.as_ptr() as *const GLchar
}

mod egl {
    use std::ffi::c_void;
    pub type EGLDisplay = *mut c_void;
    pub type EGLConfig = *mut c_void;
    pub type EGLContext = *mut c_void;
    pub type EGLSurface = *mut c_void;
    pub type EGLint = i32;
    pub type EGLBoolean = u32;

    pub const EGL_DEFAULT_DISPLAY: *mut c_void = std::ptr::null_mut();
    pub const EGL_NO_CONTEXT: EGLContext = std::ptr::null_mut();
    pub const EGL_NO_SURFACE: EGLSurface = std::ptr::null_mut();
    pub const EGL_NONE: EGLint = 0x3038;
    pub const EGL_RENDERABLE_TYPE: EGLint = 0x3040;
    pub const EGL_OPENGL_ES2_BIT: EGLint = 0x0004;
    pub const EGL_SURFACE_TYPE: EGLint = 0x3033;
    pub const EGL_WINDOW_BIT: EGLint = 0x0004;
    pub const EGL_RED_SIZE: EGLint = 0x3024;
    pub const EGL_GREEN_SIZE: EGLint = 0x3023;
    pub const EGL_BLUE_SIZE: EGLint = 0x3022;
    pub const EGL_ALPHA_SIZE: EGLint = 0x3021;
    pub const EGL_DEPTH_SIZE: EGLint = 0x3025;
    pub const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;

    pub const EGL_WIDTH: EGLint = 0x3057;
    pub const EGL_HEIGHT: EGLint = 0x3056;

    extern "C" {
        pub fn eglGetDisplay(display_id: *mut c_void) -> EGLDisplay;
        pub fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint)
            -> EGLBoolean;
        pub fn eglChooseConfig(
            dpy: EGLDisplay,
            attrib_list: *const EGLint,
            configs: *mut EGLConfig,
            config_size: EGLint,
            num_config: *mut EGLint,
        ) -> EGLBoolean;
        pub fn eglCreateWindowSurface(
            dpy: EGLDisplay,
            config: EGLConfig,
            win: *mut c_void,
            attrib_list: *const EGLint,
        ) -> EGLSurface;
        pub fn eglCreateContext(
            dpy: EGLDisplay,
            config: EGLConfig,
            share_context: EGLContext,
            attrib_list: *const EGLint,
        ) -> EGLContext;
        pub fn eglMakeCurrent(
            dpy: EGLDisplay,
            draw: EGLSurface,
            read: EGLSurface,
            ctx: EGLContext,
        ) -> EGLBoolean;
        pub fn eglQuerySurface(
            dpy: EGLDisplay,
            surface: EGLSurface,
            attribute: EGLint,
            value: *mut EGLint,
        ) -> EGLBoolean;
        pub fn eglSwapBuffers(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
        pub fn eglSwapInterval(dpy: EGLDisplay, interval: EGLint) -> EGLBoolean;
        pub fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
        pub fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
        pub fn eglTerminate(dpy: EGLDisplay) -> EGLBoolean;
        pub fn eglGetError() -> EGLint;
    }
}

pub struct NativeRenderer {
    display: egl::EGLDisplay,
    surface: egl::EGLSurface,
    context: egl::EGLContext,
    pub width: i32,
    pub height: i32,

    // 2D UI Shader & Buffers
    ui_program: GLuint,
    ui_vbo: GLuint,
    ui_ibo: GLuint,
    pub ui_batch: GlassBatch,
    font_texture: GLuint,
    pub font_atlas: FontAtlas,

    // 3D Geometry Shader & Buffers
    mesh3d_program: GLuint,
    mesh3d_vbo: GLuint,
    mesh3d_ibo: GLuint,
    mesh3d_index_count: i32,
    mesh3d_wire_ibo: GLuint,
    mesh3d_wire_count: i32,
    current_mesh: MeshType,
    current_material: MaterialPreset,
    current_custom_hue: f32,
    current_is_custom: bool,
    current_wireframe_mode: bool,

    // Interaction & Telemetry
    pub hit_regions: Vec<HitRegion>,
    pub anim_time: f32,
    pub viewport_3d_rect: Option<(f32, f32, f32, f32)>,
}

impl NativeRenderer {
    pub fn new(app: &AndroidApp, _theme: &Theme) -> Result<Self, String> {
        let native_window = app
            .native_window()
            .ok_or("No native window available from Android OS")?;
        let width = native_window.width();
        let height = native_window.height();
        let window_ptr = native_window.ptr().as_ptr() as *mut std::ffi::c_void;

        unsafe {
            let display = egl::eglGetDisplay(egl::EGL_DEFAULT_DISPLAY);
            if display.is_null() {
                return Err("eglGetDisplay failed".into());
            }

            let mut major = 0;
            let mut minor = 0;
            if egl::eglInitialize(display, &mut major, &mut minor) == 0 {
                return Err("eglInitialize failed".into());
            }
            log::info!("[TechScript EGL] EGL {major}.{minor} initialized successfully");

            let attribs = [
                egl::EGL_RENDERABLE_TYPE,
                egl::EGL_OPENGL_ES2_BIT,
                egl::EGL_SURFACE_TYPE,
                egl::EGL_WINDOW_BIT,
                egl::EGL_RED_SIZE,
                8,
                egl::EGL_GREEN_SIZE,
                8,
                egl::EGL_BLUE_SIZE,
                8,
                egl::EGL_ALPHA_SIZE,
                8,
                egl::EGL_DEPTH_SIZE,
                16,
                egl::EGL_NONE,
            ];

            let mut config: egl::EGLConfig = std::ptr::null_mut();
            let mut num_configs = 0;
            if egl::eglChooseConfig(display, attribs.as_ptr(), &mut config, 1, &mut num_configs)
                == 0
                || num_configs == 0
            {
                return Err("eglChooseConfig failed".into());
            }

            let surface =
                egl::eglCreateWindowSurface(display, config, window_ptr, std::ptr::null());
            if surface == egl::EGL_NO_SURFACE {
                return Err("eglCreateWindowSurface failed".into());
            }

            let ctx_attribs = [egl::EGL_CONTEXT_CLIENT_VERSION, 2, egl::EGL_NONE];
            let context =
                egl::eglCreateContext(display, config, egl::EGL_NO_CONTEXT, ctx_attribs.as_ptr());
            if context == egl::EGL_NO_CONTEXT {
                return Err("eglCreateContext failed".into());
            }

            if egl::eglMakeCurrent(display, surface, surface, context) == 0 {
                return Err("eglMakeCurrent failed".into());
            }
            egl::eglSwapInterval(display, 1);

            glViewport(0, 0, width, height);

            // Compile 2D UI Shader
            let ui_program = Self::create_program(UI_VS_SOURCE, UI_FS_SOURCE)?;

            // Compile 3D Mesh Shader
            let mesh3d_program = Self::create_program(MESH3D_VS_SOURCE, MESH3D_FS_SOURCE)?;

            // Generate font atlas texture
            let font_atlas = FontAtlas::create_system_atlas();
            let mut font_texture = 0;
            glGenTextures(1, &mut font_texture);
            glBindTexture(GL_TEXTURE_2D, font_texture);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                typography::ATLAS_WIDTH as i32,
                typography::ATLAS_HEIGHT as i32,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                font_atlas.atlas_rgba.as_ptr() as *const std::ffi::c_void,
            );

            // Buffers
            let mut ui_buffers = [0u32; 2];
            glGenBuffers(2, ui_buffers.as_mut_ptr());

            let mut mesh_buffers = [0u32; 3];
            glGenBuffers(3, mesh_buffers.as_mut_ptr());

            let initial_mesh = MeshType::TechCore;
            let initial_mat = MaterialPreset::LiquidCyan;
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

            // Generate initial wireframe lines for geometric view
            let mut wire_indices: Vec<u16> = Vec::with_capacity(i3d.len() * 2);
            for tri in i3d.chunks_exact(3) {
                wire_indices.push(tri[0]);
                wire_indices.push(tri[1]);
                wire_indices.push(tri[1]);
                wire_indices.push(tri[2]);
                wire_indices.push(tri[2]);
                wire_indices.push(tri[0]);
            }
            let mesh3d_wire_count = wire_indices.len() as i32;

            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, mesh_buffers[2]);
            glBufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (wire_indices.len() * std::mem::size_of::<u16>()) as isize,
                wire_indices.as_ptr() as *const std::ffi::c_void,
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
                mesh3d_wire_ibo: mesh_buffers[2],
                mesh3d_wire_count,
                current_mesh: initial_mesh,
                current_material: initial_mat,
                current_custom_hue: -1.0,
                current_is_custom: false,
                current_wireframe_mode: false,
                hit_regions: Vec::new(),
                anim_time: 0.0,
                viewport_3d_rect: None,
            })
        }
    }

    /// Render modern high-DPI text with proportional font metrics
    pub fn draw_text(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        font_size_px: f32,
        color: [f32; 4],
    ) -> f32 {
        self.ui_batch
            .push_text_ttf(&self.font_atlas, x, y, text, font_size_px, color)
    }

    /// Measures exact visual pixel width of a string at the specified font size
    pub fn measure_text(&self, text: &str, font_size_px: f32) -> f32 {
        self.font_atlas.measure_text(text, font_size_px)
    }

    /// Render word-wrapped multiline text ensuring it never overflows max_w
    pub fn draw_text_wrapped(
        &mut self,
        x: f32,
        y: f32,
        max_w: f32,
        text: &str,
        font_size_px: f32,
        color: [f32; 4],
        line_gap: f32,
    ) -> f32 {
        let lines = self.font_atlas.wrap_text(text, max_w, font_size_px);
        let line_h = font_size_px + line_gap;
        for (i, line) in lines.iter().enumerate() {
            self.draw_text(x, y + (i as f32) * line_h, line, font_size_px, color);
        }
        lines.len() as f32 * line_h
    }

    /// Render multiline text by line breaks with uniform line spacing
    pub fn draw_multiline_text(
        &mut self,
        x: f32,
        mut y: f32,
        text: &str,
        font_size_px: f32,
        line_gap: f32,
        color: [f32; 4],
    ) -> f32 {
        let line_h = font_size_px + line_gap;
        let start_y = y;
        for line in text.lines() {
            self.draw_text(x, y, line, font_size_px, color);
            y += line_h;
        }
        y - start_y
    }

    /// Renders a single line of TechScript code with rich, colorful syntax highlighting
    pub fn draw_syntax_code_line(
        &mut self,
        x: f32,
        y: f32,
        line: &str,
        font_size_px: f32,
        is_dark: bool,
    ) -> f32 {
        let trimmed = line.trim_start();
        let indent_spaces = line.len() - trimmed.len();
        let mut curr_x = x + (indent_spaces as f32) * (font_size_px * 0.48);

        if trimmed.starts_with('#') {
            // Comments in muted slate
            let comment_col = if is_dark {
                [0.45, 0.55, 0.75, 0.85]
            } else {
                [0.35, 0.42, 0.52, 1.0]
            };
            self.draw_text(curr_x, y, trimmed, font_size_px, comment_col);
            return curr_x + self.measure_text(trimmed, font_size_px);
        }

        let chars: Vec<char> = trimmed.chars().collect();
        let mut i = 0;
        let n = chars.len();

        while i < n {
            if chars[i] == '"' {
                // String literal in emerald green
                let mut s = String::new();
                s.push('"');
                i += 1;
                while i < n && chars[i] != '"' {
                    s.push(chars[i]);
                    i += 1;
                }
                if i < n {
                    s.push('"');
                    i += 1;
                }
                let w = self.measure_text(&s, font_size_px);
                let str_col = if is_dark {
                    [0.0, 0.90, 0.60, 1.0]
                } else {
                    [0.02, 0.48, 0.35, 1.0]
                };
                self.draw_text(curr_x, y, &s, font_size_px, str_col);
                curr_x += w;
            } else if chars[i].is_alphabetic() || chars[i] == '_' {
                let mut word = String::new();
                while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    word.push(chars[i]);
                    i += 1;
                }
                let color = match word.as_str() {
                    "is" | "becomes" | "constant" | "to" | "return" | "say" | "repeat"
                    | "times" | "if" | "else" | "for" | "in" | "while" => {
                        if is_dark {
                            [0.78, 0.45, 1.0, 1.0]
                        } else {
                            [0.43, 0.16, 0.85, 1.0]
                        } // Royal Violet
                    }
                    "on_tap" | "on_primary_tap" | "calc_physics" | "render_surface" | "greet"
                    | "step" | "mutate" => {
                        if is_dark {
                            [1.0, 0.45, 0.60, 1.0]
                        } else {
                            [0.75, 0.12, 0.25, 1.0]
                        } // Crimson Ruby
                    }
                    "counter" | "gravity" | "velocity" | "glow" | "title" | "result" | "speed"
                    | "radius" => {
                        if is_dark {
                            [0.0, 0.88, 1.0, 1.0]
                        } else {
                            [0.01, 0.45, 0.75, 1.0]
                        } // Deep Azure
                    }
                    _ => {
                        if is_dark {
                            [0.90, 0.94, 1.0, 1.0]
                        } else {
                            [0.06, 0.09, 0.16, 1.0]
                        } // Deep Navy Slate
                    }
                };
                let w = self.measure_text(&word, font_size_px);
                self.draw_text(curr_x, y, &word, font_size_px, color);
                curr_x += w;
            } else if chars[i].is_ascii_digit() {
                let mut num = String::new();
                while i < n && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num.push(chars[i]);
                    i += 1;
                }
                let w = self.measure_text(&num, font_size_px);
                let num_col = if is_dark {
                    [1.0, 0.78, 0.20, 1.0]
                } else {
                    [0.72, 0.35, 0.05, 1.0]
                };
                self.draw_text(curr_x, y, &num, font_size_px, num_col);
                curr_x += w;
            } else if chars[i].is_whitespace() {
                let w = self.measure_text(" ", font_size_px);
                curr_x += w;
                i += 1;
            } else {
                let ch_str = chars[i].to_string();
                let sym_color = match chars[i] {
                    ':' | '(' | ')' | '{' | '}' => {
                        if is_dark {
                            [0.0, 0.90, 1.0, 0.9]
                        } else {
                            [0.12, 0.25, 0.69, 1.0]
                        }
                    }
                    '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                        if is_dark {
                            [1.0, 0.75, 0.20, 1.0]
                        } else {
                            [0.72, 0.35, 0.05, 1.0]
                        }
                    }
                    _ => {
                        if is_dark {
                            [0.70, 0.78, 0.90, 0.9]
                        } else {
                            [0.28, 0.33, 0.41, 1.0]
                        }
                    }
                };
                let w = self.measure_text(&ch_str, font_size_px);
                self.draw_text(curr_x, y, &ch_str, font_size_px, sym_color);
                curr_x += w;
                i += 1;
            }
        }
        curr_x
    }

    /// Renders a full multi-line code snippet with colorful syntax highlighting
    pub fn draw_syntax_code_block(
        &mut self,
        x: f32,
        mut y: f32,
        code: &str,
        font_size_px: f32,
        line_gap: f32,
        is_dark: bool,
    ) -> f32 {
        let line_h = font_size_px + line_gap;
        let start_y = y;
        for line in code.lines() {
            self.draw_syntax_code_line(x, y, line, font_size_px, is_dark);
            y += line_h;
        }
        y - start_y
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        unsafe {
            glViewport(0, 0, width, height);
        }
        log::info!("[TechScript EGL] Surface resized to {width}x{height}");
    }

    pub fn render_frame(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
    ) -> Result<(), String> {
        self.anim_time += 0.016;

        unsafe {
            // Auto-detect dynamic surface resizing on orientation rotate / fold / multi-window
            let mut cur_w: egl::EGLint = 0;
            let mut cur_h: egl::EGLint = 0;
            if egl::eglQuerySurface(self.display, self.surface, egl::EGL_WIDTH, &mut cur_w) != 0
                && egl::eglQuerySurface(self.display, self.surface, egl::EGL_HEIGHT, &mut cur_h)
                    != 0
            {
                if cur_w > 0 && cur_h > 0 && (cur_w != self.width || cur_h != self.height) {
                    self.resize(cur_w, cur_h);
                }
            }
        }

        // Auto-rotation physics (paused during direct user 3D drag touch)
        if runtime.auto_rotate && !runtime.is_touching_3d {
            runtime.rot_y += 0.016 * runtime.anim_speed.max(0.6);
        }

        // Advance live sequential command execution if active
        if runtime.active_exec_cmd.is_some() {
            runtime.exec_time += 0.016;
            runtime.update_command_execution();
        }

        // Smooth inertial scroll damping
        if runtime.scroll_velocity.abs() > 0.3 {
            runtime.scroll_y =
                (runtime.scroll_y + runtime.scroll_velocity).clamp(0.0, runtime.max_scroll_y);
            runtime.scroll_velocity *= 0.95;
            if runtime.scroll_y <= 0.0 || runtime.scroll_y >= runtime.max_scroll_y {
                runtime.scroll_velocity = 0.0;
            }
        } else {
            runtime.scroll_velocity = 0.0;
        }

        unsafe {
            // Dynamic 3D Mesh / Material / Custom Color / Wireframe Mode update
            let mesh_or_mat_changed = runtime.mesh_type != self.current_mesh
                || runtime.material != self.current_material
                || runtime.is_custom_color != self.current_is_custom
                || (runtime.is_custom_color
                    && (runtime.custom_hue - self.current_custom_hue).abs() > 0.005)
                || runtime.wireframe_mode != self.current_wireframe_mode;

            if mesh_or_mat_changed {
                let (mut v3d, i3d) = geometry3d::generate_mesh(runtime.mesh_type, runtime.material);
                if runtime.is_custom_color {
                    let custom_col = runtime.custom_color_rgba;
                    for v in &mut v3d {
                        if v.color[3] >= 0.90 {
                            v.color[0] = (custom_col[0] * (v.color[0] * 0.5 + 0.5)).clamp(0.0, 1.0);
                            v.color[1] = (custom_col[1] * (v.color[1] * 0.5 + 0.5)).clamp(0.0, 1.0);
                            v.color[2] = (custom_col[2] * (v.color[2] * 0.5 + 0.5)).clamp(0.0, 1.0);
                        }
                    }
                }
                if runtime.wireframe_mode {
                    // Semi-transparent X-ray geometry so inner geometric structure and back triangles are visible through the model
                    for v in &mut v3d {
                        v.color[3] = 0.25;
                    }
                }
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

                // Generate and upload geometric wireframe line indices from triangle mesh
                let mut wire_indices: Vec<u16> = Vec::with_capacity(i3d.len() * 2);
                for tri in i3d.chunks_exact(3) {
                    wire_indices.push(tri[0]);
                    wire_indices.push(tri[1]);
                    wire_indices.push(tri[1]);
                    wire_indices.push(tri[2]);
                    wire_indices.push(tri[2]);
                    wire_indices.push(tri[0]);
                }
                self.mesh3d_wire_count = wire_indices.len() as i32;

                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.mesh3d_wire_ibo);
                glBufferData(
                    GL_ELEMENT_ARRAY_BUFFER,
                    (wire_indices.len() * std::mem::size_of::<u16>()) as isize,
                    wire_indices.as_ptr() as *const std::ffi::c_void,
                    GL_STATIC_DRAW,
                );

                self.current_mesh = runtime.mesh_type;
                self.current_material = runtime.material;
                self.current_is_custom = runtime.is_custom_color;
                self.current_custom_hue = runtime.custom_hue;
                self.current_wireframe_mode = runtime.wireframe_mode;
            }

            // Clear Screen
            let [bg_r, bg_g, bg_b, bg_a] = Theme::to_f32_color(theme.bg);
            glClearColor(bg_r, bg_g, bg_b, bg_a);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            // Responsive density scale based on minimum dimension for optimal portrait & landscape scaling
            let min_dim = (self.width.min(self.height)) as f32;
            let dp_scale = (min_dim / 360.0).clamp(1.2, 2.5);
            let font_scale = dp_scale;
            let _pad = 14.0 * dp_scale;
            let _top_inset = 34.0 * dp_scale;

            let dock_h = 66.0 * dp_scale;
            let dock_margin = 12.0 * dp_scale;
            let dock_clearance = dock_h + dock_margin;

            // ─────────────────────────────────────────────────────────────────
            // Pass 1: 2D Liquid Glass UI
            // ─────────────────────────────────────────────────────────────────
            self.build_ui_batch(runtime, theme, dp_scale, font_scale, dock_clearance);
            self.draw_ui_batch();

            // ─────────────────────────────────────────────────────────────────
            // Pass 2: 3D Procedural Mesh (scissored viewport directly from card)
            // ─────────────────────────────────────────────────────────────────
            if let Some((vx, vy, vw, vh)) = self.viewport_3d_rect {
                if vy + vh > 0.0 && vy < self.height as f32 {
                    self.render_3d_viewport(runtime, theme, vx, vy, vw, vh);
                }
            }

            // Swap buffers to display frame
            if egl::eglSwapBuffers(self.display, self.surface) == 0 {
                return Err(format!("eglSwapBuffers failed: {}", egl::eglGetError()));
            }
        }

        runtime.frame_count += 1;
        Ok(())
    }

    unsafe fn render_3d_viewport(
        &self,
        runtime: &TechScriptRuntime,
        theme: &Theme,
        vp_x: f32,
        vp_y: f32,
        vp_w: f32,
        vp_h: f32,
    ) {
        let sh = self.height as f32;
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

        let aspect = vp_w / vp_h;
        let fov = 45.0f32.to_radians();
        let near = 0.1f32;
        let far = 100.0f32;
        let proj = geometry3d::perspective(fov, aspect, near, far);

        let eye = [0.0, 0.0, 3.2 / runtime.zoom];
        let target = [0.0, 0.0, 0.0];
        let up = [0.0, 1.0, 0.0];
        let view = geometry3d::look_at(eye, target, up);

        let float_tilt = if !runtime.is_touching_3d {
            (self.anim_time * 0.85).sin() * 0.06
        } else {
            0.0
        };
        let rot_x = geometry3d::rotate_x(runtime.rot_x + float_tilt);
        let rot_y = geometry3d::rotate_y(runtime.rot_y);
        let rot = geometry3d::mat4_mul(&rot_x, &rot_y);
        let trans = geometry3d::mat4_translate(runtime.model_pos_x, runtime.model_pos_y, 0.0);
        let model = geometry3d::mat4_mul(&trans, &rot);

        let vp = geometry3d::mat4_mul(&proj, &view);
        let mvp = geometry3d::mat4_mul(&vp, &model);

        let u_mvp = glGetUniformLocation(self.mesh3d_program, c_str(b"u_mvp\0"));
        let u_model = glGetUniformLocation(self.mesh3d_program, c_str(b"u_model\0"));
        let u_light_dir = glGetUniformLocation(self.mesh3d_program, c_str(b"u_light_dir\0"));
        let u_specular_col = glGetUniformLocation(self.mesh3d_program, c_str(b"u_specular_col\0"));
        let u_rim_col = glGetUniformLocation(self.mesh3d_program, c_str(b"u_rim_col\0"));
        let u_view_pos = glGetUniformLocation(self.mesh3d_program, c_str(b"u_view_pos\0"));

        glUniformMatrix4fv(u_mvp, 1, 0, mvp.as_ptr());
        glUniformMatrix4fv(u_model, 1, 0, model.as_ptr());
        glUniform3f(u_light_dir, 0.577, 0.577, 0.577);

        let spec_col = if runtime.is_custom_color {
            [
                (runtime.custom_color_rgba[0] * 1.2).min(1.0),
                (runtime.custom_color_rgba[1] * 1.2).min(1.0),
                (runtime.custom_color_rgba[2] * 1.2).min(1.0),
            ]
        } else {
            runtime.material.specular_color()
        };
        glUniform3f(u_specular_col, spec_col[0], spec_col[1], spec_col[2]);

        let rim_col = if runtime.is_custom_color {
            [
                runtime.custom_color_rgba[0],
                runtime.custom_color_rgba[1],
                runtime.custom_color_rgba[2],
            ]
        } else {
            runtime.material.rim_color()
        };
        glUniform3f(u_rim_col, rim_col[0], rim_col[1], rim_col[2]);

        glUniform3f(u_view_pos, eye[0], eye[1], eye[2]);

        glBindBuffer(GL_ARRAY_BUFFER, self.mesh3d_vbo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.mesh3d_ibo);

        let a_pos = glGetAttribLocation(self.mesh3d_program, c_str(b"a_pos\0")) as u32;
        let a_normal = glGetAttribLocation(self.mesh3d_program, c_str(b"a_normal\0")) as u32;
        let a_color = glGetAttribLocation(self.mesh3d_program, c_str(b"a_color\0")) as u32;

        let stride = std::mem::size_of::<geometry3d::Vertex3D>() as i32;

        glEnableVertexAttribArray(a_pos);
        glVertexAttribPointer(a_pos, 3, GL_FLOAT, 0, stride, std::ptr::null());

        glEnableVertexAttribArray(a_normal);
        glVertexAttribPointer(
            a_normal,
            3,
            GL_FLOAT,
            0,
            stride,
            12 as *const std::ffi::c_void,
        );

        glEnableVertexAttribArray(a_color);
        glVertexAttribPointer(
            a_color,
            4,
            GL_FLOAT,
            0,
            stride,
            24 as *const std::ffi::c_void,
        );

        let u_is_wireframe = glGetUniformLocation(self.mesh3d_program, c_str(b"u_is_wireframe\0"));
        let u_wire_color = glGetUniformLocation(self.mesh3d_program, c_str(b"u_wire_color\0"));

        if runtime.wireframe_mode {
            // In Geometric View, translucent faces allow seeing inside the geometry
            glDepthMask(0);
        } else {
            glDepthMask(1);
        }

        glUniform1f(u_is_wireframe, 0.0);
        glDrawElements(
            GL_TRIANGLES,
            self.mesh3d_index_count,
            GL_UNSIGNED_SHORT,
            std::ptr::null(),
        );

        if runtime.wireframe_mode {
            glDepthMask(1);
            glUniform1f(u_is_wireframe, 1.0);
            let wire_col = if runtime.is_custom_color {
                [
                    (runtime.custom_color_rgba[0] * 1.4).min(1.0),
                    (runtime.custom_color_rgba[1] * 1.4).min(1.0),
                    (runtime.custom_color_rgba[2] * 1.4).min(1.0),
                    1.0,
                ]
            } else if theme.is_dark {
                [0.0, 1.0, 0.88, 1.0]
            } else {
                [0.05, 0.35, 0.85, 1.0]
            };
            glUniform4f(
                u_wire_color,
                wire_col[0],
                wire_col[1],
                wire_col[2],
                wire_col[3],
            );
            glLineWidth(2.0);

            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.mesh3d_wire_ibo);
            glDrawElements(
                GL_LINES,
                self.mesh3d_wire_count,
                GL_UNSIGNED_SHORT,
                std::ptr::null(),
            );
        }

        glDepthMask(1);
        glDisable(GL_SCISSOR_TEST);
        glDisable(GL_DEPTH_TEST);
        glViewport(0, 0, self.width, self.height);
    }

    fn build_ui_batch(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        dock_clearance: f32,
    ) {
        self.ui_batch.clear();
        self.hit_regions.clear();
        self.viewport_3d_rect = None;

        let sw = self.width as f32;
        let sh = self.height as f32;
        let pad = 14.0 * dp_scale;
        let content_w = sw - pad * 2.0;
        let top_inset = 34.0 * dp_scale;

        // Subtle ambient spatial lighting glow matching modern mockup (dark mode only)
        if theme.is_dark {
            // Soft violet aurora glow at top center
            self.ui_batch.push_rect(
                sw * 0.25,
                -20.0 * dp_scale,
                sw * 0.55,
                220.0 * dp_scale,
                110.0 * dp_scale,
                [0.55, 0.28, 0.95, 0.12],
                0.0,
            );
            // Luminous cyan nebula glow at top right
            self.ui_batch.push_rect(
                sw * 0.55,
                30.0 * dp_scale,
                sw * 0.48,
                240.0 * dp_scale,
                120.0 * dp_scale,
                [0.00, 0.85, 1.00, 0.10],
                0.0,
            );
            // Mid-screen deep sapphire glow behind quick actions
            self.ui_batch.push_rect(
                pad,
                300.0 * dp_scale,
                sw * 0.60,
                220.0 * dp_scale,
                110.0 * dp_scale,
                [0.25, 0.35, 0.95, 0.06],
                0.0,
            );
        }

        // 1. Top Persistent Brand Header
        let header_y = top_inset;
        let header_h = 44.0 * dp_scale;

        let col_card = Theme::to_f32_color(theme.card);
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_accent = Theme::to_f32_color(theme.accent);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_surface = Theme::to_f32_color(theme.surface);

        // Official TechScript Brand Logo Mark (Authentic Dragon Swirl)
        self.ui_batch.draw_techscript_logo_mark(
            pad + 17.0 * dp_scale,
            header_y + header_h * 0.5,
            34.0 * dp_scale,
        );

        // Brand Title
        self.draw_text(
            pad + 44.0 * dp_scale,
            header_y + 4.0 * dp_scale,
            "TechScript",
            19.0 * font_scale,
            col_text,
        );
        self.draw_text(
            pad + 44.0 * dp_scale,
            header_y + 25.0 * dp_scale,
            "AppForge Native",
            11.5 * font_scale,
            if theme.is_dark {
                [0.65, 0.55, 0.98, 1.0]
            } else {
                col_primary
            },
        );

        // Header Action: Modern Dual Switch Theme Capsule
        let badge_h = 30.0 * dp_scale;
        let theme_btn_w = 72.0 * dp_scale;
        let theme_btn_x = sw - pad - theme_btn_w;
        let mode = runtime.theme_mode();
        self.ui_batch.push_rect(
            theme_btn_x,
            header_y + 7.0 * dp_scale,
            theme_btn_w,
            badge_h,
            15.0 * dp_scale,
            col_card,
            1.0 * dp_scale,
        );

        let icon_cy = header_y + 7.0 * dp_scale + badge_h * 0.5;
        if mode == "dark" {
            // Dark mode selected pill on left
            self.ui_batch.push_rect(
                theme_btn_x + 2.0 * dp_scale,
                header_y + 9.0 * dp_scale,
                34.0 * dp_scale,
                badge_h - 4.0 * dp_scale,
                13.0 * dp_scale,
                [0.12, 0.18, 0.38, 0.95],
                0.0,
            );
            self.ui_batch.draw_icon_moon(
                theme_btn_x + 18.0 * dp_scale,
                icon_cy,
                13.0 * dp_scale,
                [1.0, 1.0, 1.0, 1.0],
                col_surface,
            );
            self.ui_batch.draw_icon_sun(
                theme_btn_x + theme_btn_w - 18.0 * dp_scale,
                icon_cy,
                12.0 * dp_scale,
                [0.95, 0.70, 0.20, 0.35],
            );
        } else {
            // Light mode selected pill on right
            self.ui_batch.push_rect(
                theme_btn_x + theme_btn_w - 36.0 * dp_scale,
                header_y + 9.0 * dp_scale,
                34.0 * dp_scale,
                badge_h - 4.0 * dp_scale,
                13.0 * dp_scale,
                [0.90, 0.93, 0.98, 1.0],
                0.0,
            );
            self.ui_batch.draw_icon_moon(
                theme_btn_x + 18.0 * dp_scale,
                icon_cy,
                12.0 * dp_scale,
                col_muted,
                col_surface,
            );
            self.ui_batch.draw_icon_sun(
                theme_btn_x + theme_btn_w - 18.0 * dp_scale,
                icon_cy,
                13.0 * dp_scale,
                [0.95, 0.65, 0.10, 1.0],
            );
        }

        self.hit_regions.push(HitRegion {
            x: theme_btn_x,
            y: header_y + 7.0 * dp_scale,
            w: theme_btn_w,
            h: badge_h,
            action: Action::ToggleTheme,
        });

        // Header Action: Info Circular Glass Button (crisp outline with glowing center dot)
        let info_btn_w = 30.0 * dp_scale;
        let info_btn_x = theme_btn_x - 10.0 * dp_scale - info_btn_w;
        self.ui_batch.push_rect(
            info_btn_x,
            header_y + 7.0 * dp_scale,
            info_btn_w,
            badge_h,
            15.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.ui_batch.push_rect(
            info_btn_x + info_btn_w * 0.5 - 3.0 * dp_scale,
            icon_cy - 3.0 * dp_scale,
            6.0 * dp_scale,
            6.0 * dp_scale,
            3.0 * dp_scale,
            col_accent,
            0.0,
        );
        self.hit_regions.push(HitRegion {
            x: info_btn_x,
            y: header_y + 7.0 * dp_scale,
            w: info_btn_w,
            h: badge_h,
            action: Action::OpenAbout,
        });

        // 2. Screen Content Dispatch
        let content_y = header_y + header_h + 10.0 * dp_scale;
        let content_bottom = sh - dock_clearance;
        match runtime.active_screen() {
            "home" => self.render_home_content(
                runtime,
                theme,
                dp_scale,
                font_scale,
                pad,
                content_w,
                content_y,
                content_bottom,
            ),
            "explorer" => self.render_explorer_content(
                runtime,
                theme,
                dp_scale,
                font_scale,
                pad,
                content_w,
                content_y,
                content_bottom,
            ),
            "labs" => self.render_labs_content(
                runtime,
                theme,
                dp_scale,
                font_scale,
                pad,
                content_w,
                content_y,
                content_bottom,
            ),
            "projects" => self.render_projects_content(
                runtime,
                theme,
                dp_scale,
                font_scale,
                pad,
                content_w,
                content_y,
                content_bottom,
            ),
            "docs" => self.render_docs_content(
                runtime,
                theme,
                dp_scale,
                font_scale,
                pad,
                content_w,
                content_y,
                content_bottom,
            ),
            "about" => self.render_about_content(
                runtime,
                theme,
                dp_scale,
                font_scale,
                pad,
                content_w,
                content_y,
                content_bottom,
            ),
            _ => {}
        }

        // 3. Persistent Bottom Navigation Dock (5 Destinations)
        self.render_navigation_dock(runtime, theme, dp_scale, font_scale, dock_clearance);
    }

    /// Destination 1: Home Screen (Modernized Liquid Glass Dashboard - 100% Mockup Parity)
    fn render_home_content(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        pad: f32,
        cw: f32,
        start_y: f32,
        content_bottom: f32,
    ) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);
        let col_violet = Theme::to_f32_color(theme.violet);

        let sy = start_y - runtime.scroll_y;

        // 1. 3D Procedural Engine Showcase Card (Starts directly at top of content)
        let hero_y = sy;
        let hero_h = 246.0 * dp_scale;

        // Frosted Card Container with Specular Rim
        self.ui_batch.push_rect(
            pad,
            hero_y,
            cw,
            hero_h,
            18.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );

        // 3D Viewport bounds inside the card (between title and bottom pills)
        let vp_top = hero_y + 34.0 * dp_scale;
        let vp_h = hero_h - 74.0 * dp_scale;
        self.viewport_3d_rect = Some((pad, vp_top, cw, vp_h));

        // Ambient radial illumination halo behind the 3D crystal
        let center_y = vp_top + vp_h * 0.5;
        if theme.is_dark {
            self.ui_batch.push_rect(
                pad + cw * 0.5 - 85.0 * dp_scale,
                center_y - 85.0 * dp_scale,
                170.0 * dp_scale,
                170.0 * dp_scale,
                85.0 * dp_scale,
                [0.39, 0.40, 0.95, 0.12],
                0.0,
            );
        } else {
            // Ambient soft celestial sunlit glow behind model in light mode
            self.ui_batch.push_rect(
                pad + cw * 0.5 - 85.0 * dp_scale,
                center_y - 85.0 * dp_scale,
                170.0 * dp_scale,
                170.0 * dp_scale,
                85.0 * dp_scale,
                [0.78, 0.88, 1.0, 0.45],
                0.0,
            );
            // Crisp inner disc
            self.ui_batch.push_rect(
                pad + cw * 0.5 - 60.0 * dp_scale,
                center_y - 60.0 * dp_scale,
                120.0 * dp_scale,
                120.0 * dp_scale,
                60.0 * dp_scale,
                [0.85, 0.92, 1.0, 0.55],
                0.0,
            );
        }

        // Ambient luminous orbital ring centered exactly behind the 3D crystal
        // 3D Card Header: Dynamic Mesh Title on left, Material Badge on right
        let hero_title = format!("3D {}", runtime.mesh_type.name());
        self.draw_text(
            pad + 18.0 * dp_scale,
            hero_y + 12.0 * dp_scale,
            &hero_title,
            15.5 * font_scale,
            col_text,
        );
        let mat_badge = format!("[ {} ]", runtime.material.name());
        let mat_w = self.measure_text(&mat_badge, 12.5 * font_scale);
        self.draw_text(
            pad + cw - mat_w - 18.0 * dp_scale,
            hero_y + 12.0 * dp_scale,
            &mat_badge,
            12.5 * font_scale,
            col_accent,
        );

        // Ambient luminous orbital ring centered exactly behind the 3D model, illuminated with material rim color
        let rim_col = runtime.material.rim_color();
        self.ui_batch.push_ring(
            pad + cw * 0.5,
            center_y,
            64.0 * dp_scale,
            1.5 * dp_scale,
            [
                rim_col[0],
                rim_col[1],
                rim_col[2],
                if theme.is_dark { 0.40 } else { 0.25 },
            ],
        );

        // Interactive Material Chips with colored dot indicator matching reference
        let chip_y = hero_y + hero_h - 38.0 * dp_scale;
        let chip_w = (cw - 28.0 * dp_scale - 3.0 * 8.0 * dp_scale) / 4.0;
        let chip_h = 28.0 * dp_scale;
        let mats = [
            (
                MaterialPreset::TechScriptIndigo,
                "Indigo",
                [0.38, 0.45, 1.0, 1.0],
            ),
            (MaterialPreset::LiquidCyan, "Cyan", [0.00, 0.90, 1.00, 1.0]),
            (
                MaterialPreset::AmethystFire,
                "Amethyst",
                [0.72, 0.28, 0.98, 1.0],
            ),
            (
                MaterialPreset::EmeraldGem,
                "Emerald",
                [0.00, 0.90, 0.62, 1.0],
            ),
        ];

        for (i, (mat, lbl, dot_col)) in mats.iter().enumerate() {
            let cx = pad + 14.0 * dp_scale + (i as f32) * (chip_w + 8.0 * dp_scale);
            let is_sel = runtime.material == *mat;
            let chip_bg = if is_sel {
                if theme.is_dark {
                    [0.05, 0.10, 0.22, 0.98]
                } else {
                    [0.90, 0.94, 1.0, 0.95]
                }
            } else {
                if theme.is_dark {
                    [0.03, 0.06, 0.15, 0.85]
                } else {
                    [0.96, 0.97, 0.99, 0.85]
                }
            };
            self.ui_batch.push_rect(
                cx,
                chip_y,
                chip_w,
                chip_h,
                14.0 * dp_scale,
                chip_bg,
                if is_sel {
                    1.5 * dp_scale
                } else {
                    0.8 * dp_scale
                },
            );

            // Dot indicator on left
            let dot_x = cx + 11.0 * dp_scale;
            let dot_y = chip_y + chip_h * 0.5;
            self.ui_batch.push_rect(
                dot_x - 3.0 * dp_scale,
                dot_y - 3.0 * dp_scale,
                6.0 * dp_scale,
                6.0 * dp_scale,
                3.0 * dp_scale,
                *dot_col,
                0.0,
            );

            // Label text
            self.draw_text(
                cx + 20.0 * dp_scale,
                chip_y + 8.0 * dp_scale,
                lbl,
                10.5 * font_scale,
                if is_sel { col_text } else { col_muted },
            );

            // Generous touch target for flawless mobile response
            self.hit_regions.push(HitRegion {
                x: cx - 2.0 * dp_scale,
                y: chip_y - 4.0 * dp_scale,
                w: chip_w + 4.0 * dp_scale,
                h: chip_h + 8.0 * dp_scale,
                action: Action::Set3DMaterial(*mat),
            });
        }

        // Register Hero Touch Region for Dragging & Model Cycling
        self.hit_regions.push(HitRegion {
            x: pad,
            y: hero_y,
            w: cw,
            h: hero_h - 42.0 * dp_scale,
            action: Action::Cycle3DMesh,
        });

        // 2. Toolchain Quick Actions Section
        let qa_y = hero_y + hero_h + 16.0 * dp_scale;
        self.draw_text(
            pad,
            qa_y,
            "Toolchain Quick Actions",
            15.5 * font_scale,
            col_text,
        );

        let actions = [
            ("tsc run", "tsc run examples/01_hello.txs (VM)", 0),
            ("tsc app", "tsc app build --release (Mobile)", 1),
            ("tsc check", "tsc check src/main.txs (AST)", 2),
        ];

        let mut action_y = qa_y + 24.0 * dp_scale;
        let action_h = 58.0 * dp_scale;

        for (cmd_title, cmd_sub, icon_type) in actions {
            self.ui_batch.push_rect(
                pad,
                action_y,
                cw,
                action_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );

            // Leading rounded square badge
            let badge_sz = 38.0 * dp_scale;
            let badge_x = pad + 10.0 * dp_scale;
            let badge_y = action_y + (action_h - badge_sz) * 0.5;
            let badge_cx = badge_x + badge_sz * 0.5;
            let badge_cy = badge_y + badge_sz * 0.5;

            match icon_type {
                0 => {
                    let b_bg = if theme.is_dark {
                        [0.02, 0.08, 0.20, 0.95]
                    } else {
                        [0.84, 0.92, 1.0, 0.95]
                    };
                    self.ui_batch.push_rect(
                        badge_x,
                        badge_y,
                        badge_sz,
                        badge_sz,
                        10.0 * dp_scale,
                        b_bg,
                        1.0 * dp_scale,
                    );
                    self.ui_batch.draw_logo_tsc_run(
                        badge_cx,
                        badge_cy,
                        22.0 * dp_scale,
                        theme.is_dark,
                    );
                }
                1 => {
                    let b_bg = if theme.is_dark {
                        [0.08, 0.05, 0.22, 0.95]
                    } else {
                        [0.90, 0.86, 1.0, 0.95]
                    };
                    self.ui_batch.push_rect(
                        badge_x,
                        badge_y,
                        badge_sz,
                        badge_sz,
                        10.0 * dp_scale,
                        b_bg,
                        1.0 * dp_scale,
                    );
                    self.ui_batch.draw_logo_tsc_app(
                        badge_cx,
                        badge_cy,
                        24.0 * dp_scale,
                        theme.is_dark,
                    );
                }
                _ => {
                    let b_bg = if theme.is_dark {
                        [0.02, 0.12, 0.10, 0.95]
                    } else {
                        [0.84, 0.96, 0.90, 0.95]
                    };
                    self.ui_batch.push_rect(
                        badge_x,
                        badge_y,
                        badge_sz,
                        badge_sz,
                        10.0 * dp_scale,
                        b_bg,
                        1.0 * dp_scale,
                    );
                    self.ui_batch.draw_logo_tsc_check(
                        badge_cx,
                        badge_cy,
                        22.0 * dp_scale,
                        theme.is_dark,
                    );
                }
            }

            self.draw_text(
                pad + 58.0 * dp_scale,
                action_y + 11.0 * dp_scale,
                cmd_title,
                14.5 * font_scale,
                col_text,
            );
            self.draw_text(
                pad + 58.0 * dp_scale,
                action_y + 33.0 * dp_scale,
                cmd_sub,
                10.5 * font_scale,
                col_muted,
            );

            // Glowing Neon Cyan / Royal Sky Outline ▶ RUN Pill
            let run_w = 72.0 * dp_scale;
            let run_h = 30.0 * dp_scale;
            let run_x = pad + cw - run_w - 12.0 * dp_scale;
            let run_y = action_y + (action_h - run_h) * 0.5;
            let run_bg = if theme.is_dark {
                [0.04, 0.09, 0.20, 0.70]
            } else {
                [0.86, 0.93, 1.0, 0.95]
            };
            self.ui_batch.push_rect(
                run_x,
                run_y,
                run_w,
                run_h,
                15.0 * dp_scale,
                run_bg,
                1.4 * dp_scale,
            );
            self.ui_batch.draw_play_icon(
                run_x + 18.0 * dp_scale,
                run_y + run_h * 0.5,
                10.0 * dp_scale,
                col_accent,
            );
            self.draw_text(
                run_x + 32.0 * dp_scale,
                run_y + 8.0 * dp_scale,
                "RUN",
                11.5 * font_scale,
                col_accent,
            );

            self.hit_regions.push(HitRegion {
                x: pad,
                y: action_y,
                w: cw,
                h: action_h,
                action: Action::RunQuickAction(cmd_title.to_string()),
            });

            action_y += action_h + 12.0 * dp_scale;
        }

        // 3. Live TechScript VM Terminal Console Card (Long Authentic Multi-line Terminal)
        let console_y = action_y + 4.0 * dp_scale;
        let console_h = 120.0 * dp_scale;
        let term_bg = if theme.is_dark {
            [0.03, 0.05, 0.12, 0.98]
        } else {
            [0.90, 0.95, 0.99, 0.96]
        };
        self.ui_batch.push_rect(
            pad,
            console_y,
            cw,
            console_h,
            14.0 * dp_scale,
            term_bg,
            1.2 * dp_scale,
        );

        // Terminal Top Bar with macOS Window Dots
        self.ui_batch.push_rect(
            pad + 14.0 * dp_scale,
            console_y + 11.0 * dp_scale,
            7.0 * dp_scale,
            7.0 * dp_scale,
            3.5 * dp_scale,
            [0.94, 0.27, 0.27, 1.0],
            0.0,
        );
        self.ui_batch.push_rect(
            pad + 25.0 * dp_scale,
            console_y + 11.0 * dp_scale,
            7.0 * dp_scale,
            7.0 * dp_scale,
            3.5 * dp_scale,
            [0.96, 0.62, 0.04, 1.0],
            0.0,
        );
        self.ui_batch.push_rect(
            pad + 36.0 * dp_scale,
            console_y + 11.0 * dp_scale,
            7.0 * dp_scale,
            7.0 * dp_scale,
            3.5 * dp_scale,
            [0.06, 0.72, 0.50, 1.0],
            0.0,
        );
        let term_title_col = if theme.is_dark {
            [0.70, 0.75, 0.85, 0.9]
        } else {
            col_text
        };
        self.draw_text(
            pad + 54.0 * dp_scale,
            console_y + 7.5 * dp_scale,
            "TechScript VM Console • txs-core v2.0",
            11.0 * font_scale,
            term_title_col,
        );

        // Terminal Titlebar Separator
        let term_sep_col = if theme.is_dark {
            [0.15, 0.22, 0.45, 0.6]
        } else {
            [0.68, 0.78, 0.92, 0.50]
        };
        self.ui_batch.push_rect(
            pad,
            console_y + 24.0 * dp_scale,
            cw,
            1.0 * dp_scale,
            0.5 * dp_scale,
            term_sep_col,
            0.0,
        );

        // Terminal Multi-line Syntax-Highlighted Output
        let mut line_y = console_y + 30.0 * dp_scale;
        let line_spacing = 14.5 * dp_scale;
        for line in runtime.live_code_output.lines().take(6) {
            let line_col = if line.starts_with('$') {
                col_accent // Neon cyan in dark mode, sky azure in light mode
            } else if line.contains("STATUS: OK")
                || line.contains("BUILD SUCCESS")
                || line.contains("✓")
                || line.contains("Ready")
            {
                if theme.is_dark {
                    [0.0, 0.90, 0.62, 1.0]
                } else {
                    [0.02, 0.48, 0.35, 1.0]
                } // Bright or deep emerald
            } else if line.contains("Compiling")
                || line.contains("Parsing")
                || line.contains("Validating")
                || line.contains("Core")
            {
                if theme.is_dark {
                    [0.85, 0.90, 1.0, 0.95]
                } else {
                    col_text
                } // Crisp text
            } else {
                col_muted // Muted slate
            };
            self.draw_text(
                pad + 14.0 * dp_scale,
                line_y,
                line,
                10.5 * font_scale,
                line_col,
            );
            line_y += line_spacing;
        }

        // 4. Engine Metrics HUD Card
        let stats_y = console_y + console_h + 16.0 * dp_scale;
        let stats_h = 52.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            stats_y,
            cw,
            stats_h,
            14.0 * dp_scale,
            col_card,
            1.1 * dp_scale,
        );
        let col_w = cw / 4.0;
        let emerald_hud = if theme.is_dark {
            [0.0, 0.85, 0.55, 1.0]
        } else {
            [0.02, 0.48, 0.35, 1.0]
        };
        let metrics = [
            ("v2.0.0", "Compiler", col_violet),
            ("Rust NDK", "Runtime", col_accent),
            ("60 FPS", "EGL Canvas", col_primary),
            ("0% Java", "Codebase", emerald_hud),
        ];
        for (i, (val, lbl, color)) in metrics.iter().enumerate() {
            let cx = pad + (i as f32) * col_w;
            // Draw subtle vertical divider between columns
            if i > 0 {
                let div_col = if theme.is_dark {
                    [0.20, 0.28, 0.55, 0.40]
                } else {
                    [0.68, 0.78, 0.92, 0.50]
                };
                self.ui_batch.push_rect(
                    cx,
                    stats_y + 10.0 * dp_scale,
                    1.0 * dp_scale,
                    stats_h - 20.0 * dp_scale,
                    0.5 * dp_scale,
                    div_col,
                    0.0,
                );
            }
            let v_w = self.measure_text(val, 14.0 * font_scale);
            let l_w = self.measure_text(lbl, 10.5 * font_scale);
            self.draw_text(
                cx + (col_w - v_w) * 0.5,
                stats_y + 8.0 * dp_scale,
                val,
                14.0 * font_scale,
                *color,
            );
            self.draw_text(
                cx + (col_w - l_w) * 0.5,
                stats_y + 30.0 * dp_scale,
                lbl,
                10.5 * font_scale,
                col_muted,
            );
        }

        // 5. Navigation Shortcuts (Enlarged 2x2 Feature Grid)
        let grid_y = stats_y + stats_h + 16.0 * dp_scale;
        let cell_w = (cw - 14.0 * dp_scale) * 0.5;
        let cell_h = 76.0 * dp_scale;
        let icon_bg_sz = 46.0 * dp_scale;
        let icon_bg_y = grid_y + (cell_h - icon_bg_sz) * 0.5;

        // Row 1 - Cell 1: Explorer
        self.ui_batch.push_rect(
            pad,
            grid_y,
            cell_w,
            cell_h,
            16.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.ui_batch.push_rect(
            pad + 12.0 * dp_scale,
            icon_bg_y,
            icon_bg_sz,
            icon_bg_sz,
            icon_bg_sz * 0.5,
            [0.13, 0.83, 0.93, 0.18],
            1.0 * dp_scale,
        );
        self.ui_batch.draw_icon_compass(
            pad + 12.0 * dp_scale + icon_bg_sz * 0.5,
            icon_bg_y + icon_bg_sz * 0.5,
            22.0 * dp_scale,
            col_accent,
            col_primary,
        );
        self.draw_text(
            pad + 68.0 * dp_scale,
            grid_y + 16.0 * dp_scale,
            "Explorer",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad + 68.0 * dp_scale,
            grid_y + 42.0 * dp_scale,
            "(widgets)",
            11.5 * font_scale,
            col_muted,
        );
        self.hit_regions.push(HitRegion {
            x: pad,
            y: grid_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("explorer".into()),
        });

        // Row 1 - Cell 2: Labs
        let cell2_x = pad + cell_w + 14.0 * dp_scale;
        self.ui_batch.push_rect(
            cell2_x,
            grid_y,
            cell_w,
            cell_h,
            16.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.ui_batch.push_rect(
            cell2_x + 12.0 * dp_scale,
            icon_bg_y,
            icon_bg_sz,
            icon_bg_sz,
            icon_bg_sz * 0.5,
            [0.85, 0.27, 0.94, 0.18],
            1.0 * dp_scale,
        );
        self.ui_batch.draw_icon_labs(
            cell2_x + 12.0 * dp_scale + icon_bg_sz * 0.5,
            icon_bg_y + icon_bg_sz * 0.5,
            22.0 * dp_scale,
            [0.85, 0.27, 0.94, 1.0],
            col_accent,
        );
        self.draw_text(
            cell2_x + 68.0 * dp_scale,
            grid_y + 16.0 * dp_scale,
            "Labs",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            cell2_x + 68.0 * dp_scale,
            grid_y + 42.0 * dp_scale,
            "(motion)",
            11.5 * font_scale,
            col_muted,
        );
        self.hit_regions.push(HitRegion {
            x: cell2_x,
            y: grid_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("labs".into()),
        });

        // Row 2 - Cell 3: Projects
        let grid2_y = grid_y + cell_h + 14.0 * dp_scale;
        let icon_bg2_y = grid2_y + (cell_h - icon_bg_sz) * 0.5;
        self.ui_batch.push_rect(
            pad,
            grid2_y,
            cell_w,
            cell_h,
            16.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.ui_batch.push_rect(
            pad + 12.0 * dp_scale,
            icon_bg2_y,
            icon_bg_sz,
            icon_bg_sz,
            icon_bg_sz * 0.5,
            [0.54, 0.36, 0.96, 0.18],
            1.0 * dp_scale,
        );
        self.ui_batch.draw_icon_projects(
            pad + 12.0 * dp_scale + icon_bg_sz * 0.5,
            icon_bg2_y + icon_bg_sz * 0.5,
            22.0 * dp_scale,
            col_primary,
            col_accent,
        );
        self.draw_text(
            pad + 68.0 * dp_scale,
            grid2_y + 16.0 * dp_scale,
            "Projects",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad + 68.0 * dp_scale,
            grid2_y + 42.0 * dp_scale,
            "(samples)",
            11.5 * font_scale,
            col_muted,
        );
        self.hit_regions.push(HitRegion {
            x: pad,
            y: grid2_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("projects".into()),
        });

        // Row 2 - Cell 4: Docs
        self.ui_batch.push_rect(
            cell2_x,
            grid2_y,
            cell_w,
            cell_h,
            16.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.ui_batch.push_rect(
            cell2_x + 12.0 * dp_scale,
            icon_bg2_y,
            icon_bg_sz,
            icon_bg_sz,
            icon_bg_sz * 0.5,
            [0.10, 0.85, 0.70, 0.18],
            1.0 * dp_scale,
        );
        self.ui_batch.draw_icon_docs(
            cell2_x + 12.0 * dp_scale + icon_bg_sz * 0.5,
            icon_bg2_y + icon_bg_sz * 0.5,
            22.0 * dp_scale,
            col_accent,
        );
        self.draw_text(
            cell2_x + 68.0 * dp_scale,
            grid2_y + 16.0 * dp_scale,
            "Docs",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            cell2_x + 68.0 * dp_scale,
            grid2_y + 42.0 * dp_scale,
            "(reference)",
            11.5 * font_scale,
            col_muted,
        );
        self.hit_regions.push(HitRegion {
            x: cell2_x,
            y: grid2_y,
            w: cell_w,
            h: cell_h,
            action: Action::Navigate("docs".into()),
        });

        // ─────────────────────────────────────────────────────────────────────
        // 6. Recent Toolchain Activity & Build Records Feed
        // ─────────────────────────────────────────────────────────────────────
        let feed_y = grid2_y + cell_h + 20.0 * dp_scale;
        self.draw_text(
            pad,
            feed_y,
            "Recent Toolchain Activity",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad,
            feed_y + 18.0 * dp_scale,
            "Live compilation logs and hardware execution records",
            10.5 * font_scale,
            col_muted,
        );

        let feed_items = [
            (
                "build_app_mobile.txs",
                "aarch64-linux-android • Target ARM64-v8a",
                "BUILD SUCCESS",
                if theme.is_dark {
                    [0.0, 0.85, 0.55, 1.0]
                } else {
                    [0.04, 0.52, 0.35, 1.0]
                }, // Emerald
                "1.48s • libts_android.so (14.7 MB) • 0% Java Bytecode",
                0, // Package icon
            ),
            (
                "liquid_particle_physics.txs",
                "EGL 1.4 Hardware Canvas • 2,400 Particles",
                "60 FPS VERIFIED",
                col_accent, // Cyan
                "Zero frame drops • SDF Glass Shader Pipeline",
                1, // Terminal icon
            ),
            (
                "reactive_state_store.txs",
                "Unit Tests & Language Invariants",
                "42/42 PASSED",
                col_violet, // Violet
                "0.04s execution • Canonical syntax verified",
                2, // Checkmark icon
            ),
        ];

        let mut item_y = feed_y + 36.0 * dp_scale;
        let item_h = 64.0 * dp_scale;

        for (title, target, badge_text, badge_col, meta, icon_type) in feed_items {
            self.ui_batch.push_rect(
                pad,
                item_y,
                cw,
                item_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );

            // Leading icon badge
            let b_sz = 38.0 * dp_scale;
            let b_x = pad + 12.0 * dp_scale;
            let b_y = item_y + (item_h - b_sz) * 0.5;
            let b_cx = b_x + b_sz * 0.5;
            let b_cy = b_y + b_sz * 0.5;

            match icon_type {
                0 => {
                    let b_bg = if theme.is_dark {
                        [0.03, 0.12, 0.10, 0.95]
                    } else {
                        [0.90, 0.98, 0.94, 0.95]
                    };
                    let b_fg = if theme.is_dark {
                        [0.0, 0.85, 0.55, 1.0]
                    } else {
                        [0.04, 0.52, 0.35, 1.0]
                    };
                    self.ui_batch.push_rect(
                        b_x,
                        b_y,
                        b_sz,
                        b_sz,
                        10.0 * dp_scale,
                        b_bg,
                        1.0 * dp_scale,
                    );
                    self.ui_batch
                        .draw_icon_package(b_cx, b_cy, 18.0 * dp_scale, b_fg);
                }
                1 => {
                    let b_bg = if theme.is_dark {
                        [0.03, 0.08, 0.18, 0.95]
                    } else {
                        [0.92, 0.97, 1.0, 0.95]
                    };
                    self.ui_batch.push_rect(
                        b_x,
                        b_y,
                        b_sz,
                        b_sz,
                        10.0 * dp_scale,
                        b_bg,
                        1.0 * dp_scale,
                    );
                    self.ui_batch
                        .draw_icon_terminal(b_cx, b_cy, 18.0 * dp_scale, col_accent);
                }
                _ => {
                    let b_bg = if theme.is_dark {
                        [0.08, 0.05, 0.18, 0.95]
                    } else {
                        [0.95, 0.92, 1.0, 0.95]
                    };
                    self.ui_batch.push_rect(
                        b_x,
                        b_y,
                        b_sz,
                        b_sz,
                        10.0 * dp_scale,
                        b_bg,
                        1.0 * dp_scale,
                    );
                    self.ui_batch
                        .draw_icon_check(b_cx, b_cy, 18.0 * dp_scale, col_violet);
                }
            }

            // Title & target
            self.draw_text(
                pad + 58.0 * dp_scale,
                item_y + 11.0 * dp_scale,
                title,
                13.5 * font_scale,
                col_text,
            );
            self.draw_text(
                pad + 58.0 * dp_scale,
                item_y + 28.0 * dp_scale,
                target,
                10.0 * font_scale,
                col_muted,
            );
            self.draw_text(
                pad + 58.0 * dp_scale,
                item_y + 45.0 * dp_scale,
                meta,
                9.5 * font_scale,
                col_muted,
            );

            // Status badge pill on right
            let badge_w = self.measure_text(badge_text, 9.5 * font_scale) + 16.0 * dp_scale;
            let badge_h = 22.0 * dp_scale;
            let badge_x = pad + cw - badge_w - 12.0 * dp_scale;
            let badge_y = item_y + 12.0 * dp_scale;
            let p_bg = if theme.is_dark {
                [
                    badge_col[0] * 0.15,
                    badge_col[1] * 0.15,
                    badge_col[2] * 0.15,
                    0.9,
                ]
            } else {
                [
                    badge_col[0] * 0.15 + 0.83,
                    badge_col[1] * 0.15 + 0.83,
                    badge_col[2] * 0.15 + 0.83,
                    0.95,
                ]
            };
            self.ui_batch.push_rect(
                badge_x,
                badge_y,
                badge_w,
                badge_h,
                11.0 * dp_scale,
                p_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                badge_x + 8.0 * dp_scale,
                badge_y + 6.0 * dp_scale,
                badge_text,
                9.5 * font_scale,
                badge_col,
            );

            self.hit_regions.push(HitRegion {
                x: pad,
                y: item_y,
                w: cw,
                h: item_h,
                action: Action::RunQuickAction("tsc check".into()),
            });

            item_y += item_h + 10.0 * dp_scale;
        }

        // ─────────────────────────────────────────────────────────────────────
        // 7. TechScript Forge Ecosystem
        // ─────────────────────────────────────────────────────────────────────
        let eco_y = item_y + 12.0 * dp_scale;
        self.draw_text(
            pad,
            eco_y,
            "TechScript Forge Ecosystem",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad,
            eco_y + 18.0 * dp_scale,
            "Hardware backends, graphics pipelines, and compilation targets",
            10.5 * font_scale,
            col_muted,
        );

        let eco_cards = [
            (
                "AppForge Native Mobile",
                "ACTIVE ENGINE",
                if theme.is_dark { [0.0, 0.85, 0.55, 1.0] } else { [0.04, 0.52, 0.35, 1.0] },
                "Direct Android OS NativeActivity entry via Rust NDK. Renders 60 FPS hardware EGL liquid glass UI without Java/Kotlin bytecode.",
                col_accent,
                Action::Navigate("explorer".into()),
            ),
            (
                "WebForge Wasm Engine",
                "WASM READY",
                col_primary,
                "Compiles canonical TechScript directly to high-throughput WebAssembly with direct browser DOM and Canvas bindings.",
                col_primary,
                Action::Navigate("projects".into()),
            ),
            (
                "GameForge 3D Graphics",
                "GLES 2.0 / 3.0",
                col_violet,
                "Procedural 3D geometry engine with Blinn-Phong specular shading, Fresnel refraction, and camera physics.",
                col_violet,
                Action::Navigate("labs".into()),
            ),
        ];

        let mut card_y = eco_y + 36.0 * dp_scale;
        let c_card_h = 80.0 * dp_scale;

        for (c_title, c_badge, c_badge_col, c_desc, icon_col, c_action) in eco_cards {
            self.ui_batch.push_rect(
                pad,
                card_y,
                cw,
                c_card_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );

            // Leading colored dot indicator
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                card_y + 14.0 * dp_scale,
                8.0 * dp_scale,
                8.0 * dp_scale,
                4.0 * dp_scale,
                icon_col,
                0.0,
            );

            // Title
            self.draw_text(
                pad + 28.0 * dp_scale,
                card_y + 10.0 * dp_scale,
                c_title,
                13.5 * font_scale,
                col_text,
            );

            // Badge
            let b_w = self.measure_text(c_badge, 9.0 * font_scale) + 14.0 * dp_scale;
            let b_h = 20.0 * dp_scale;
            let b_x = pad + cw - b_w - 12.0 * dp_scale;
            let b_y = card_y + 10.0 * dp_scale;
            let b_bg = if theme.is_dark {
                [
                    c_badge_col[0] * 0.15,
                    c_badge_col[1] * 0.15,
                    c_badge_col[2] * 0.15,
                    0.9,
                ]
            } else {
                [
                    c_badge_col[0] * 0.15 + 0.83,
                    c_badge_col[1] * 0.15 + 0.83,
                    c_badge_col[2] * 0.15 + 0.83,
                    0.95,
                ]
            };
            self.ui_batch
                .push_rect(b_x, b_y, b_w, b_h, 10.0 * dp_scale, b_bg, 1.0 * dp_scale);
            self.draw_text(
                b_x + 7.0 * dp_scale,
                b_y + 5.5 * dp_scale,
                c_badge,
                9.0 * font_scale,
                c_badge_col,
            );

            // Wrapped description (prevents text overflow off the screen edge)
            self.draw_text_wrapped(
                pad + 14.0 * dp_scale,
                card_y + 32.0 * dp_scale,
                cw - 28.0 * dp_scale,
                c_desc,
                9.0 * font_scale,
                col_muted,
                3.5 * dp_scale,
            );

            self.hit_regions.push(HitRegion {
                x: pad,
                y: card_y,
                w: cw,
                h: c_card_h,
                action: c_action,
            });

            card_y += c_card_h + 10.0 * dp_scale;
        }

        // ─────────────────────────────────────────────────────────────────────
        // 8. Hardware Telemetry & Invariants HUD
        // ─────────────────────────────────────────────────────────────────────
        let hud_y = card_y + 12.0 * dp_scale;
        self.draw_text(
            pad,
            hud_y,
            "Hardware Telemetry & Invariants",
            15.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad,
            hud_y + 18.0 * dp_scale,
            "Real-time native execution environment diagnostics",
            10.5 * font_scale,
            col_muted,
        );

        let telem_card_y = hud_y + 36.0 * dp_scale;
        let telem_card_h = 86.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            telem_card_y,
            cw,
            telem_card_h,
            14.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );

        let t_half_w = cw * 0.5;
        let t_row_h = telem_card_h * 0.5;

        // Cell 1: Target ABI
        self.draw_text(
            pad + 14.0 * dp_scale,
            telem_card_y + 10.0 * dp_scale,
            "Target ABI",
            10.0 * font_scale,
            col_muted,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            telem_card_y + 25.0 * dp_scale,
            "aarch64-linux-android",
            12.0 * font_scale,
            col_accent,
        );

        // Cell 2: GC Invariant
        self.draw_text(
            pad + t_half_w + 14.0 * dp_scale,
            telem_card_y + 10.0 * dp_scale,
            "GC Overhead",
            10.0 * font_scale,
            col_muted,
        );
        let gc_col = if theme.is_dark {
            [0.0, 0.85, 0.55, 1.0]
        } else {
            [0.04, 0.52, 0.35, 1.0]
        };
        self.draw_text(
            pad + t_half_w + 14.0 * dp_scale,
            telem_card_y + 25.0 * dp_scale,
            "0.0 ms (Zero GC Engine)",
            12.0 * font_scale,
            gc_col,
        );

        // Divider
        let div_col = if theme.is_dark {
            [0.15, 0.22, 0.45, 0.4]
        } else {
            [0.68, 0.78, 0.92, 0.50]
        };
        self.ui_batch.push_rect(
            pad + 10.0 * dp_scale,
            telem_card_y + t_row_h,
            cw - 20.0 * dp_scale,
            1.0 * dp_scale,
            0.5 * dp_scale,
            div_col,
            0.0,
        );

        // Cell 3: Native Threads
        self.draw_text(
            pad + 14.0 * dp_scale,
            telem_card_y + t_row_h + 8.0 * dp_scale,
            "Execution Model",
            10.0 * font_scale,
            col_muted,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            telem_card_y + t_row_h + 23.0 * dp_scale,
            "Native OS Threads (POSIX)",
            12.0 * font_scale,
            col_text,
        );

        // Cell 4: Memory Invariant
        self.draw_text(
            pad + t_half_w + 14.0 * dp_scale,
            telem_card_y + t_row_h + 8.0 * dp_scale,
            "Memory Footprint",
            10.0 * font_scale,
            col_muted,
        );
        self.draw_text(
            pad + t_half_w + 14.0 * dp_scale,
            telem_card_y + t_row_h + 23.0 * dp_scale,
            "42.8 MB (Low Overhead)",
            12.0 * font_scale,
            col_violet,
        );

        // ─────────────────────────────────────────────────────────────────────
        // 9. Community & Official Documentation
        // ─────────────────────────────────────────────────────────────────────
        let links_y = telem_card_y + telem_card_h + 18.0 * dp_scale;
        self.draw_text(
            pad,
            links_y,
            "Community & Resources",
            15.5 * font_scale,
            col_text,
        );

        let link_items = [
            (
                "Official Language Specification",
                "Locked canonical syntax, master spec, standard library",
                Action::Navigate("docs".into()),
            ),
            (
                "GitHub Source Repository",
                "github.com/Tcode-Motion/techscript",
                Action::OpenLink("https://github.com/Tcode-Motion/techscript".into()),
            ),
            (
                "Creator & Lead Architect",
                "Tanmoy (@Tcode-Motion) • Founder",
                Action::OpenLink("https://github.com/Tcode-Motion".into()),
            ),
            (
                "Official Documentation Web",
                "techscript.is-a.dev/techscript",
                Action::OpenLink("https://techscript.is-a.dev/techscript/".into()),
            ),
        ];

        let mut l_y = links_y + 24.0 * dp_scale;
        let l_h = 52.0 * dp_scale;

        for (l_title, l_sub, l_act) in link_items {
            self.ui_batch
                .push_rect(pad, l_y, cw, l_h, 14.0 * dp_scale, col_card, 1.2 * dp_scale);
            self.draw_text(
                pad + 14.0 * dp_scale,
                l_y + 10.0 * dp_scale,
                l_title,
                13.0 * font_scale,
                col_text,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                l_y + 28.0 * dp_scale,
                l_sub,
                10.0 * font_scale,
                col_muted,
            );

            // Trailing arrow badge
            self.draw_text(
                pad + cw - 26.0 * dp_scale,
                l_y + 16.0 * dp_scale,
                "↗",
                15.0 * font_scale,
                col_accent,
            );

            self.hit_regions.push(HitRegion {
                x: pad,
                y: l_y,
                w: cw,
                h: l_h,
                action: l_act,
            });

            l_y += l_h + 10.0 * dp_scale;
        }

        // ─────────────────────────────────────────────────────────────────────
        // 10. Footer
        // ─────────────────────────────────────────────────────────────────────
        let foot_y = l_y + 14.0 * dp_scale;
        let foot_text1 = "TechScript AppForge v2.0.0 • Pure Rust NativeActivity Engine";
        let foot_w1 = self.measure_text(foot_text1, 9.5 * font_scale);
        self.draw_text(
            pad + (cw - foot_w1) * 0.5,
            foot_y,
            foot_text1,
            9.5 * font_scale,
            col_muted,
        );

        let foot_text2 = "Syntax is Locked to Master Spec • Standard Edition";
        let foot_w2 = self.measure_text(foot_text2, 9.0 * font_scale);
        self.draw_text(
            pad + (cw - foot_w2) * 0.5,
            foot_y + 16.0 * dp_scale,
            foot_text2,
            9.0 * font_scale,
            [col_muted[0], col_muted[1], col_muted[2], 0.5],
        );

        let total_content_h = foot_y + 44.0 * dp_scale - sy;
        runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
    }

    /// Destination 2: Explorer Screen
    fn render_explorer_content(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        pad: f32,
        cw: f32,
        start_y: f32,
        content_bottom: f32,
    ) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);
        let col_violet = Theme::to_f32_color(theme.violet);

        let sy = start_y - runtime.scroll_y;

        // Title & Subtitle at top of Explorer
        self.draw_text(
            pad,
            sy,
            "TechScript Explorer",
            17.0 * font_scale,
            col_primary,
        );
        self.draw_text_wrapped(
            pad,
            sy + 20.0 * dp_scale,
            cw,
            "Interactive gallery of verified native capabilities & reactive state",
            10.5 * font_scale,
            col_muted,
            3.0 * dp_scale,
        );

        // Category Filter Pills
        let cat_y = sy + 40.0 * dp_scale;
        let cat_h = 28.0 * dp_scale;
        let cats = ["All", "Buttons", "Controls", "Surfaces", "State"];
        let pill_w = (cw - 4.0 * 6.0 * dp_scale) / 5.0;

        for (i, cat) in cats.iter().enumerate() {
            let px = pad + (i as f32) * (pill_w + 6.0 * dp_scale);
            let is_sel = runtime.explorer_category == i;
            let bg = if is_sel { col_primary } else { col_card };
            let fg = if is_sel {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                col_muted
            };
            self.ui_batch.push_rect(
                px,
                cat_y,
                pill_w,
                cat_h,
                14.0 * dp_scale,
                bg,
                if is_sel { 1.5 } else { 1.0 },
            );
            let t_w = self.measure_text(cat, 10.0 * font_scale);
            self.draw_text(
                px + (pill_w - t_w) * 0.5,
                cat_y + 8.0 * dp_scale,
                cat,
                10.0 * font_scale,
                fg,
            );
            self.hit_regions.push(HitRegion {
                x: px,
                y: cat_y,
                w: pill_w,
                h: cat_h,
                action: Action::SelectExplorerCategory(i),
            });
        }

        // Content cards flow below the category bar
        let mut cur_y = cat_y + cat_h + 10.0 * dp_scale;
        let code_bg = if theme.is_dark {
            [0.03, 0.05, 0.12, 0.98]
        } else {
            [0.91, 0.95, 0.99, 0.96]
        };
        let sel_cat = runtime.explorer_category;

        // ═════════════════════════════════════════════════════════════════════
        // SECTION 1: BUTTONS & INTERACTIVE STATES (Shown in 'All' and 'Buttons')
        // ═════════════════════════════════════════════════════════════════════
        if sel_cat == 0 || sel_cat == 1 {
            // Card 1: Interactive Button Styles & States
            let c1_h = 162.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                c1_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Interactive Button Styles & States",
                13.0 * font_scale,
                col_text,
            );

            // Large Glowing Primary Button (Shows tap counter)
            let p_btn_y = cur_y + 32.0 * dp_scale;
            let p_btn_h = 36.0 * dp_scale;
            let p_btn_w = cw - 28.0 * dp_scale;
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                p_btn_y,
                p_btn_w,
                p_btn_h,
                10.0 * dp_scale,
                col_primary,
                1.0 * dp_scale,
            );
            let p_lbl = format!("Primary Glow Button  [ Taps: {} ]", runtime.demo_counter);
            let p_w = self.measure_text(&p_lbl, 11.5 * font_scale);
            self.draw_text(
                pad + 14.0 * dp_scale + (p_btn_w - p_w) * 0.5,
                p_btn_y + 10.0 * dp_scale,
                &p_lbl,
                11.5 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: pad + 14.0 * dp_scale,
                y: p_btn_y,
                w: p_btn_w,
                h: p_btn_h,
                action: Action::IncrementCounter,
            });

            // Secondary Button Row (-1 Dec, +1 Inc, Reset)
            let r2_y = p_btn_y + p_btn_h + 8.0 * dp_scale;
            let r2_h = 32.0 * dp_scale;
            let sub_w = (cw - 28.0 * dp_scale - 16.0 * dp_scale) / 3.0;

            // -1 Decrement
            let b1_x = pad + 14.0 * dp_scale;
            let b1_bg = if theme.is_dark {
                [0.39, 0.40, 0.95, 0.35]
            } else {
                [0.86, 0.92, 1.0, 0.95]
            };
            let b1_fg = if theme.is_dark {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                col_primary
            };
            self.ui_batch.push_rect(
                b1_x,
                r2_y,
                sub_w,
                r2_h,
                8.0 * dp_scale,
                b1_bg,
                1.0 * dp_scale,
            );
            let b1_txt = "- 1 Dec";
            let b1_w = self.measure_text(b1_txt, 10.5 * font_scale);
            self.draw_text(
                b1_x + (sub_w - b1_w) * 0.5,
                r2_y + 9.0 * dp_scale,
                b1_txt,
                10.5 * font_scale,
                b1_fg,
            );
            self.hit_regions.push(HitRegion {
                x: b1_x,
                y: r2_y,
                w: sub_w,
                h: r2_h,
                action: Action::DecrementCounter,
            });

            // +1 Increment
            let b2_x = b1_x + sub_w + 8.0 * dp_scale;
            let b2_bg = if theme.is_dark {
                [0.00, 0.85, 0.55, 0.35]
            } else {
                [0.84, 0.96, 0.90, 0.95]
            };
            let b2_fg = if theme.is_dark {
                [0.00, 0.95, 0.65, 1.0]
            } else {
                [0.02, 0.48, 0.35, 1.0]
            };
            self.ui_batch.push_rect(
                b2_x,
                r2_y,
                sub_w,
                r2_h,
                8.0 * dp_scale,
                b2_bg,
                1.0 * dp_scale,
            );
            let b2_txt = "+ 1 Inc";
            let b2_w = self.measure_text(b2_txt, 10.5 * font_scale);
            self.draw_text(
                b2_x + (sub_w - b2_w) * 0.5,
                r2_y + 9.0 * dp_scale,
                b2_txt,
                10.5 * font_scale,
                b2_fg,
            );
            self.hit_regions.push(HitRegion {
                x: b2_x,
                y: r2_y,
                w: sub_w,
                h: r2_h,
                action: Action::IncrementCounter,
            });

            // Reset Count
            let b3_x = b2_x + sub_w + 8.0 * dp_scale;
            let b3_bg = if theme.is_dark {
                [0.95, 0.25, 0.38, 0.35]
            } else {
                [0.98, 0.88, 0.90, 0.95]
            };
            let b3_fg = if theme.is_dark {
                [1.0, 0.55, 0.65, 1.0]
            } else {
                [0.75, 0.12, 0.25, 1.0]
            };
            self.ui_batch.push_rect(
                b3_x,
                r2_y,
                sub_w,
                r2_h,
                8.0 * dp_scale,
                b3_bg,
                1.0 * dp_scale,
            );
            let b3_txt = "Reset";
            let b3_w = self.measure_text(b3_txt, 10.5 * font_scale);
            self.draw_text(
                b3_x + (sub_w - b3_w) * 0.5,
                r2_y + 9.0 * dp_scale,
                b3_txt,
                10.5 * font_scale,
                b3_fg,
            );
            self.hit_regions.push(HitRegion {
                x: b3_x,
                y: r2_y,
                w: sub_w,
                h: r2_h,
                action: Action::ResetCounter,
            });

            // Translucent Glass & Cyan Outline Pills Row
            let r3_y = r2_y + r2_h + 8.0 * dp_scale;
            let r3_h = 28.0 * dp_scale;
            let half_w = (cw - 28.0 * dp_scale - 8.0 * dp_scale) * 0.5;

            // Liquid Glass Translucent
            let g_bg = if theme.is_dark {
                [0.18, 0.22, 0.35, 0.65]
            } else {
                [0.86, 0.93, 0.99, 0.85]
            };
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                r3_y,
                half_w,
                r3_h,
                14.0 * dp_scale,
                g_bg,
                1.0 * dp_scale,
            );
            let g_lbl = "Translucent Glass";
            let g_w = self.measure_text(g_lbl, 9.5 * font_scale);
            self.draw_text(
                pad + 14.0 * dp_scale + (half_w - g_w) * 0.5,
                r3_y + 7.5 * dp_scale,
                g_lbl,
                9.5 * font_scale,
                col_accent,
            );

            // Cyan Outline Rim
            let c_btn_x = pad + 14.0 * dp_scale + half_w + 8.0 * dp_scale;
            let co_bg = if theme.is_dark {
                [0.00, 0.90, 1.00, 0.15]
            } else {
                [0.84, 0.92, 1.0, 0.85]
            };
            self.ui_batch.push_rect(
                c_btn_x,
                r3_y,
                half_w,
                r3_h,
                14.0 * dp_scale,
                co_bg,
                1.2 * dp_scale,
            );
            let co_lbl = "Cyan Outline Rim";
            let co_w = self.measure_text(co_lbl, 9.5 * font_scale);
            self.draw_text(
                c_btn_x + (half_w - co_w) * 0.5,
                r3_y + 7.5 * dp_scale,
                co_lbl,
                9.5 * font_scale,
                col_accent,
            );

            cur_y += c1_h + 12.0 * dp_scale;

            // Card 1b: Colorful Syntax-Highlighted Code (Buttons Component)
            let cd1_h = 144.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                cd1_h,
                14.0 * dp_scale,
                code_bg,
                1.2 * dp_scale,
            );

            // macOS Window Header
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.94, 0.27, 0.27, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 25.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.96, 0.62, 0.04, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 36.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.06, 0.72, 0.50, 1.0],
                0.0,
            );
            let c_fn_col = if theme.is_dark {
                [0.85, 0.90, 1.0, 0.9]
            } else {
                col_text
            };
            self.draw_text(
                pad + 54.0 * dp_scale,
                cur_y + 7.5 * dp_scale,
                "01_buttons_demo.txs",
                11.0 * font_scale,
                c_fn_col,
            );

            let badge_b = "[ REACTIVE DISPATCH ]";
            let bb_w = self.measure_text(badge_b, 9.0 * font_scale);
            self.draw_text(
                pad + cw - bb_w - 14.0 * dp_scale,
                cur_y + 8.0 * dp_scale,
                badge_b,
                9.0 * font_scale,
                col_accent,
            );

            let c_sep_col = if theme.is_dark {
                [0.15, 0.22, 0.45, 0.6]
            } else {
                [0.68, 0.78, 0.92, 0.50]
            };
            self.ui_batch.push_rect(
                pad,
                cur_y + 24.0 * dp_scale,
                cw,
                1.0 * dp_scale,
                0.5 * dp_scale,
                c_sep_col,
                0.0,
            );

            let snippet_btn = [
                "# Declarative button handlers",
                "counter is 0",
                "",
                "to on_primary_tap():",
                "    counter becomes counter + 1",
                "    say \"Taps: {counter}\"",
            ]
            .join("\n");
            self.draw_syntax_code_block(
                pad + 14.0 * dp_scale,
                cur_y + 32.0 * dp_scale,
                &snippet_btn,
                9.5 * font_scale,
                4.5 * dp_scale,
                theme.is_dark,
            );

            // Run Button
            let r_btn_w = 98.0 * dp_scale;
            let r_btn_h = 24.0 * dp_scale;
            let r_btn_x = pad + cw - r_btn_w - 12.0 * dp_scale;
            let r_btn_y = cur_y + cd1_h - 32.0 * dp_scale;
            let r_btn_bg = if theme.is_dark {
                [0.12, 0.83, 0.93, 0.18]
            } else {
                [0.84, 0.92, 1.0, 0.95]
            };
            self.ui_batch.push_rect(
                r_btn_x,
                r_btn_y,
                r_btn_w,
                r_btn_h,
                12.0 * dp_scale,
                r_btn_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                r_btn_x + 8.0 * dp_scale,
                r_btn_y + 6.0 * dp_scale,
                "▶ RUN ACTION",
                9.0 * font_scale,
                col_accent,
            );
            self.hit_regions.push(HitRegion {
                x: r_btn_x,
                y: r_btn_y,
                w: r_btn_w,
                h: r_btn_h,
                action: Action::RunQuickAction("tsc run".into()),
            });

            cur_y += cd1_h + 12.0 * dp_scale;
        }

        // ═════════════════════════════════════════════════════════════════════
        // SECTION 2: CONTROLS & SLIDERS (Shown in 'All' and 'Controls')
        // ═════════════════════════════════════════════════════════════════════
        if sel_cat == 0 || sel_cat == 2 {
            let c2_h = 168.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                c2_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Hardware Controls & Precision Sliders",
                13.0 * font_scale,
                col_text,
            );

            // Switch 1: Hardware Acceleration
            let sw1_y = cur_y + 32.0 * dp_scale;
            self.draw_text(
                pad + 14.0 * dp_scale,
                sw1_y + 4.0 * dp_scale,
                "Hardware Acceleration:",
                11.0 * font_scale,
                col_text,
            );
            let sw_w = 46.0 * dp_scale;
            let sw_h = 22.0 * dp_scale;
            let sw_x = pad + cw - sw_w - 14.0 * dp_scale;
            let sw1_bg = if runtime.demo_toggle {
                col_primary
            } else {
                [0.25, 0.28, 0.38, 0.7]
            };
            self.ui_batch
                .push_rect(sw_x, sw1_y, sw_w, sw_h, 11.0 * dp_scale, sw1_bg, 1.0);
            let knob1_x = if runtime.demo_toggle {
                sw_x + sw_w - 20.0 * dp_scale
            } else {
                sw_x + 2.0 * dp_scale
            };
            self.ui_batch.push_rect(
                knob1_x,
                sw1_y + 2.0 * dp_scale,
                18.0 * dp_scale,
                18.0 * dp_scale,
                9.0 * dp_scale,
                [1.0, 1.0, 1.0, 1.0],
                0.0,
            );
            self.hit_regions.push(HitRegion {
                x: sw_x,
                y: sw1_y,
                w: sw_w,
                h: sw_h,
                action: Action::ToggleSwitch,
            });

            // Switch 2: 3D Auto-Rotation
            let sw2_y = sw1_y + 30.0 * dp_scale;
            self.draw_text(
                pad + 14.0 * dp_scale,
                sw2_y + 4.0 * dp_scale,
                "3D Auto-Rotation Physics:",
                11.0 * font_scale,
                col_text,
            );
            let sw2_bg = if runtime.auto_rotate {
                [0.00, 0.85, 0.55, 0.90]
            } else {
                [0.25, 0.28, 0.38, 0.7]
            };
            self.ui_batch
                .push_rect(sw_x, sw2_y, sw_w, sw_h, 11.0 * dp_scale, sw2_bg, 1.0);
            let knob2_x = if runtime.auto_rotate {
                sw_x + sw_w - 20.0 * dp_scale
            } else {
                sw_x + 2.0 * dp_scale
            };
            self.ui_batch.push_rect(
                knob2_x,
                sw2_y + 2.0 * dp_scale,
                18.0 * dp_scale,
                18.0 * dp_scale,
                9.0 * dp_scale,
                [1.0, 1.0, 1.0, 1.0],
                0.0,
            );
            self.hit_regions.push(HitRegion {
                x: sw_x,
                y: sw2_y,
                w: sw_w,
                h: sw_h,
                action: Action::ToggleAutoRotate,
            });

            // Slider: Liquid Glass Refraction Intensity with Tap Presets
            let sl_y = sw2_y + 30.0 * dp_scale;
            let pct_val = (runtime.demo_slider * 100.0).round() as i32;
            let sl_lbl = format!("SDF Refraction Intensity: {}%", pct_val);
            self.draw_text(
                pad + 14.0 * dp_scale,
                sl_y,
                &sl_lbl,
                11.0 * font_scale,
                col_text,
            );

            let track_y = sl_y + 18.0 * dp_scale;
            let track_w = cw - 28.0 * dp_scale;
            let track_bg = if theme.is_dark {
                [0.20, 0.25, 0.35, 0.8]
            } else {
                [0.78, 0.86, 0.96, 0.70]
            };
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                track_y,
                track_w,
                6.0 * dp_scale,
                3.0 * dp_scale,
                track_bg,
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                track_y,
                track_w * runtime.demo_slider,
                6.0 * dp_scale,
                3.0 * dp_scale,
                col_accent,
                0.0,
            );

            // Preset Buttons Row [ 35% ] [ 65% ] [ 95% ]
            let p_row_y = track_y + 12.0 * dp_scale;
            let p_btn_w = (track_w - 16.0 * dp_scale) / 3.0;
            let presets = [
                ("35% Light", 0.35f32),
                ("65% Glass", 0.65f32),
                ("95% Deep", 0.95f32),
            ];
            for (pi, (plbl, pval)) in presets.iter().enumerate() {
                let px = pad + 14.0 * dp_scale + (pi as f32) * (p_btn_w + 8.0 * dp_scale);
                let is_cur = (runtime.demo_slider - pval).abs() < 0.08;
                let bg = if is_cur {
                    if theme.is_dark {
                        [0.00, 0.85, 0.55, 0.25]
                    } else {
                        [0.82, 0.96, 0.88, 0.95]
                    }
                } else {
                    if theme.is_dark {
                        [0.15, 0.20, 0.30, 0.6]
                    } else {
                        [0.86, 0.93, 0.99, 0.80]
                    }
                };
                self.ui_batch.push_rect(
                    px,
                    p_row_y,
                    p_btn_w,
                    22.0 * dp_scale,
                    6.0 * dp_scale,
                    bg,
                    if is_cur { 1.2 } else { 0.8 },
                );
                let l_w = self.measure_text(plbl, 9.0 * font_scale);
                let fg = if is_cur {
                    if theme.is_dark {
                        [0.00, 0.95, 0.65, 1.0]
                    } else {
                        [0.02, 0.48, 0.35, 1.0]
                    }
                } else {
                    col_muted
                };
                self.draw_text(
                    px + (p_btn_w - l_w) * 0.5,
                    p_row_y + 6.0 * dp_scale,
                    plbl,
                    9.0 * font_scale,
                    fg,
                );
                self.hit_regions.push(HitRegion {
                    x: px,
                    y: p_row_y,
                    w: p_btn_w,
                    h: 22.0 * dp_scale,
                    action: Action::SetSlider(*pval),
                });
            }

            cur_y += c2_h + 12.0 * dp_scale;

            // Card 2b: Hardware Canvas & Performance Oscilloscope
            let c2b_h = 102.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                c2b_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Hardware Canvas & Performance Monitor",
                13.0 * font_scale,
                col_text,
            );

            // 16-point Waveform Sparkline
            let spark_y = cur_y + 50.0 * dp_scale;
            let spark_w = cw - 28.0 * dp_scale;
            let step_x = spark_w / 15.0;
            for i in 0..16 {
                let px = pad + 14.0 * dp_scale + (i as f32) * step_x;
                let wave = (self.anim_time * 2.8 + (i as f32) * 0.45).sin();
                let py = spark_y + wave * 13.0 * dp_scale;
                let bar_h = (wave * 12.0 * dp_scale).abs() + 4.0 * dp_scale;
                self.ui_batch.push_rect(
                    px - 1.0 * dp_scale,
                    py.min(spark_y),
                    2.0 * dp_scale,
                    bar_h,
                    1.0 * dp_scale,
                    [0.0, 0.85, 1.0, 0.25],
                    0.0,
                );
                self.ui_batch.push_rect(
                    px - 2.5 * dp_scale,
                    py - 2.5 * dp_scale,
                    5.0 * dp_scale,
                    5.0 * dp_scale,
                    2.5 * dp_scale,
                    col_accent,
                    0.0,
                );
            }
            let fps_col = if theme.is_dark {
                [0.0, 0.85, 0.55, 1.0]
            } else {
                [0.02, 0.48, 0.35, 1.0]
            };
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 80.0 * dp_scale,
                "60.0 FPS • 16.6ms frame time • 0 frame drops • Zero GC Engine",
                9.0 * font_scale,
                fps_col,
            );

            cur_y += c2b_h + 12.0 * dp_scale;

            // Card 2c: Syntax-Highlighted Code (Controls Engine)
            let cd2_h = 148.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                cd2_h,
                14.0 * dp_scale,
                code_bg,
                1.2 * dp_scale,
            );

            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.94, 0.27, 0.27, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 25.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.96, 0.62, 0.04, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 36.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.06, 0.72, 0.50, 1.0],
                0.0,
            );
            let c2_fn_col = if theme.is_dark {
                [0.85, 0.90, 1.0, 0.9]
            } else {
                col_text
            };
            self.draw_text(
                pad + 54.0 * dp_scale,
                cur_y + 7.5 * dp_scale,
                "02_controls_math.txs",
                11.0 * font_scale,
                c2_fn_col,
            );

            let badge_c = "[ HARDWARE CONTROL ]";
            let bc_w = self.measure_text(badge_c, 9.0 * font_scale);
            let c2_badge_col = if theme.is_dark {
                [0.78, 0.45, 1.0, 1.0]
            } else {
                col_violet
            };
            self.draw_text(
                pad + cw - bc_w - 14.0 * dp_scale,
                cur_y + 8.0 * dp_scale,
                badge_c,
                9.0 * font_scale,
                c2_badge_col,
            );

            let c2_sep_col = if theme.is_dark {
                [0.15, 0.22, 0.45, 0.6]
            } else {
                [0.68, 0.78, 0.92, 0.50]
            };
            self.ui_batch.push_rect(
                pad,
                cur_y + 24.0 * dp_scale,
                cw,
                1.0 * dp_scale,
                0.5 * dp_scale,
                c2_sep_col,
                0.0,
            );

            let snippet_ctrl = [
                "# Hardware control loop",
                "constant gravity is 9.81",
                "velocity is 0.0",
                "",
                "repeat 4 times:",
                "    velocity becomes velocity + 0.16",
                "    say \"Velocity: {velocity}\"",
            ]
            .join("\n");
            self.draw_syntax_code_block(
                pad + 14.0 * dp_scale,
                cur_y + 32.0 * dp_scale,
                &snippet_ctrl,
                9.5 * font_scale,
                4.5 * dp_scale,
                theme.is_dark,
            );

            let sim_btn_w = 98.0 * dp_scale;
            let sim_btn_h = 24.0 * dp_scale;
            let sim_btn_x = pad + cw - sim_btn_w - 12.0 * dp_scale;
            let sim_btn_y = cur_y + cd2_h - 32.0 * dp_scale;
            let sim_btn_bg = if theme.is_dark {
                [0.78, 0.45, 1.0, 0.18]
            } else {
                [0.90, 0.86, 1.0, 0.95]
            };
            let sim_btn_fg = if theme.is_dark {
                [0.78, 0.45, 1.0, 1.0]
            } else {
                col_violet
            };
            self.ui_batch.push_rect(
                sim_btn_x,
                sim_btn_y,
                sim_btn_w,
                sim_btn_h,
                12.0 * dp_scale,
                sim_btn_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                sim_btn_x + 8.0 * dp_scale,
                sim_btn_y + 6.0 * dp_scale,
                "▶ APPLY SETTINGS",
                8.5 * font_scale,
                sim_btn_fg,
            );
            self.hit_regions.push(HitRegion {
                x: sim_btn_x,
                y: sim_btn_y,
                w: sim_btn_w,
                h: sim_btn_h,
                action: Action::RunQuickAction("tsc check".into()),
            });

            cur_y += cd2_h + 12.0 * dp_scale;
        }

        // ═════════════════════════════════════════════════════════════════════
        // SECTION 3: SURFACES & COLOR MATRIX (Shown in 'All' and 'Surfaces')
        // ═════════════════════════════════════════════════════════════════════
        if sel_cat == 0 || sel_cat == 3 {
            // Card 3: Interactive Liquid Glass Design Tokens & Color Palette
            let c3_h = 138.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                c3_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Liquid Glass Tokens & Color Matrix",
                13.0 * font_scale,
                col_text,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 27.0 * dp_scale,
                "Tap any color token to update 3D shader specular rim & material:",
                9.0 * font_scale,
                col_muted,
            );

            // 6 Interactive Color Tokens (Indigo, Cyan, Amethyst, Emerald, Coral, Amber)
            let swatches = [
                (
                    "Indigo",
                    [0.39, 0.40, 0.95, 0.95],
                    geometry3d::MaterialPreset::TechScriptIndigo,
                ),
                (
                    "Cyan",
                    [0.00, 0.90, 1.00, 0.95],
                    geometry3d::MaterialPreset::LiquidCyan,
                ),
                (
                    "Amethyst",
                    [0.72, 0.28, 0.98, 0.95],
                    geometry3d::MaterialPreset::AmethystPurple,
                ),
                (
                    "Emerald",
                    [0.00, 0.85, 0.55, 0.95],
                    geometry3d::MaterialPreset::EmeraldChrome,
                ),
                (
                    "Coral",
                    [0.95, 0.25, 0.38, 0.95],
                    geometry3d::MaterialPreset::CoralFire,
                ),
                (
                    "Amber",
                    [0.98, 0.65, 0.10, 0.95],
                    geometry3d::MaterialPreset::AmberGold,
                ),
            ];
            let sw_w = (cw - 28.0 * dp_scale - 5.0 * 6.0 * dp_scale) / 6.0;
            let sw_y = cur_y + 44.0 * dp_scale;
            let sw_h = 42.0 * dp_scale;

            for (i, (lbl, col, mat)) in swatches.iter().enumerate() {
                let sx = pad + 14.0 * dp_scale + (i as f32) * (sw_w + 6.0 * dp_scale);
                let is_active_mat = runtime.material == *mat;

                // Glowing background chip
                self.ui_batch.push_rect(
                    sx,
                    sw_y,
                    sw_w,
                    sw_h,
                    10.0 * dp_scale,
                    *col,
                    if is_active_mat { 2.0 * dp_scale } else { 0.0 },
                );
                if is_active_mat {
                    // White checkmark pill
                    self.draw_text(
                        sx + sw_w * 0.5 - 4.0 * dp_scale,
                        sw_y + 8.0 * dp_scale,
                        "✓",
                        11.0 * font_scale,
                        [1.0, 1.0, 1.0, 1.0],
                    );
                }
                let lw = self.measure_text(lbl, 8.5 * font_scale);
                self.draw_text(
                    sx + (sw_w - lw) * 0.5,
                    sw_y + sw_h - 14.0 * dp_scale,
                    lbl,
                    8.5 * font_scale,
                    [1.0, 1.0, 1.0, 1.0],
                );

                // Register Touch Hit Region for Color Chip!
                self.hit_regions.push(HitRegion {
                    x: sx,
                    y: sw_y,
                    w: sw_w,
                    h: sw_h,
                    action: Action::Set3DMaterial(*mat),
                });
            }

            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 116.0 * dp_scale,
                "SDF Refraction Index: 1.52 • Specular Exponent: 32.0 • EGL 1.4 Native",
                9.0 * font_scale,
                col_accent,
            );

            cur_y += c3_h + 12.0 * dp_scale;

            // Card 3b: Spatial Glass Depth & Elevation
            let c3b_h = 76.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                c3b_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Spatial Glass Depth & Elevation Layer",
                13.0 * font_scale,
                col_text,
            );
            let c3b_bg = if theme.is_dark {
                [0.39, 0.40, 0.95, 0.22]
            } else {
                [0.86, 0.93, 0.99, 0.90]
            };
            let c3b_fg = if theme.is_dark {
                col_accent
            } else {
                col_primary
            };
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                cur_y + 32.0 * dp_scale,
                cw - 28.0 * dp_scale,
                32.0 * dp_scale,
                8.0 * dp_scale,
                c3b_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                pad + 24.0 * dp_scale,
                cur_y + 40.0 * dp_scale,
                "Inner Glass Layer  •  Elevation Z = +2.4dp  •  Fresnel Ambient Occlusion",
                9.5 * font_scale,
                c3b_fg,
            );

            cur_y += c3b_h + 12.0 * dp_scale;

            // Card 3c: Syntax-Highlighted Code (Glass Surface)
            let cd3_h = 148.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                cd3_h,
                14.0 * dp_scale,
                code_bg,
                1.2 * dp_scale,
            );

            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.94, 0.27, 0.27, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 25.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.96, 0.62, 0.04, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 36.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.06, 0.72, 0.50, 1.0],
                0.0,
            );
            let c3_fn_col = if theme.is_dark {
                [0.85, 0.90, 1.0, 0.9]
            } else {
                col_text
            };
            self.draw_text(
                pad + 54.0 * dp_scale,
                cur_y + 7.5 * dp_scale,
                "03_glass_surface.txs",
                11.0 * font_scale,
                c3_fn_col,
            );

            let badge_s = "[ SHADER SURFACE ]";
            let bs_w = self.measure_text(badge_s, 9.0 * font_scale);
            let c3_badge_col = if theme.is_dark {
                [0.0, 0.85, 0.55, 1.0]
            } else {
                [0.02, 0.48, 0.35, 1.0]
            };
            self.draw_text(
                pad + cw - bs_w - 14.0 * dp_scale,
                cur_y + 8.0 * dp_scale,
                badge_s,
                9.0 * font_scale,
                c3_badge_col,
            );

            let c3_sep_col = if theme.is_dark {
                [0.15, 0.22, 0.45, 0.6]
            } else {
                [0.68, 0.78, 0.92, 0.50]
            };
            self.ui_batch.push_rect(
                pad,
                cur_y + 24.0 * dp_scale,
                cw,
                1.0 * dp_scale,
                0.5 * dp_scale,
                c3_sep_col,
                0.0,
            );

            let snippet_surf = [
                "# Declarative liquid glass surface",
                "to render_surface(glow):",
                "    if glow:",
                "        say \"Rim: Cyan Glow\"",
                "    else:",
                "        say \"Rim: Dark Navy\"",
                "    return \"done\"",
            ]
            .join("\n");
            self.draw_syntax_code_block(
                pad + 14.0 * dp_scale,
                cur_y + 32.0 * dp_scale,
                &snippet_surf,
                9.5 * font_scale,
                4.5 * dp_scale,
                theme.is_dark,
            );

            let shader_btn_w = 88.0 * dp_scale;
            let shader_btn_h = 24.0 * dp_scale;
            let shader_btn_x = pad + cw - shader_btn_w - 12.0 * dp_scale;
            let shader_btn_y = cur_y + cd3_h - 32.0 * dp_scale;
            let shader_btn_bg = if theme.is_dark {
                [0.0, 0.85, 0.55, 0.18]
            } else {
                [0.84, 0.96, 0.90, 0.95]
            };
            let shader_btn_fg = if theme.is_dark {
                [0.0, 0.85, 0.55, 1.0]
            } else {
                [0.02, 0.48, 0.35, 1.0]
            };
            self.ui_batch.push_rect(
                shader_btn_x,
                shader_btn_y,
                shader_btn_w,
                shader_btn_h,
                12.0 * dp_scale,
                shader_btn_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                shader_btn_x + 8.0 * dp_scale,
                shader_btn_y + 6.0 * dp_scale,
                "▶ TEST SHADER",
                9.0 * font_scale,
                shader_btn_fg,
            );
            self.hit_regions.push(HitRegion {
                x: shader_btn_x,
                y: shader_btn_y,
                w: shader_btn_w,
                h: shader_btn_h,
                action: Action::RunQuickAction("tsc check".into()),
            });

            cur_y += cd3_h + 12.0 * dp_scale;
        }

        // ═════════════════════════════════════════════════════════════════════
        // SECTION 4: STATE & INVARIANTS (Shown in 'All' and 'State')
        // ═════════════════════════════════════════════════════════════════════
        if sel_cat == 0 || sel_cat == 4 {
            // Card 4: Reactive State Machine & Telemetry
            let c4_h = 142.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                c4_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Reactive State Machine & Telemetry",
                13.0 * font_scale,
                col_text,
            );

            let count_lbl = format!("Active State Value: {}", runtime.demo_counter);
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 30.0 * dp_scale,
                &count_lbl,
                15.0 * font_scale,
                col_accent,
            );

            // Row of state action buttons: [- 1] [+ 1] [+ 10] [Reset]
            let s_btn_y = cur_y + 54.0 * dp_scale;
            let s_btn_h = 30.0 * dp_scale;
            let s_btn_w = (cw - 28.0 * dp_scale - 24.0 * dp_scale) / 4.0;

            // [- 1]
            let s1_x = pad + 14.0 * dp_scale;
            self.ui_batch.push_rect(
                s1_x,
                s_btn_y,
                s_btn_w,
                s_btn_h,
                8.0 * dp_scale,
                col_primary,
                0.0,
            );
            self.draw_text(
                s1_x + s_btn_w * 0.5 - 7.0 * dp_scale,
                s_btn_y + 8.0 * dp_scale,
                "- 1",
                11.0 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: s1_x,
                y: s_btn_y,
                w: s_btn_w,
                h: s_btn_h,
                action: Action::DecrementCounter,
            });

            // [+ 1]
            let s2_x = s1_x + s_btn_w + 8.0 * dp_scale;
            let s2_bg = if theme.is_dark {
                [0.00, 0.85, 0.55, 1.0]
            } else {
                [0.04, 0.52, 0.38, 1.0]
            };
            self.ui_batch
                .push_rect(s2_x, s_btn_y, s_btn_w, s_btn_h, 8.0 * dp_scale, s2_bg, 0.0);
            self.draw_text(
                s2_x + s_btn_w * 0.5 - 7.0 * dp_scale,
                s_btn_y + 8.0 * dp_scale,
                "+ 1",
                11.0 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: s2_x,
                y: s_btn_y,
                w: s_btn_w,
                h: s_btn_h,
                action: Action::IncrementCounter,
            });

            // [+ 10]
            let s3_x = s2_x + s_btn_w + 8.0 * dp_scale;
            let s3_bg = if theme.is_dark {
                [0.72, 0.28, 0.98, 1.0]
            } else {
                [0.43, 0.16, 0.85, 1.0]
            };
            self.ui_batch
                .push_rect(s3_x, s_btn_y, s_btn_w, s_btn_h, 8.0 * dp_scale, s3_bg, 0.0);
            self.draw_text(
                s3_x + s_btn_w * 0.5 - 11.0 * dp_scale,
                s_btn_y + 8.0 * dp_scale,
                "+ 10",
                11.0 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: s3_x,
                y: s_btn_y,
                w: s_btn_w,
                h: s_btn_h,
                action: Action::IncrementCounter,
            });

            // [Reset]
            let s4_x = s3_x + s_btn_w + 8.0 * dp_scale;
            let s4_bg = if theme.is_dark {
                [0.95, 0.25, 0.38, 1.0]
            } else {
                [0.75, 0.12, 0.25, 1.0]
            };
            self.ui_batch
                .push_rect(s4_x, s_btn_y, s_btn_w, s_btn_h, 8.0 * dp_scale, s4_bg, 0.0);
            self.draw_text(
                s4_x + s_btn_w * 0.5 - 13.0 * dp_scale,
                s_btn_y + 8.0 * dp_scale,
                "Reset",
                10.0 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: s4_x,
                y: s_btn_y,
                w: s_btn_w,
                h: s_btn_h,
                action: Action::ResetCounter,
            });

            // 7-Bar Realtime Telemetry Chart
            let chart_y = s_btn_y + s_btn_h + 10.0 * dp_scale;
            let bars = [42.0, 75.0, 58.0, 92.0, 64.0, 85.0, 100.0];
            let bar_w = (cw - 28.0 * dp_scale - 24.0 * dp_scale) / 7.0;
            for (bi, bh) in bars.iter().enumerate() {
                let bx = pad + 14.0 * dp_scale + (bi as f32) * (bar_w + 4.0 * dp_scale);
                let bar_h_px = (bh / 100.0) * 18.0 * dp_scale;
                self.ui_batch.push_rect(
                    bx,
                    chart_y + 18.0 * dp_scale - bar_h_px,
                    bar_w,
                    bar_h_px,
                    2.0 * dp_scale,
                    col_primary,
                    0.0,
                );
            }
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 124.0 * dp_scale,
                "Zero GC Pauses  •  Instant differential reactive UI dispatch",
                9.0 * font_scale,
                col_muted,
            );

            cur_y += c4_h + 12.0 * dp_scale;

            // Card 4b: Interactive Compiler REPL Shell
            let repl_h = 92.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                repl_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "Interactive Compiler Shell (REPL)",
                13.0 * font_scale,
                col_text,
            );

            let repl_bg = if theme.is_dark {
                [0.03, 0.06, 0.14, 0.95]
            } else {
                [0.93, 0.95, 0.98, 0.95]
            };
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                cur_y + 32.0 * dp_scale,
                cw - 28.0 * dp_scale,
                26.0 * dp_scale,
                8.0 * dp_scale,
                repl_bg,
                0.8 * dp_scale,
            );
            self.draw_text(
                pad + 22.0 * dp_scale,
                cur_y + 39.0 * dp_scale,
                "$ tsc eval \"radius is 14; 2 * 3.14159 * radius\"",
                9.5 * font_scale,
                col_accent,
            );

            let eval_out_col = if theme.is_dark {
                [0.0, 0.85, 0.55, 1.0]
            } else {
                [0.02, 0.48, 0.35, 1.0]
            };
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 68.0 * dp_scale,
                "✓ Output: 87.9645 (Float64) • Execution: 4ms",
                9.5 * font_scale,
                eval_out_col,
            );
            let eval_pill_w = 64.0 * dp_scale;
            let eval_pill_h = 22.0 * dp_scale;
            let eval_x = pad + cw - eval_pill_w - 14.0 * dp_scale;
            let eval_y = cur_y + 62.0 * dp_scale;
            let eval_btn_bg = if theme.is_dark {
                [0.0, 0.85, 0.55, 0.18]
            } else {
                [0.84, 0.96, 0.90, 0.95]
            };
            let eval_btn_fg = if theme.is_dark {
                [0.0, 0.85, 0.55, 1.0]
            } else {
                [0.02, 0.48, 0.35, 1.0]
            };
            self.ui_batch.push_rect(
                eval_x,
                eval_y,
                eval_pill_w,
                eval_pill_h,
                11.0 * dp_scale,
                eval_btn_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                eval_x + 10.0 * dp_scale,
                eval_y + 5.5 * dp_scale,
                "▶ EVAL",
                9.5 * font_scale,
                eval_btn_fg,
            );
            self.hit_regions.push(HitRegion {
                x: eval_x,
                y: eval_y,
                w: eval_pill_w,
                h: eval_pill_h,
                action: Action::RunQuickAction("tsc run".into()),
            });

            cur_y += repl_h + 12.0 * dp_scale;

            // Card 4c: Syntax-Highlighted Code (Reactive State Component)
            let cd4_h = 144.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                cd4_h,
                14.0 * dp_scale,
                code_bg,
                1.2 * dp_scale,
            );

            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.94, 0.27, 0.27, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 25.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.96, 0.62, 0.04, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 36.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.06, 0.72, 0.50, 1.0],
                0.0,
            );
            let c4_fn_col = if theme.is_dark {
                [0.85, 0.90, 1.0, 0.9]
            } else {
                col_text
            };
            self.draw_text(
                pad + 54.0 * dp_scale,
                cur_y + 7.5 * dp_scale,
                "04_reactive_state.txs",
                11.0 * font_scale,
                c4_fn_col,
            );

            let badge_st = "[ BYTECODE VM ]";
            let bst_w = self.measure_text(badge_st, 9.0 * font_scale);
            self.draw_text(
                pad + cw - bst_w - 14.0 * dp_scale,
                cur_y + 8.0 * dp_scale,
                badge_st,
                9.0 * font_scale,
                col_accent,
            );

            let c4_sep_col = if theme.is_dark {
                [0.15, 0.22, 0.45, 0.6]
            } else {
                [0.68, 0.78, 0.92, 0.50]
            };
            self.ui_batch.push_rect(
                pad,
                cur_y + 24.0 * dp_scale,
                cw,
                1.0 * dp_scale,
                0.5 * dp_scale,
                c4_sep_col,
                0.0,
            );

            let snippet_state = [
                "# TechScript reactive state",
                "counter is 0",
                "",
                "to mutate(step):",
                "    counter becomes counter + step",
                "    say \"Value: {counter}\"",
            ]
            .join("\n");
            self.draw_syntax_code_block(
                pad + 14.0 * dp_scale,
                cur_y + 32.0 * dp_scale,
                &snippet_state,
                9.5 * font_scale,
                4.5 * dp_scale,
                theme.is_dark,
            );

            let run_st_w = 98.0 * dp_scale;
            let run_st_h = 24.0 * dp_scale;
            let run_st_x = pad + cw - run_st_w - 12.0 * dp_scale;
            let run_st_y = cur_y + cd4_h - 32.0 * dp_scale;
            let run_st_bg = if theme.is_dark {
                [0.12, 0.83, 0.93, 0.18]
            } else {
                [0.84, 0.92, 1.0, 0.95]
            };
            self.ui_batch.push_rect(
                run_st_x,
                run_st_y,
                run_st_w,
                run_st_h,
                12.0 * dp_scale,
                run_st_bg,
                1.0 * dp_scale,
            );
            self.draw_text(
                run_st_x + 8.0 * dp_scale,
                run_st_y + 6.0 * dp_scale,
                "▶ RUN STATE TEST",
                8.5 * font_scale,
                col_accent,
            );
            self.hit_regions.push(HitRegion {
                x: run_st_x,
                y: run_st_y,
                w: run_st_w,
                h: run_st_h,
                action: Action::RunQuickAction("tsc run".into()),
            });

            cur_y += cd4_h + 12.0 * dp_scale;

            // Card 4d: TechScript Language Canonical Invariants (Clean 2-row layout with ZERO horizontal overflow!)
            let inv_h = 196.0 * dp_scale;
            self.ui_batch.push_rect(
                pad,
                cur_y,
                cw,
                inv_h,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                cur_y + 11.0 * dp_scale,
                "TechScript Language Canonical Invariants",
                13.0 * font_scale,
                col_text,
            );

            let rules = [
                (
                    "• Declaration:",
                    "name is \"TechScript\"",
                    "↳ Uses 'is' for first variable binding",
                ),
                (
                    "• Mutation:",
                    "count becomes count + 1",
                    "↳ Uses 'becomes' for explicit mutation",
                ),
                (
                    "• Constants:",
                    "constant max is 100",
                    "↳ Immutable compile-time constant",
                ),
                (
                    "• Functions:",
                    "to greet(p): return \"ok\"",
                    "↳ 4-space block with colon delimiter",
                ),
                (
                    "• Output:",
                    "say \"Count: {count}\"",
                    "↳ Automatic curly string interpolation",
                ),
            ];

            let mut r_y = cur_y + 32.0 * dp_scale;
            for (r_name, r_code, r_desc) in rules {
                // Line 1: Invariant Name + Code snippet
                self.draw_text(
                    pad + 14.0 * dp_scale,
                    r_y,
                    r_name,
                    10.0 * font_scale,
                    col_accent,
                );
                self.draw_text(
                    pad + 110.0 * dp_scale,
                    r_y,
                    r_code,
                    10.0 * font_scale,
                    col_text,
                );
                // Line 2: Note / Rule description (indented, comfortably within width)
                self.draw_text(
                    pad + 28.0 * dp_scale,
                    r_y + 13.0 * dp_scale,
                    r_desc,
                    9.0 * font_scale,
                    col_muted,
                );
                r_y += 30.0 * dp_scale;
            }

            cur_y += inv_h + 12.0 * dp_scale;
        }

        // Footer & Content Boundaries
        let foot_y = cur_y + 8.0 * dp_scale;
        let foot_text1 = "TechScript AppForge v2.0.0 • Verified Native Component Gallery";
        let foot_w1 = self.measure_text(foot_text1, 9.5 * font_scale);
        self.draw_text(
            pad + (cw - foot_w1) * 0.5,
            foot_y,
            foot_text1,
            9.5 * font_scale,
            col_muted,
        );

        let total_content_h = (foot_y + 36.0 * dp_scale) - sy;
        runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
    }

    /// Destination 3: Labs Screen (3D Engine, Spatial Models, Custom Color Matrix, & Shaders)
    fn render_labs_content(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        pad: f32,
        cw: f32,
        start_y: f32,
        content_bottom: f32,
    ) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);
        let code_bg = if theme.is_dark {
            [0.04, 0.07, 0.15, 0.95]
        } else {
            [0.94, 0.96, 0.98, 0.95]
        };

        let sy = start_y - runtime.scroll_y;

        // Title & High-Tech HUD Subtitle
        self.draw_text(
            pad,
            sy,
            "TechScript 3D Spatial Lab",
            17.5 * font_scale,
            col_accent,
        );
        self.draw_text(
            pad,
            sy + 21.0 * dp_scale,
            "Procedural Geometry • Multi-Touch 360° Gestures • Realtime Shaders",
            9.5 * font_scale,
            col_muted,
        );

        let mut cur_y = sy + 38.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 1: 3D PLAYGROUND VIEWPORT (Hardware GLES 2.0 Scissored Surface)
        // ═════════════════════════════════════════════════════════════════════
        let vp_card_h = 250.0 * dp_scale;
        self.viewport_3d_rect = Some((pad, cur_y, cw, vp_card_h));
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            vp_card_h,
            16.0 * dp_scale,
            col_card,
            1.4 * dp_scale,
        );

        // Ambient illumination halo and celestial disc behind 3D model
        let center_y = cur_y + vp_card_h * 0.5;
        if theme.is_dark {
            self.ui_batch.push_rect(
                pad + cw * 0.5 - 90.0 * dp_scale,
                center_y - 90.0 * dp_scale,
                180.0 * dp_scale,
                180.0 * dp_scale,
                90.0 * dp_scale,
                [0.39, 0.40, 0.95, 0.14],
                0.0,
            );
        } else {
            self.ui_batch.push_rect(
                pad + cw * 0.5 - 95.0 * dp_scale,
                center_y - 95.0 * dp_scale,
                190.0 * dp_scale,
                190.0 * dp_scale,
                95.0 * dp_scale,
                [0.78, 0.88, 1.0, 0.45],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + cw * 0.5 - 65.0 * dp_scale,
                center_y - 65.0 * dp_scale,
                130.0 * dp_scale,
                130.0 * dp_scale,
                65.0 * dp_scale,
                [0.85, 0.92, 1.0, 0.55],
                0.0,
            );
        }
        let rim_col = runtime.material.rim_color();
        self.ui_batch.push_ring(
            pad + cw * 0.5,
            center_y,
            70.0 * dp_scale,
            1.5 * dp_scale,
            [
                rim_col[0],
                rim_col[1],
                rim_col[2],
                if theme.is_dark { 0.40 } else { 0.25 },
            ],
        );

        // Header inside viewport card
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 12.0 * dp_scale,
            "3D Spatial Viewport",
            13.0 * font_scale,
            col_text,
        );

        // Geometric mesh badge & toggle hit region
        let geom_badge = if runtime.wireframe_mode {
            "[ 📐 Mesh: ON ]"
        } else {
            "[ 📐 Mesh: OFF ]"
        };
        let geom_w = self.measure_text(geom_badge, 9.5 * font_scale);
        let geom_x = pad + cw - geom_w - 14.0 * dp_scale;
        let geom_col = if runtime.wireframe_mode {
            [0.0, 1.0, 0.85, 1.0]
        } else {
            col_muted
        };
        self.draw_text(
            geom_x,
            cur_y + 12.0 * dp_scale,
            geom_badge,
            9.5 * font_scale,
            geom_col,
        );
        self.hit_regions.push(HitRegion {
            x: geom_x - 4.0 * dp_scale,
            y: cur_y + 6.0 * dp_scale,
            w: geom_w + 8.0 * dp_scale,
            h: 22.0 * dp_scale,
            action: Action::ToggleWireframe,
        });

        // Active material badge
        let mat_badge = if runtime.is_custom_color {
            format!("[ Hue: {:.0}° ]", runtime.custom_hue * 360.0)
        } else {
            format!("[ {} ]", runtime.material.name())
        };
        let mat_w = self.measure_text(&mat_badge, 9.5 * font_scale);
        self.draw_text(
            geom_x - mat_w - 8.0 * dp_scale,
            cur_y + 12.0 * dp_scale,
            &mat_badge,
            9.5 * font_scale,
            col_accent,
        );

        let mesh_label = format!("Model: {} • Move Anywhere Active", runtime.mesh_type.name());
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 28.0 * dp_scale,
            &mesh_label,
            10.0 * font_scale,
            col_muted,
        );

        // Real-Time Pose & Spatial Telemetry HUD (Bottom of Viewport)
        let hud_y = cur_y + vp_card_h - 26.0 * dp_scale;
        let hud_bg = if theme.is_dark {
            [0.02, 0.04, 0.10, 0.85]
        } else {
            [0.86, 0.93, 0.99, 0.90]
        };
        self.ui_batch.push_rect(
            pad + 8.0 * dp_scale,
            hud_y,
            cw - 16.0 * dp_scale,
            20.0 * dp_scale,
            6.0 * dp_scale,
            hud_bg,
            0.8 * dp_scale,
        );
        let pose_hud = format!(
            "Yaw: {:>3.0}° • Pitch: {:>3.0}° • Zoom: {:>3.2}x • Pan: ({:>+.2}, {:>+.2})",
            runtime.rot_y.to_degrees() % 360.0,
            runtime.rot_x.to_degrees() % 360.0,
            runtime.zoom,
            runtime.model_pos_x,
            runtime.model_pos_y,
        );
        let hud_col = if theme.is_dark {
            [0.0, 0.90, 1.0, 0.95]
        } else {
            col_primary
        };
        self.draw_text(
            pad + 14.0 * dp_scale,
            hud_y + 5.0 * dp_scale,
            &pose_hud,
            8.5 * font_scale,
            hud_col,
        );

        cur_y += vp_card_h + 12.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 2: PROCEDURAL 3D MODELS (23 Complex & Futuristic Models)
        // ═════════════════════════════════════════════════════════════════════
        let cat_models: &[&[(MeshType, &str)]] = &[
            // Category 0: Fleet & Aerospace (7)
            &[
                (MeshType::Starship, "Starfighter"),
                (MeshType::Dreadnought, "Cruiser"),
                (MeshType::CyberDrone, "Drone"),
                (MeshType::SpaceStation, "Station"),
                (MeshType::MarsRover, "Rover"),
                (MeshType::StealthJet, "Stealth"),
                (MeshType::Satellite, "Satellite"),
            ],
            // Category 1: Cosmos & Astronomy (7)
            &[
                (MeshType::EarthGlobe, "Earth"),
                (MeshType::SolarSystem, "Solar Sys"),
                (MeshType::SpiralGalaxy, "Galaxy"),
                (MeshType::SaturnRings, "Saturn"),
                (MeshType::BlackHole, "Black Hole"),
                (MeshType::Pulsar, "Pulsar"),
                (MeshType::Meteoroid, "Asteroid"),
            ],
            // Category 2: Math, Tech & Crystals (9)
            &[
                (MeshType::QuantumTorus, "Torus"),
                (MeshType::DNAHelix, "DNA Helix"),
                (MeshType::Tesseract, "4D Cube"),
                (MeshType::CyberTower, "Tower"),
                (MeshType::TechCore, "Core"),
                (MeshType::StarGem, "Star"),
                (MeshType::Octahedron, "Octa"),
                (MeshType::HexPrism, "Prism"),
                (MeshType::Crystal, "Crystal"),
            ],
        ];

        let active_cat = runtime.model_category.min(2);
        let models_in_cat = cat_models[active_cat];
        let num_rows = (models_in_cat.len() + 3) / 4;
        let c2_h = 36.0 * dp_scale
            + 30.0 * dp_scale
            + (num_rows as f32) * (26.0 * dp_scale + 6.0 * dp_scale)
            + 34.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            c2_h,
            14.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 10.0 * dp_scale,
            "Procedural 3D Geometry Models (23 Total)",
            12.5 * font_scale,
            col_text,
        );

        let cur_idx = runtime.mesh_type.index();
        let cat_counter = format!(
            "Active [{}/23]: {} • Tap to Select",
            cur_idx,
            runtime.mesh_type.name()
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 24.0 * dp_scale,
            &cat_counter,
            8.5 * font_scale,
            col_accent,
        );

        // Category Tabs: [ 🚀 Fleet (7) ] [ 🌌 Cosmos (7) ] [ 🔬 Tech (9) ]
        let cat_tabs = [
            ("🚀 Fleet (7)", 0usize),
            ("🌌 Cosmos (7)", 1usize),
            ("🔬 Tech (9)", 2usize),
        ];
        let tab_w = (cw - 28.0 * dp_scale - 2.0 * 6.0 * dp_scale) / 3.0;
        let tab_y = cur_y + 38.0 * dp_scale;
        for (ti, (tlbl, tcat)) in cat_tabs.iter().enumerate() {
            let tx = pad + 14.0 * dp_scale + (ti as f32) * (tab_w + 6.0 * dp_scale);
            let is_sel = active_cat == *tcat;
            let bg = if is_sel {
                col_primary
            } else {
                if theme.is_dark {
                    [0.15, 0.20, 0.32, 0.60]
                } else {
                    [0.86, 0.93, 0.99, 0.85]
                }
            };
            self.ui_batch.push_rect(
                tx,
                tab_y,
                tab_w,
                24.0 * dp_scale,
                6.0 * dp_scale,
                bg,
                if is_sel {
                    1.5 * dp_scale
                } else {
                    0.8 * dp_scale
                },
            );
            let tw = self.measure_text(tlbl, 9.0 * font_scale);
            self.draw_text(
                tx + (tab_w - tw) * 0.5,
                tab_y + 7.0 * dp_scale,
                tlbl,
                9.0 * font_scale,
                if is_sel {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    col_muted
                },
            );
            self.hit_regions.push(HitRegion {
                x: tx,
                y: tab_y,
                w: tab_w,
                h: 24.0 * dp_scale,
                action: Action::SelectModelCategory(*tcat),
            });
        }

        // Active Category Models Grid (4 items per row)
        let m_btn_w = (cw - 28.0 * dp_scale - 3.0 * 6.0 * dp_scale) / 4.0;
        let m_btn_h = 26.0 * dp_scale;
        let grid_start_y = tab_y + 28.0 * dp_scale;

        for (i, (m, lbl)) in models_in_cat.iter().enumerate() {
            let row = i / 4;
            let col = i % 4;
            let bx = pad + 14.0 * dp_scale + (col as f32) * (m_btn_w + 6.0 * dp_scale);
            let by = grid_start_y + (row as f32) * (m_btn_h + 6.0 * dp_scale);

            let is_sel = runtime.mesh_type == *m;
            let bg = if is_sel {
                col_primary
            } else {
                if theme.is_dark {
                    [0.12, 0.16, 0.28, 0.70]
                } else {
                    [0.88, 0.94, 1.0, 0.85]
                }
            };
            self.ui_batch.push_rect(
                bx,
                by,
                m_btn_w,
                m_btn_h,
                7.0 * dp_scale,
                bg,
                if is_sel {
                    1.6 * dp_scale
                } else {
                    0.8 * dp_scale
                },
            );
            let tw = self.measure_text(lbl, 9.0 * font_scale);
            self.draw_text(
                bx + (m_btn_w - tw) * 0.5,
                by + 7.0 * dp_scale,
                lbl,
                9.0 * font_scale,
                if is_sel {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    col_text
                },
            );
            self.hit_regions.push(HitRegion {
                x: bx,
                y: by,
                w: m_btn_w,
                h: m_btn_h,
                action: Action::Set3DMesh(*m),
            });
        }

        // Sequential Carousel Navigation Bar: [ ‹ Prev Model ] [ Next Model › ]
        let nav_y = grid_start_y + (num_rows as f32) * (m_btn_h + 6.0 * dp_scale) + 2.0 * dp_scale;
        let nav_w = (cw - 28.0 * dp_scale - 8.0 * dp_scale) * 0.5;
        let nav_btn_bg = if theme.is_dark {
            [0.15, 0.22, 0.38, 0.85]
        } else {
            [0.86, 0.93, 0.99, 0.90]
        };
        // [ ‹ Prev Model ]
        self.ui_batch.push_rect(
            pad + 14.0 * dp_scale,
            nav_y,
            nav_w,
            24.0 * dp_scale,
            6.0 * dp_scale,
            nav_btn_bg,
            0.8 * dp_scale,
        );
        let tw_prev = self.measure_text("‹ Prev Model", 9.5 * font_scale);
        self.draw_text(
            pad + 14.0 * dp_scale + (nav_w - tw_prev) * 0.5,
            nav_y + 6.0 * dp_scale,
            "‹ Prev Model",
            9.5 * font_scale,
            col_accent,
        );
        self.hit_regions.push(HitRegion {
            x: pad + 14.0 * dp_scale,
            y: nav_y,
            w: nav_w,
            h: 24.0 * dp_scale,
            action: Action::Prev3DMesh,
        });

        // [ Next Model › ]
        let next_x = pad + 14.0 * dp_scale + nav_w + 8.0 * dp_scale;
        self.ui_batch.push_rect(
            next_x,
            nav_y,
            nav_w,
            24.0 * dp_scale,
            6.0 * dp_scale,
            nav_btn_bg,
            0.8 * dp_scale,
        );
        let tw_next = self.measure_text("Next Model ›", 9.5 * font_scale);
        self.draw_text(
            next_x + (nav_w - tw_next) * 0.5,
            nav_y + 6.0 * dp_scale,
            "Next Model ›",
            9.5 * font_scale,
            col_accent,
        );
        self.hit_regions.push(HitRegion {
            x: next_x,
            y: nav_y,
            w: nav_w,
            h: 24.0 * dp_scale,
            action: Action::Cycle3DMesh,
        });

        cur_y += c2_h + 12.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 3: COLOR PALETTE & CUSTOM SPECTRUM RECOLOR
        // ═════════════════════════════════════════════════════════════════════
        let c3_h = 160.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            c3_h,
            14.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 10.0 * dp_scale,
            "Color Palette & Custom Color Spectrum",
            12.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 25.0 * dp_scale,
            "Tap preset chips or slide across spectrum to recolor model:",
            8.5 * font_scale,
            col_muted,
        );

        let mat_row1 = [
            (
                "Indigo",
                [0.39, 0.40, 0.95, 0.95],
                MaterialPreset::TechScriptIndigo,
            ),
            ("Cyan", [0.00, 0.90, 1.00, 0.95], MaterialPreset::LiquidCyan),
            (
                "Amethyst",
                [0.72, 0.28, 0.98, 0.95],
                MaterialPreset::AmethystPurple,
            ),
            (
                "Emerald",
                [0.00, 0.85, 0.55, 0.95],
                MaterialPreset::EmeraldChrome,
            ),
            ("Coral", [0.95, 0.25, 0.38, 0.95], MaterialPreset::CoralFire),
        ];

        let mat_row2 = [
            ("Amber", [0.98, 0.65, 0.10, 0.95], MaterialPreset::AmberGold),
            ("Ruby", [0.98, 0.14, 0.24, 0.95], MaterialPreset::RubyLaser),
            ("Lime", [0.55, 0.95, 0.05, 0.95], MaterialPreset::CyberLime),
            (
                "Plasma",
                [1.00, 0.80, 0.15, 0.95],
                MaterialPreset::PlasmaGold,
            ),
            (
                "Diamond",
                [0.35, 0.85, 1.00, 0.95],
                MaterialPreset::DiamondIce,
            ),
        ];

        let col_chip_w = (cw - 28.0 * dp_scale - 4.0 * 6.0 * dp_scale) / 5.0;
        let col_chip_h = 24.0 * dp_scale;

        // Color Row 1
        let cr1_y = cur_y + 40.0 * dp_scale;
        for (i, (lbl, col, mat)) in mat_row1.iter().enumerate() {
            let cx = pad + 14.0 * dp_scale + (i as f32) * (col_chip_w + 6.0 * dp_scale);
            let is_sel = !runtime.is_custom_color && runtime.material == *mat;
            self.ui_batch.push_rect(
                cx,
                cr1_y,
                col_chip_w,
                col_chip_h,
                6.0 * dp_scale,
                *col,
                if is_sel { 2.0 * dp_scale } else { 0.0 },
            );
            let tw = self.measure_text(lbl, 8.0 * font_scale);
            let chip_txt_col = if (col[0] * 0.299 + col[1] * 0.587 + col[2] * 0.114) > 0.6 {
                [0.05, 0.08, 0.15, 1.0]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            };
            self.draw_text(
                cx + (col_chip_w - tw) * 0.5,
                cr1_y + 7.0 * dp_scale,
                lbl,
                8.0 * font_scale,
                chip_txt_col,
            );
            self.hit_regions.push(HitRegion {
                x: cx,
                y: cr1_y,
                w: col_chip_w,
                h: col_chip_h,
                action: Action::Set3DMaterial(*mat),
            });
        }

        // Color Row 2
        let cr2_y = cr1_y + col_chip_h + 6.0 * dp_scale;
        for (i, (lbl, col, mat)) in mat_row2.iter().enumerate() {
            let cx = pad + 14.0 * dp_scale + (i as f32) * (col_chip_w + 6.0 * dp_scale);
            let is_sel = !runtime.is_custom_color && runtime.material == *mat;
            self.ui_batch.push_rect(
                cx,
                cr2_y,
                col_chip_w,
                col_chip_h,
                6.0 * dp_scale,
                *col,
                if is_sel { 2.0 * dp_scale } else { 0.0 },
            );
            let tw = self.measure_text(lbl, 8.0 * font_scale);
            let chip_txt_col = if (col[0] * 0.299 + col[1] * 0.587 + col[2] * 0.114) > 0.6 {
                [0.05, 0.08, 0.15, 1.0]
            } else {
                [1.0, 1.0, 1.0, 1.0]
            };
            self.draw_text(
                cx + (col_chip_w - tw) * 0.5,
                cr2_y + 7.0 * dp_scale,
                lbl,
                8.0 * font_scale,
                chip_txt_col,
            );
            self.hit_regions.push(HitRegion {
                x: cx,
                y: cr2_y,
                w: col_chip_w,
                h: col_chip_h,
                action: Action::Set3DMaterial(*mat),
            });
        }

        // Custom Spectrum Bar (8 Spectrum segments)
        let spec_y = cr2_y + col_chip_h + 10.0 * dp_scale;
        let spec_h = 20.0 * dp_scale;
        let spectrum_steps = [
            (0.00, [1.00, 0.15, 0.15, 1.0]), // Red
            (0.12, [1.00, 0.55, 0.10, 1.0]), // Orange
            (0.25, [1.00, 0.90, 0.15, 1.0]), // Yellow
            (0.38, [0.20, 0.90, 0.30, 1.0]), // Green
            (0.52, [0.00, 0.90, 1.00, 1.0]), // Cyan
            (0.68, [0.25, 0.45, 1.00, 1.0]), // Blue
            (0.80, [0.70, 0.25, 0.98, 1.0]), // Purple
            (0.92, [0.98, 0.20, 0.65, 1.0]), // Magenta
        ];
        let spec_w = (cw - 28.0 * dp_scale - 7.0 * 3.0 * dp_scale) / 8.0;

        for (i, (hue, col)) in spectrum_steps.iter().enumerate() {
            let sx = pad + 14.0 * dp_scale + (i as f32) * (spec_w + 3.0 * dp_scale);
            let is_cur = runtime.is_custom_color && (runtime.custom_hue - *hue).abs() < 0.07;
            self.ui_batch.push_rect(
                sx,
                spec_y,
                spec_w,
                spec_h,
                4.0 * dp_scale,
                *col,
                if is_cur { 2.0 * dp_scale } else { 0.0 },
            );
            self.hit_regions.push(HitRegion {
                x: sx,
                y: spec_y,
                w: spec_w,
                h: spec_h,
                action: Action::SetCustomHue(*hue),
            });
        }

        let cur_col = runtime.active_material_color();
        let col_desc = format!(
            "Active RGB: ({:.2}, {:.2}, {:.2}) • Specular: Blinn-Phong 32x • Refraction: 1.52",
            cur_col[0], cur_col[1], cur_col[2]
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            spec_y + spec_h + 8.0 * dp_scale,
            &col_desc,
            8.5 * font_scale,
            col_accent,
        );

        cur_y += c3_h + 12.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 4: PRECISION CAMERA & SPATIAL TRANSLATION (Move Anywhere!)
        // ═════════════════════════════════════════════════════════════════════
        let c4_h = 136.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            c4_h,
            14.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 10.0 * dp_scale,
            "Precision Camera & Spatial Translation",
            12.5 * font_scale,
            col_text,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 25.0 * dp_scale,
            "Pinch to zoom • Drag to rotate 360° • Translate model anywhere:",
            8.5 * font_scale,
            col_muted,
        );

        // Row 1: Pan D-Pad Buttons: [ ◄ Left ] [ ▲ Up ] [ ▼ Down ] [ Right ► ] [ ⊙ Center ]
        let dpad_y = cur_y + 40.0 * dp_scale;
        let dpad_btns = [
            ("◄ Left", Action::Pan3D(-0.35, 0.0)),
            ("▲ Up", Action::Pan3D(0.0, 0.35)),
            ("▼ Down", Action::Pan3D(0.0, -0.35)),
            ("Right ►", Action::Pan3D(0.35, 0.0)),
            ("⊙ Center", Action::Reset3DPosition),
        ];
        let dp_btn_w = (cw - 28.0 * dp_scale - 4.0 * 6.0 * dp_scale) / 5.0;
        let dp_btn_h = 28.0 * dp_scale;

        for (i, (lbl, act)) in dpad_btns.iter().enumerate() {
            let bx = pad + 14.0 * dp_scale + (i as f32) * (dp_btn_w + 6.0 * dp_scale);
            let dpad_bg = if theme.is_dark {
                [0.15, 0.20, 0.35, 0.85]
            } else {
                [0.86, 0.93, 0.99, 0.90]
            };
            self.ui_batch.push_rect(
                bx,
                dpad_y,
                dp_btn_w,
                dp_btn_h,
                7.0 * dp_scale,
                dpad_bg,
                1.0 * dp_scale,
            );
            let tw = self.measure_text(lbl, 8.5 * font_scale);
            self.draw_text(
                bx + (dp_btn_w - tw) * 0.5,
                dpad_y + 8.0 * dp_scale,
                lbl,
                8.5 * font_scale,
                col_accent,
            );
            self.hit_regions.push(HitRegion {
                x: bx,
                y: dpad_y,
                w: dp_btn_w,
                h: dp_btn_h,
                action: act.clone(),
            });
        }

        // Row 2: Zoom & Motion Buttons: [ Zoom + ] [ Zoom - ] [ Auto-Rot ] [ 📐 Geom ] [ ⟲ Reset ]
        let zoom_y = dpad_y + dp_btn_h + 8.0 * dp_scale;
        let auto_lbl = if runtime.auto_rotate {
            "Auto: ON"
        } else {
            "Auto: OFF"
        };
        let geom_lbl = if runtime.wireframe_mode {
            "Geom: ON"
        } else {
            "Geom: OFF"
        };
        let ctrl_btns = [
            ("Zoom +", Action::AdjustZoom(0.25)),
            ("Zoom -", Action::AdjustZoom(-0.25)),
            (auto_lbl, Action::ToggleAutoRotate),
            (geom_lbl, Action::ToggleWireframe),
            ("⟲ Reset", Action::Reset3DCamera),
        ];

        for (i, (lbl, act)) in ctrl_btns.iter().enumerate() {
            let bx = pad + 14.0 * dp_scale + (i as f32) * (dp_btn_w + 6.0 * dp_scale);
            let is_geom = i == 3 && runtime.wireframe_mode;
            let bg = if is_geom {
                if theme.is_dark {
                    [0.00, 0.85, 0.65, 0.95]
                } else {
                    [0.04, 0.52, 0.38, 1.0]
                }
            } else {
                col_primary
            };
            self.ui_batch
                .push_rect(bx, zoom_y, dp_btn_w, dp_btn_h, 7.0 * dp_scale, bg, 0.0);
            let tw = self.measure_text(lbl, 8.5 * font_scale);
            self.draw_text(
                bx + (dp_btn_w - tw) * 0.5,
                zoom_y + 8.0 * dp_scale,
                lbl,
                8.5 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: bx,
                y: zoom_y,
                w: dp_btn_w,
                h: dp_btn_h,
                action: act.clone(),
            });
        }

        self.draw_text(
            pad + 14.0 * dp_scale,
            zoom_y + dp_btn_h + 8.0 * dp_scale,
            "Zoom Range: 0.4x - 3.5x • Infinite 360° Free Gyroscope Orbit",
            8.5 * font_scale,
            col_muted,
        );

        cur_y += c4_h + 12.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 5: 3D HARDWARE TELEMETRY & DIAGNOSTICS (No Text Overflow)
        // ═════════════════════════════════════════════════════════════════════
        let c5_h = 94.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            c5_h,
            14.0 * dp_scale,
            col_card,
            1.2 * dp_scale,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 10.0 * dp_scale,
            "EGL 1.4 Native 3D Hardware Telemetry",
            12.5 * font_scale,
            col_text,
        );

        let telem_txt1 = format!(
            "Indices: {} • Triangles: {} • Vertices: {}",
            self.mesh3d_index_count,
            self.mesh3d_index_count / 3,
            self.mesh3d_index_count / 3 * 2,
        );
        let telem_col = if theme.is_dark {
            [0.0, 0.85, 0.55, 1.0]
        } else {
            [0.02, 0.48, 0.35, 1.0]
        };
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 28.0 * dp_scale,
            &telem_txt1,
            9.5 * font_scale,
            telem_col,
        );

        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 46.0 * dp_scale,
            "Lighting: Blinn-Phong Specular (s=32) • Fresnel Rim",
            9.0 * font_scale,
            col_accent,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 64.0 * dp_scale,
            "Surface: ANativeWindow EGL 1.4 Direct • Zero-GC Engine",
            9.0 * font_scale,
            col_muted,
        );

        cur_y += c5_h + 12.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 6: SYNTAX-HIGHLIGHTED 3D SPATIAL KERNEL (Dynamic TechScript Code)
        // ═════════════════════════════════════════════════════════════════════
        let (code_fname, code_body) = runtime.get_model_code();
        let num_code_lines = code_body.lines().count();
        let cd6_h =
            44.0 * dp_scale + (num_code_lines as f32) * (14.0 * font_scale) + 36.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            cd6_h,
            14.0 * dp_scale,
            code_bg,
            1.2 * dp_scale,
        );

        self.ui_batch.push_rect(
            pad + 14.0 * dp_scale,
            cur_y + 11.0 * dp_scale,
            7.0 * dp_scale,
            7.0 * dp_scale,
            3.5 * dp_scale,
            [0.94, 0.27, 0.27, 1.0],
            0.0,
        );
        self.ui_batch.push_rect(
            pad + 25.0 * dp_scale,
            cur_y + 11.0 * dp_scale,
            7.0 * dp_scale,
            7.0 * dp_scale,
            3.5 * dp_scale,
            [0.96, 0.62, 0.04, 1.0],
            0.0,
        );
        self.ui_batch.push_rect(
            pad + 36.0 * dp_scale,
            cur_y + 11.0 * dp_scale,
            7.0 * dp_scale,
            7.0 * dp_scale,
            3.5 * dp_scale,
            [0.06, 0.72, 0.50, 1.0],
            0.0,
        );
        let cd6_fn_col = if theme.is_dark {
            [0.85, 0.90, 1.0, 0.9]
        } else {
            col_text
        };
        self.draw_text(
            pad + 54.0 * dp_scale,
            cur_y + 7.5 * dp_scale,
            code_fname,
            10.5 * font_scale,
            cd6_fn_col,
        );

        let badge_sp = "[ 3D SPATIAL KERNEL ]";
        let bsp_w = self.measure_text(badge_sp, 9.0 * font_scale);
        let cd6_badge_col = if theme.is_dark {
            [0.0, 0.90, 1.0, 1.0]
        } else {
            col_accent
        };
        self.draw_text(
            pad + cw - bsp_w - 14.0 * dp_scale,
            cur_y + 8.0 * dp_scale,
            badge_sp,
            9.0 * font_scale,
            cd6_badge_col,
        );

        let cd6_sep_col = if theme.is_dark {
            [0.15, 0.22, 0.45, 0.6]
        } else {
            [0.68, 0.78, 0.92, 0.50]
        };
        self.ui_batch.push_rect(
            pad,
            cur_y + 24.0 * dp_scale,
            cw,
            1.0 * dp_scale,
            0.5 * dp_scale,
            cd6_sep_col,
            0.0,
        );

        self.draw_syntax_code_block(
            pad + 14.0 * dp_scale,
            cur_y + 32.0 * dp_scale,
            code_body,
            9.5 * font_scale,
            4.5 * dp_scale,
            theme.is_dark,
        );

        let run_3d_w = 114.0 * dp_scale;
        let run_3d_h = 24.0 * dp_scale;
        let run_3d_x = pad + cw - run_3d_w - 12.0 * dp_scale;
        let run_3d_y = cur_y + cd6_h - 32.0 * dp_scale;
        let run_3d_bg = if theme.is_dark {
            [0.0, 0.90, 1.0, 0.20]
        } else {
            [0.84, 0.92, 1.0, 0.95]
        };
        let run_3d_fg = if theme.is_dark {
            [0.0, 0.90, 1.0, 1.0]
        } else {
            col_primary
        };
        self.ui_batch.push_rect(
            run_3d_x,
            run_3d_y,
            run_3d_w,
            run_3d_h,
            12.0 * dp_scale,
            run_3d_bg,
            1.0 * dp_scale,
        );
        self.draw_text(
            run_3d_x + 8.0 * dp_scale,
            run_3d_y + 6.0 * dp_scale,
            "▶ EXECUTE KERNEL",
            8.5 * font_scale,
            run_3d_fg,
        );
        self.hit_regions.push(HitRegion {
            x: run_3d_x,
            y: run_3d_y,
            w: run_3d_w,
            h: run_3d_h,
            action: Action::RunQuickAction(format!("tsc run {}", code_fname)),
        });

        cur_y += cd6_h + 12.0 * dp_scale;

        // Footer
        let foot_y = cur_y + 6.0 * dp_scale;
        let foot_text = "TechScript AppForge v2.0.0 • Hardware-Accelerated 3D Engine";
        let foot_w = self.measure_text(foot_text, 9.5 * font_scale);
        self.draw_text(
            pad + (cw - foot_w) * 0.5,
            foot_y,
            foot_text,
            9.5 * font_scale,
            col_muted,
        );

        let total_content_h = (foot_y + 36.0 * dp_scale) - sy;
        runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
    }

    /// Destination 4: Projects Screen
    fn render_projects_content(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        pad: f32,
        cw: f32,
        start_y: f32,
        content_bottom: f32,
    ) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);
        let code_bg = if theme.is_dark {
            [0.03, 0.05, 0.12, 0.98]
        } else {
            [0.91, 0.95, 0.99, 0.96]
        };

        let sy = start_y - runtime.scroll_y;

        if runtime.show_source {
            // Source Code Inspector View & Live Runner
            let prj = &runtime.projects[runtime.selected_project];
            self.draw_text(pad, sy, prj.name, 17.0 * font_scale, col_accent);
            let prj_meta = format!("{} • {} • {} lines", prj.tier, prj.path, prj.lines);
            self.draw_text(
                pad,
                sy + 21.0 * dp_scale,
                &prj_meta,
                9.5 * font_scale,
                col_muted,
            );

            // Back button
            let back_btn_w = 90.0 * dp_scale;
            let back_btn_x = pad + cw - back_btn_w;
            self.ui_batch.push_rect(
                back_btn_x,
                sy,
                back_btn_w,
                26.0 * dp_scale,
                13.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            self.draw_text(
                back_btn_x + 12.0 * dp_scale,
                sy + 7.0 * dp_scale,
                "‹ PROJECTS",
                10.0 * font_scale,
                col_accent,
            );
            self.hit_regions.push(HitRegion {
                x: back_btn_x,
                y: sy,
                w: back_btn_w,
                h: 26.0 * dp_scale,
                action: Action::ToggleCodeView,
            });

            // Action Row: Run Code Live + Validate
            let action_row_y = sy + 38.0 * dp_scale;
            let run_btn_w = 160.0 * dp_scale;

            // Run in Interpreter Button
            let run_btn_bg = if theme.is_dark {
                [0.06, 0.72, 0.45, 0.95]
            } else {
                [0.04, 0.52, 0.35, 1.0]
            };
            self.ui_batch.push_rect(
                pad,
                action_row_y,
                run_btn_w,
                28.0 * dp_scale,
                14.0 * dp_scale,
                run_btn_bg,
                0.0,
            );
            self.draw_text(
                pad + 14.0 * dp_scale,
                action_row_y + 8.0 * dp_scale,
                "▶ RUN IN INTERPRETER",
                9.5 * font_scale,
                [1.0, 1.0, 1.0, 1.0],
            );
            self.hit_regions.push(HitRegion {
                x: pad,
                y: action_row_y,
                w: run_btn_w,
                h: 28.0 * dp_scale,
                action: Action::RunSelectedProject,
            });

            // Validate Button
            let val_btn_w = 130.0 * dp_scale;
            let val_x = pad + run_btn_w + 8.0 * dp_scale;
            self.ui_batch.push_rect(
                val_x,
                action_row_y,
                val_btn_w,
                28.0 * dp_scale,
                14.0 * dp_scale,
                col_card,
                1.2 * dp_scale,
            );
            let val_lbl = if runtime.project_validated {
                "✔ VALID (0 ERR)"
            } else {
                "⚡ CHECK SYNTAX"
            };
            self.draw_text(
                val_x + 12.0 * dp_scale,
                action_row_y + 8.0 * dp_scale,
                val_lbl,
                9.5 * font_scale,
                col_accent,
            );
            self.hit_regions.push(HitRegion {
                x: val_x,
                y: action_row_y,
                w: val_btn_w,
                h: 28.0 * dp_scale,
                action: Action::ValidateProject,
            });

            // Live Output Terminal Card (if code has run)
            let mut next_y = action_row_y + 36.0 * dp_scale;
            if let Some(ref out) = runtime.project_run_result {
                let term_h = 68.0 * dp_scale;
                let term_bg = if theme.is_dark {
                    [0.03, 0.05, 0.10, 0.98]
                } else {
                    [0.90, 0.95, 0.99, 0.96]
                };
                let term_out_col = if theme.is_dark {
                    [0.25, 0.95, 0.55, 1.0]
                } else {
                    [0.02, 0.48, 0.35, 1.0]
                };
                self.ui_batch.push_rect(
                    pad,
                    next_y,
                    cw,
                    term_h,
                    12.0 * dp_scale,
                    term_bg,
                    1.2 * dp_scale,
                );
                self.draw_text(
                    pad + 12.0 * dp_scale,
                    next_y + 8.0 * dp_scale,
                    "Terminal Execution Console (Realtime Log):",
                    9.0 * font_scale,
                    col_accent,
                );
                self.draw_text_wrapped(
                    pad + 12.0 * dp_scale,
                    next_y + 24.0 * dp_scale,
                    cw - 24.0 * dp_scale,
                    out,
                    10.0 * font_scale,
                    term_out_col,
                    3.0 * dp_scale,
                );
                next_y += term_h + 10.0 * dp_scale;
            }

            // Code Editor Box
            let line_count = prj.code_sample.lines().count();
            let code_h = (line_count as f32 * 18.0 + 36.0) * dp_scale;
            self.ui_batch.push_rect(
                pad,
                next_y,
                cw,
                code_h,
                14.0 * dp_scale,
                code_bg,
                1.4 * dp_scale,
            );

            // Mac window dots
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                next_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.94, 0.27, 0.27, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 25.0 * dp_scale,
                next_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.96, 0.62, 0.04, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 36.0 * dp_scale,
                next_y + 11.0 * dp_scale,
                7.0 * dp_scale,
                7.0 * dp_scale,
                3.5 * dp_scale,
                [0.06, 0.72, 0.50, 1.0],
                0.0,
            );
            let path_col = if theme.is_dark {
                [0.85, 0.90, 1.0, 0.9]
            } else {
                col_text
            };
            self.draw_text(
                pad + 52.0 * dp_scale,
                next_y + 8.0 * dp_scale,
                prj.path,
                10.0 * font_scale,
                path_col,
            );

            let sep_col = if theme.is_dark {
                [0.15, 0.22, 0.45, 0.6]
            } else {
                [0.68, 0.78, 0.92, 0.50]
            };
            self.ui_batch.push_rect(
                pad,
                next_y + 24.0 * dp_scale,
                cw,
                1.0 * dp_scale,
                0.5 * dp_scale,
                sep_col,
                0.0,
            );

            self.draw_syntax_code_block(
                pad + 14.0 * dp_scale,
                next_y + 32.0 * dp_scale,
                prj.code_sample,
                10.0 * font_scale,
                4.0 * dp_scale,
                theme.is_dark,
            );

            let total_content_h = next_y + code_h + 20.0 * dp_scale - sy;
            runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
        } else {
            // Projects List View
            self.draw_text(
                pad,
                sy,
                "TechScript Projects Ecosystem",
                17.5 * font_scale,
                col_accent,
            );
            self.draw_text(
                pad,
                sy + 21.0 * dp_scale,
                "Production-grade systems, engines, & kernels written in pure .txs",
                9.5 * font_scale,
                col_muted,
            );

            // Category Filter Bar
            let filter_y = sy + 38.0 * dp_scale;
            let filter_cats = ["All (8)", "Systems", "Cloud & Net", "AI & ML", "Mobile"];
            let num_filters = filter_cats.len();
            let filter_w =
                (cw - ((num_filters - 1) as f32) * 6.0 * dp_scale) / (num_filters as f32);
            let filter_h = 28.0 * dp_scale;

            for (i, cat_name) in filter_cats.iter().enumerate() {
                let fx = pad + (i as f32) * (filter_w + 6.0 * dp_scale);
                let is_sel = runtime.project_filter == i;
                let bg = if is_sel { col_primary } else { col_card };
                let fg = if is_sel {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    col_muted
                };
                self.ui_batch.push_rect(
                    fx,
                    filter_y,
                    filter_w,
                    filter_h,
                    8.0 * dp_scale,
                    bg,
                    if is_sel {
                        1.4 * dp_scale
                    } else {
                        1.0 * dp_scale
                    },
                );
                let tw = self.measure_text(cat_name, 9.0 * font_scale);
                self.draw_text(
                    fx + (filter_w - tw) * 0.5,
                    filter_y + 8.0 * dp_scale,
                    cat_name,
                    9.0 * font_scale,
                    fg,
                );
                self.hit_regions.push(HitRegion {
                    x: fx,
                    y: filter_y,
                    w: filter_w,
                    h: filter_h,
                    action: Action::SelectProjectFilter(i),
                });
            }

            let mut list_y = filter_y + filter_h + 12.0 * dp_scale;
            let item_h = 138.0 * dp_scale;
            let card_spacing = 14.0 * dp_scale;

            // Render all projects matching filter
            for (i, prj) in runtime.projects.iter().enumerate() {
                let matches_filter = match runtime.project_filter {
                    1 => prj.category == "Systems",
                    2 => prj.category == "Cloud & Net",
                    3 => prj.category == "AI & ML",
                    4 => prj.category == "Mobile",
                    _ => true,
                };
                if !matches_filter {
                    continue;
                }

                // Generous card background with distance
                self.ui_batch.push_rect(
                    pad,
                    list_y,
                    cw,
                    item_h,
                    16.0 * dp_scale,
                    col_card,
                    1.4 * dp_scale,
                );

                // Row 1: Title (left) & Status Badge (right)
                self.draw_text(
                    pad + 14.0 * dp_scale,
                    list_y + 12.0 * dp_scale,
                    prj.name,
                    13.0 * font_scale,
                    col_text,
                );
                let status_badge = format!("[ {} ]", prj.status);
                let sb_w = self.measure_text(&status_badge, 9.5 * font_scale);
                let badge_col = if theme.is_dark {
                    [0.0, 0.90, 0.60, 1.0]
                } else {
                    [0.04, 0.52, 0.35, 1.0]
                };
                self.draw_text(
                    pad + cw - sb_w - 14.0 * dp_scale,
                    list_y + 12.0 * dp_scale,
                    &status_badge,
                    9.5 * font_scale,
                    badge_col,
                );

                // Row 2: Metadata (Tier • Lines • Category • Path)
                let meta_str = format!(
                    "{} • {} • {} lines • {}",
                    prj.tier, prj.category, prj.lines, prj.path
                );
                self.draw_text(
                    pad + 14.0 * dp_scale,
                    list_y + 30.0 * dp_scale,
                    &meta_str,
                    9.0 * font_scale,
                    col_accent,
                );

                // Row 3: Description (Wrapped in 2 lines with proper spacing)
                let desc_col = if theme.is_dark {
                    [0.72, 0.78, 0.90, 0.90]
                } else {
                    col_muted
                };
                self.draw_text_wrapped(
                    pad + 14.0 * dp_scale,
                    list_y + 48.0 * dp_scale,
                    cw - 28.0 * dp_scale,
                    prj.description,
                    9.5 * font_scale,
                    desc_col,
                    3.5 * dp_scale,
                );

                // Row 4: Workable Buttons Row: [ ▶ RUN CODE ] [ 🔍 SOURCE ] [ ⚡ CHECK AST ]
                let btn_y = list_y + item_h - 34.0 * dp_scale;
                let btn_h = 26.0 * dp_scale;
                let btn_gap = 6.0 * dp_scale;
                let avail_w = cw - 28.0 * dp_scale;
                let btn_w = (avail_w - 2.0 * btn_gap) / 3.0;

                // Button 1: Run Project Direct (immediately executes and opens inspector)
                let b1_x = pad + 14.0 * dp_scale;
                let b1_bg = if theme.is_dark {
                    [0.06, 0.72, 0.45, 0.95]
                } else {
                    [0.04, 0.52, 0.35, 1.0]
                };
                self.ui_batch
                    .push_rect(b1_x, btn_y, btn_w, btn_h, 13.0 * dp_scale, b1_bg, 0.0);
                let b1_text = "▶ RUN CODE";
                let b1_tw = self.measure_text(b1_text, 8.5 * font_scale);
                self.draw_text(
                    b1_x + (btn_w - b1_tw) * 0.5,
                    btn_y + 7.5 * dp_scale,
                    b1_text,
                    8.5 * font_scale,
                    [1.0, 1.0, 1.0, 1.0],
                );
                self.hit_regions.push(HitRegion {
                    x: b1_x,
                    y: btn_y,
                    w: btn_w,
                    h: btn_h,
                    action: Action::RunProjectDirect(i),
                });

                // Button 2: View Source Code
                let b2_x = b1_x + btn_w + btn_gap;
                let b2_bg = if theme.is_dark {
                    [0.0, 0.85, 1.0, 0.18]
                } else {
                    [0.84, 0.92, 1.0, 0.95]
                };
                let b2_fg = if theme.is_dark {
                    col_accent
                } else {
                    col_primary
                };
                self.ui_batch.push_rect(
                    b2_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    13.0 * dp_scale,
                    b2_bg,
                    1.0 * dp_scale,
                );
                let b2_text = "🔍 SOURCE";
                let b2_tw = self.measure_text(b2_text, 8.5 * font_scale);
                self.draw_text(
                    b2_x + (btn_w - b2_tw) * 0.5,
                    btn_y + 7.5 * dp_scale,
                    b2_text,
                    8.5 * font_scale,
                    b2_fg,
                );
                self.hit_regions.push(HitRegion {
                    x: b2_x,
                    y: btn_y,
                    w: btn_w,
                    h: btn_h,
                    action: Action::SelectProject(i),
                });

                // Button 3: Check AST (Interactive Validation with instant visual state feedback)
                let b3_x = b2_x + btn_w + btn_gap;
                let is_val = runtime.validated_projects.get(i).copied().unwrap_or(false);
                let val_bg = if is_val {
                    if theme.is_dark {
                        [0.06, 0.72, 0.45, 0.95]
                    } else {
                        [0.04, 0.52, 0.35, 1.0]
                    }
                } else {
                    if theme.is_dark {
                        [0.12, 0.16, 0.28, 0.85]
                    } else {
                        [0.84, 0.91, 0.98, 0.95]
                    }
                };
                let val_fg = if is_val {
                    [1.0, 1.0, 1.0, 1.0]
                } else {
                    if theme.is_dark {
                        [0.90, 0.94, 1.0, 0.85]
                    } else {
                        col_primary
                    }
                };
                let val_text = if is_val {
                    "✔ VALID AST"
                } else {
                    "⚡ CHECK AST"
                };
                self.ui_batch.push_rect(
                    b3_x,
                    btn_y,
                    btn_w,
                    btn_h,
                    13.0 * dp_scale,
                    val_bg,
                    1.0 * dp_scale,
                );
                let b3_tw = self.measure_text(val_text, 8.5 * font_scale);
                self.draw_text(
                    b3_x + (btn_w - b3_tw) * 0.5,
                    btn_y + 7.5 * dp_scale,
                    val_text,
                    8.5 * font_scale,
                    val_fg,
                );
                self.hit_regions.push(HitRegion {
                    x: b3_x,
                    y: btn_y,
                    w: btn_w,
                    h: btn_h,
                    action: Action::ValidateProjectDirect(i),
                });

                list_y += item_h + card_spacing;
            }

            // Footer
            let foot_y = list_y + 6.0 * dp_scale;
            let foot_text = "TechScript AppForge v2.0.0 • Verified Real Projects";
            let foot_w = self.measure_text(foot_text, 9.5 * font_scale);
            self.draw_text(
                pad + (cw - foot_w) * 0.5,
                foot_y,
                foot_text,
                9.5 * font_scale,
                col_muted,
            );

            let total_content_h = foot_y + 36.0 * dp_scale - sy;
            runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
        }
    }

    /// Destination 5: Docs Screen
    fn render_docs_content(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        pad: f32,
        cw: f32,
        start_y: f32,
        content_bottom: f32,
    ) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);
        let col_violet = Theme::to_f32_color(theme.violet);

        let sy = start_y - runtime.scroll_y;

        // Header Title
        self.draw_text(
            pad,
            sy,
            "Documentation & Architecture Portals",
            17.5 * font_scale,
            col_accent,
        );
        self.draw_text(
            pad,
            sy + 21.0 * dp_scale,
            "Official verified language specification, stdlib, & repository resources",
            9.5 * font_scale,
            col_muted,
        );

        let mut cur_y = sy + 38.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 1: OFFICIAL ECOSYSTEM, CREATOR & DOCUMENTATION HUBS
        // ═════════════════════════════════════════════════════════════════════
        let link_card_h = 224.0 * dp_scale;
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            link_card_h,
            16.0 * dp_scale,
            col_card,
            1.4 * dp_scale,
        );

        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 11.0 * dp_scale,
            "🌐 Official Ecosystem & Documentation Portals",
            12.0 * font_scale,
            col_text,
        );
        self.draw_text(
            pad + 14.0 * dp_scale,
            cur_y + 27.0 * dp_scale,
            "Created by Tanmoy (@Tcode-Motion) • Tap to open in browser:",
            9.0 * font_scale,
            col_muted,
        );

        let link_grid_y = cur_y + 44.0 * dp_scale;
        let l_btn_w = (cw - 28.0 * dp_scale - 8.0 * dp_scale) * 0.5;
        let l_btn_h = 52.0 * dp_scale;

        let links: &[(&str, &str, &str, [f32; 4], [f32; 4])] = &[
            // Row 1: GitHub & Creator
            (
                "⭐️ GitHub Repo",
                "github.com/Tcode-Motion/techscript",
                "https://github.com/Tcode-Motion/techscript",
                if theme.is_dark { [0.10, 0.14, 0.24, 0.90] } else { [0.86, 0.92, 1.0, 0.92] },
                if theme.is_dark { col_accent } else { col_primary },
            ),
            (
                "👤 Creator Profile",
                "Tanmoy (@Tcode-Motion)",
                "https://github.com/Tcode-Motion",
                if theme.is_dark { [0.16, 0.12, 0.28, 0.90] } else { [0.92, 0.88, 1.0, 0.92] },
                if theme.is_dark { [0.85, 0.65, 1.0, 1.0] } else { col_violet },
            ),
            // Row 2: Official Docs & Master Spec
            (
                "📖 Official Docs",
                "techscript.is-a.dev/techscript",
                "https://techscript.is-a.dev/techscript/",
                if theme.is_dark { [0.06, 0.40, 0.65, 0.40] } else { [0.85, 0.94, 1.0, 0.92] },
                if theme.is_dark { [0.0, 0.90, 1.0, 1.0] } else { col_accent },
            ),
            (
                "📜 Canonical Master Spec",
                "Syntax Master Specification",
                "https://github.com/Tcode-Motion/techscript/blob/main/TechScript_Final_Syntax_and_Language_Master_Spec.md",
                if theme.is_dark { [0.12, 0.22, 0.35, 0.60] } else { [0.86, 0.96, 0.92, 0.92] },
                if theme.is_dark { [0.20, 0.95, 0.70, 1.0] } else { [0.04, 0.52, 0.35, 1.0] },
            ),
            // Row 3: StdLib Reference & AppForge Mobile Engine
            (
                "📦 StdLib Reference",
                "docs/stdlib (math, text, sys)",
                "https://github.com/Tcode-Motion/techscript/tree/main/docs/stdlib",
                if theme.is_dark { [0.18, 0.16, 0.35, 0.60] } else { [0.91, 0.89, 0.99, 0.92] },
                if theme.is_dark { [0.85, 0.65, 1.0, 1.0] } else { col_violet },
            ),
            (
                "🚀 AppForge Mobile",
                "Pure Rust NDK Runtime",
                "https://github.com/Tcode-Motion/techscript/tree/main/examples/appforge",
                if theme.is_dark { [0.05, 0.30, 0.25, 0.80] } else { [0.85, 0.95, 0.92, 0.92] },
                if theme.is_dark { [0.0, 0.95, 0.60, 1.0] } else { [0.02, 0.48, 0.35, 1.0] },
            ),
        ];

        for (i, (title, sub, url, bg, fg)) in links.iter().enumerate() {
            let col_idx = i % 2;
            let row_idx = i / 2;
            let lx = pad + 14.0 * dp_scale + (col_idx as f32) * (l_btn_w + 8.0 * dp_scale);
            let ly = link_grid_y + (row_idx as f32) * (l_btn_h + 6.0 * dp_scale);

            self.ui_batch.push_rect(
                lx,
                ly,
                l_btn_w,
                l_btn_h,
                10.0 * dp_scale,
                *bg,
                1.2 * dp_scale,
            );
            self.draw_text(
                lx + 10.0 * dp_scale,
                ly + 9.0 * dp_scale,
                title,
                10.5 * font_scale,
                *fg,
            );
            self.draw_text(
                lx + 10.0 * dp_scale,
                ly + 28.0 * dp_scale,
                sub,
                8.5 * font_scale,
                col_muted,
            );
            self.draw_text(
                lx + l_btn_w - 20.0 * dp_scale,
                ly + 18.0 * dp_scale,
                "↗",
                12.0 * font_scale,
                *fg,
            );

            self.hit_regions.push(HitRegion {
                x: lx,
                y: ly,
                w: l_btn_w,
                h: l_btn_h,
                action: Action::OpenLink((*url).to_string()),
            });
        }

        cur_y += link_card_h + 14.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 2: DOCUMENTATION TOPICS SELECTOR (Clean 4x2 Grid)
        // ═════════════════════════════════════════════════════════════════════
        let pill_h = 26.0 * dp_scale;
        let num_topics = runtime.doc_topics.len();
        let cols = 4;
        let pill_gap = 6.0 * dp_scale;
        let pill_w = (cw - ((cols - 1) as f32) * pill_gap) / (cols as f32);

        for (i, topic) in runtime.doc_topics.iter().enumerate() {
            let col_i = i % cols;
            let row_i = i / cols;
            let px = pad + (col_i as f32) * (pill_w + pill_gap);
            let py = cur_y + (row_i as f32) * (pill_h + 6.0 * dp_scale);
            let is_sel = runtime.selected_doc_topic == i;
            let bg = if is_sel { col_primary } else { col_card };
            let fg = if is_sel {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                col_muted
            };
            self.ui_batch.push_rect(
                px,
                py,
                pill_w,
                pill_h,
                8.0 * dp_scale,
                bg,
                if is_sel {
                    1.4 * dp_scale
                } else {
                    1.0 * dp_scale
                },
            );
            let t_w = self.measure_text(topic.category, 8.5 * font_scale);
            self.draw_text(
                px + (pill_w - t_w) * 0.5,
                py + 7.5 * dp_scale,
                topic.category,
                8.5 * font_scale,
                fg,
            );
            self.hit_regions.push(HitRegion {
                x: px,
                y: py,
                w: pill_w,
                h: pill_h,
                action: Action::SelectDocTopic(i),
            });
        }

        let total_rows = (num_topics + cols - 1) / cols;
        cur_y += (total_rows as f32) * (pill_h + 6.0 * dp_scale) + 8.0 * dp_scale;

        // ═════════════════════════════════════════════════════════════════════
        // CARD 3: ACTIVE TOPIC SPECIFICATION & CODE GUIDE
        // ═════════════════════════════════════════════════════════════════════
        let topic = &runtime.doc_topics[runtime.selected_doc_topic];

        // Measure content lines
        let body_lines =
            self.font_atlas
                .wrap_text(topic.content, cw - 32.0 * dp_scale, 10.5 * font_scale);
        let body_h = body_lines.len() as f32 * 16.0 * dp_scale;

        let code_lines = topic.code_snippet.map(|s| s.lines().count()).unwrap_or(0);
        let snip_h = if code_lines > 0 {
            (code_lines as f32 * 16.0 + 36.0) * dp_scale
        } else {
            0.0
        };

        let doc_card_h =
            (54.0 * dp_scale + body_h + snip_h + 20.0 * dp_scale).max(200.0 * dp_scale);
        self.ui_batch.push_rect(
            pad,
            cur_y,
            cw,
            doc_card_h,
            16.0 * dp_scale,
            col_card,
            1.4 * dp_scale,
        );

        self.draw_text(
            pad + 16.0 * dp_scale,
            cur_y + 12.0 * dp_scale,
            topic.title,
            13.5 * font_scale,
            col_text,
        );
        let cat_badge = format!("[ {} ]", topic.category);
        let cat_w = self.measure_text(&cat_badge, 10.0 * font_scale);
        self.draw_text(
            pad + cw - cat_w - 16.0 * dp_scale,
            cur_y + 12.0 * dp_scale,
            &cat_badge,
            10.0 * font_scale,
            col_accent,
        );

        // Content Body
        let mut text_y = cur_y + 36.0 * dp_scale;
        let content_fg = if theme.is_dark {
            [0.82, 0.88, 0.96, 0.95]
        } else {
            col_text
        };
        for line in &body_lines {
            self.draw_text(
                pad + 16.0 * dp_scale,
                text_y,
                line,
                10.0 * font_scale,
                content_fg,
            );
            text_y += 16.0 * dp_scale;
        }

        // Code Snippet with syntax block
        if let Some(snippet) = topic.code_snippet {
            text_y += 8.0 * dp_scale;
            let code_box_w = cw - 28.0 * dp_scale;
            let snip_bg = if theme.is_dark {
                [0.04, 0.06, 0.12, 0.98]
            } else {
                [0.91, 0.95, 0.99, 0.96]
            };
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                text_y,
                code_box_w,
                snip_h,
                12.0 * dp_scale,
                snip_bg,
                1.2 * dp_scale,
            );

            // Mac dots
            self.ui_batch.push_rect(
                pad + 24.0 * dp_scale,
                text_y + 9.0 * dp_scale,
                6.0 * dp_scale,
                6.0 * dp_scale,
                3.0 * dp_scale,
                [0.94, 0.27, 0.27, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 34.0 * dp_scale,
                text_y + 9.0 * dp_scale,
                6.0 * dp_scale,
                6.0 * dp_scale,
                3.0 * dp_scale,
                [0.96, 0.62, 0.04, 1.0],
                0.0,
            );
            self.ui_batch.push_rect(
                pad + 44.0 * dp_scale,
                text_y + 9.0 * dp_scale,
                6.0 * dp_scale,
                6.0 * dp_scale,
                3.0 * dp_scale,
                [0.06, 0.72, 0.50, 1.0],
                0.0,
            );
            self.draw_text(
                pad + 58.0 * dp_scale,
                text_y + 6.0 * dp_scale,
                "example.txs",
                9.0 * font_scale,
                col_muted,
            );

            let snip_sep = if theme.is_dark {
                [0.15, 0.22, 0.45, 0.5]
            } else {
                [0.68, 0.78, 0.92, 0.50]
            };
            self.ui_batch.push_rect(
                pad + 14.0 * dp_scale,
                text_y + 20.0 * dp_scale,
                code_box_w,
                1.0 * dp_scale,
                0.5 * dp_scale,
                snip_sep,
                0.0,
            );

            self.draw_syntax_code_block(
                pad + 24.0 * dp_scale,
                text_y + 26.0 * dp_scale,
                snippet,
                9.5 * font_scale,
                4.0 * dp_scale,
                theme.is_dark,
            );
        }

        cur_y += doc_card_h + 14.0 * dp_scale;

        // Footer
        let foot_y = cur_y + 6.0 * dp_scale;
        let foot_text = "TechScript AppForge v2.0.0 • Canonical Documentation";
        let foot_w = self.measure_text(foot_text, 9.5 * font_scale);
        self.draw_text(
            pad + (cw - foot_w) * 0.5,
            foot_y,
            foot_text,
            9.5 * font_scale,
            col_muted,
        );

        let total_content_h = foot_y + 36.0 * dp_scale - sy;
        runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
    }

    /// Secondary Screen: About / Diagnostics Screen
    fn render_about_content(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        pad: f32,
        cw: f32,
        start_y: f32,
        content_bottom: f32,
    ) {
        let col_text = Theme::to_f32_color(theme.text);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_card = Theme::to_f32_color(theme.card);
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_accent = Theme::to_f32_color(theme.accent);

        let sy = start_y - runtime.scroll_y;

        self.draw_text(
            pad,
            sy,
            "Official Verification & Developer",
            17.0 * font_scale,
            col_primary,
        );
        self.draw_text(
            pad,
            sy + 20.0 * dp_scale,
            "Native architecture audit and verified links",
            10.5 * font_scale,
            col_muted,
        );

        // Back button
        let back_btn_w = 76.0 * dp_scale;
        let back_btn_x = pad + cw - back_btn_w;
        self.ui_batch.push_rect(
            back_btn_x,
            sy,
            back_btn_w,
            26.0 * dp_scale,
            13.0 * dp_scale,
            col_card,
            1.0,
        );
        self.draw_text(
            back_btn_x + 10.0 * dp_scale,
            sy + 7.0 * dp_scale,
            "< CLOSE",
            10.5 * font_scale,
            col_accent,
        );
        self.hit_regions.push(HitRegion {
            x: back_btn_x,
            y: sy,
            w: back_btn_w,
            h: 26.0 * dp_scale,
            action: Action::Navigate("home".into()),
        });

        // Developer Profile Card (from about.txs)
        let prof_y = sy + 38.0 * dp_scale;
        let prof_h = 96.0 * dp_scale;
        self.ui_batch
            .push_rect(pad, prof_y, cw, prof_h, 16.0 * dp_scale, col_card, 1.2);

        self.ui_batch.draw_techscript_logo_mark(
            pad + 24.0 * dp_scale,
            prof_y + 30.0 * dp_scale,
            28.0 * dp_scale,
        );
        self.draw_text(
            pad + 48.0 * dp_scale,
            prof_y + 12.0 * dp_scale,
            &runtime.bridge.about_data.app_name,
            14.0 * font_scale,
            col_text,
        );
        let dev_str = format!(
            "Creator: {} (@{})",
            runtime.bridge.about_data.creator, runtime.bridge.about_data.github_identity
        );
        self.draw_text(
            pad + 48.0 * dp_scale,
            prof_y + 30.0 * dp_scale,
            &dev_str,
            11.0 * font_scale,
            col_accent,
        );
        let lic_str = format!(
            "License: {} • Spec: TechScript 2.0",
            runtime.bridge.about_data.license
        );
        self.draw_text(
            pad + 48.0 * dp_scale,
            prof_y + 48.0 * dp_scale,
            &lic_str,
            10.0 * font_scale,
            col_muted,
        );
        self.draw_text(
            pad + 48.0 * dp_scale,
            prof_y + 64.0 * dp_scale,
            &runtime.bridge.about_data.developer_contact,
            10.0 * font_scale,
            col_primary,
        );

        // Architectural Verification Card
        let audit_y = prof_y + prof_h + 10.0 * dp_scale;
        let audit_h = 160.0 * dp_scale;
        self.ui_batch
            .push_rect(pad, audit_y, cw, audit_h, 16.0 * dp_scale, col_card, 1.0);

        let diag_text = format!(
            "Resolution: {}x{} • Scale: {:.2} • Frames: {}\nRenderer: Hardware EGL 1.4 + OpenGL ES 2.0 (GLES2)\nEntry Point: Android OS NativeActivity (0% Java/Kotlin)\nTrueType Engine: fontdue Roboto TrueType (512x512 atlas)\nArchitecture: arm64-v8a Direct Native Execution\nLanguage Engine: Real .txs Lexer, Parser & VM Active\nPackage ID: com.techscript.appforge",
            self.width, self.height, dp_scale, runtime.frame_count
        );
        self.draw_text_wrapped(
            pad + 16.0 * dp_scale,
            audit_y + 14.0 * dp_scale,
            cw - 32.0 * dp_scale,
            &diag_text,
            10.5 * font_scale,
            col_text,
            4.0 * dp_scale,
        );

        // Verified Links Grid (loaded from about.txs)
        let mut link_row_y = audit_y + audit_h + 10.0 * dp_scale;
        let item_h = 36.0 * dp_scale;

        for link in &runtime.bridge.about_data.verified_links {
            let link_bg = if theme.is_dark {
                [0.15, 0.18, 0.28, 0.85]
            } else {
                [0.88, 0.94, 1.0, 0.95]
            };
            self.ui_batch
                .push_rect(pad, link_row_y, cw, item_h, 10.0 * dp_scale, link_bg, 1.0);
            self.draw_text(
                pad + 14.0 * dp_scale,
                link_row_y + 10.0 * dp_scale,
                &link.label,
                11.5 * font_scale,
                col_text,
            );
            self.draw_text(
                pad + cw - 28.0 * dp_scale,
                link_row_y + 10.0 * dp_scale,
                "↗",
                12.0 * font_scale,
                col_accent,
            );

            self.hit_regions.push(HitRegion {
                x: pad,
                y: link_row_y,
                w: cw,
                h: item_h,
                action: Action::OpenLink(link.url.clone()),
            });

            link_row_y += item_h + 6.0 * dp_scale;
        }

        let total_content_h = link_row_y + 16.0 * dp_scale - sy;
        runtime.max_scroll_y = (total_content_h - (content_bottom - start_y)).max(0.0);
    }

    /// Persistent Bottom Floating Liquid Glass Navigation Dock (5 Destinations)
    fn render_navigation_dock(
        &mut self,
        runtime: &mut TechScriptRuntime,
        theme: &Theme,
        dp_scale: f32,
        font_scale: f32,
        _dock_clearance: f32,
    ) {
        let sw = self.width as f32;
        let sh = self.height as f32;

        let dock_w = (sw - 20.0 * dp_scale).min(520.0 * dp_scale);
        let dock_h = 66.0 * dp_scale;
        let dock_x = (sw - dock_w) * 0.5;
        let dock_margin = 12.0 * dp_scale;
        let dock_y = sh - dock_h - dock_margin;

        let col_dock = Theme::to_f32_color(theme.dock_bg);
        let col_active = if theme.is_dark {
            [0.18, 0.24, 0.48, 0.65]
        } else {
            Theme::to_f32_color(theme.dock_active)
        };
        let col_primary = Theme::to_f32_color(theme.primary);
        let col_muted = Theme::to_f32_color(theme.text_muted);
        let col_cyan = Theme::to_f32_color(theme.accent);
        let col_magenta = Theme::to_f32_color(theme.magenta);
        let col_violet = Theme::to_f32_color(theme.violet);
        let col_text = Theme::to_f32_color(theme.text);

        // Dock background pill with specular border highlight
        self.ui_batch.push_rect(
            dock_x,
            dock_y,
            dock_w,
            dock_h,
            dock_h * 0.5,
            col_dock,
            1.2 * dp_scale,
        );

        let tabs = [
            ("home", "Home"),
            ("explorer", "Explorer"),
            ("labs", "Labs"),
            ("projects", "Projects"),
            ("docs", "Docs"),
        ];

        let tab_w = dock_w / 5.0;
        let active_screen = runtime.active_screen.clone();

        // Calculate active index and smooth sliding target
        let mut active_idx = 0;
        for (i, (screen_id, _)) in tabs.iter().enumerate() {
            if *screen_id == active_screen.as_str() {
                active_idx = i;
                break;
            }
        }

        let target_x = dock_x + (active_idx as f32) * tab_w;
        if !runtime.dock_initialized {
            runtime.dock_anim_x = target_x;
            runtime.dock_initialized = true;
        } else {
            let diff = target_x - runtime.dock_anim_x;
            if diff.abs() < 0.4 {
                runtime.dock_anim_x = target_x;
            } else {
                runtime.dock_anim_x += diff * 0.35;
            }
        }

        // Active refined glowing pill highlight (sliding smoothly across tabs, zero bottom overlap)
        let active_pad_x = 4.0 * dp_scale;
        let active_pad_y = 4.0 * dp_scale;
        let pill_w = tab_w - active_pad_x * 2.0;
        let pill_h = dock_h - 16.0 * dp_scale;
        let pill_y = dock_y + active_pad_y;
        self.ui_batch.push_rect(
            runtime.dock_anim_x + active_pad_x,
            pill_y,
            pill_w,
            pill_h,
            15.0 * dp_scale,
            col_active,
            if theme.is_dark { 0.0 } else { 1.0 * dp_scale },
        );

        // Luminous bottom indicator bar sliding with active tab cleanly spaced
        let bar_w = 20.0 * dp_scale;
        let bar_h = 2.5 * dp_scale;
        let bar_x = runtime.dock_anim_x + (tab_w - bar_w) * 0.5;
        let bar_y = dock_y + dock_h - 6.5 * dp_scale;
        let col_indicator = if theme.is_dark { col_cyan } else { col_primary };
        self.ui_batch.push_rect(
            bar_x,
            bar_y,
            bar_w,
            bar_h,
            1.25 * dp_scale,
            col_indicator,
            0.0,
        );

        for (i, (screen_id, label)) in tabs.iter().enumerate() {
            let tx = dock_x + (i as f32) * tab_w;
            let tab_center = tx + tab_w * 0.5;
            let anim_center = runtime.dock_anim_x + tab_w * 0.5;
            let dist_norm = ((tab_center - anim_center) / tab_w).abs().min(1.0);
            let active_weight = (1.0 - dist_norm).clamp(0.0, 1.0);

            // Dynamic color interpolation matching gliding indicator
            let fg = [
                col_muted[0] + (col_text[0] - col_muted[0]) * active_weight,
                col_muted[1] + (col_text[1] - col_muted[1]) * active_weight,
                col_muted[2] + (col_text[2] - col_muted[2]) * active_weight,
                1.0,
            ];
            let icon_col = [
                col_muted[0] + (col_primary[0] - col_muted[0]) * active_weight,
                col_muted[1] + (col_primary[1] - col_muted[1]) * active_weight,
                col_muted[2] + (col_primary[2] - col_muted[2]) * active_weight,
                1.0,
            ];

            let icon_cx = tx + tab_w * 0.5;
            let icon_cy = dock_y + 19.0 * dp_scale;
            let icon_sz = 19.0 * dp_scale;

            // Render crisp procedural vector icon
            let home_door_bg = if active_weight > 0.5 {
                col_active
            } else {
                col_dock
            };
            match *screen_id {
                "home" => {
                    self.ui_batch
                        .draw_icon_home(icon_cx, icon_cy, icon_sz, icon_col, home_door_bg)
                }
                "explorer" => self.ui_batch.draw_icon_explorer(
                    icon_cx,
                    icon_cy,
                    icon_sz,
                    fg,
                    if active_weight > 0.5 { col_cyan } else { fg },
                ),
                "labs" => self.ui_batch.draw_icon_labs(
                    icon_cx,
                    icon_cy,
                    icon_sz,
                    fg,
                    if active_weight > 0.5 { col_magenta } else { fg },
                ),
                "projects" => self.ui_batch.draw_icon_projects(
                    icon_cx,
                    icon_cy,
                    icon_sz,
                    fg,
                    if active_weight > 0.5 { col_violet } else { fg },
                ),
                "docs" => self.ui_batch.draw_icon_docs(
                    icon_cx,
                    icon_cy,
                    icon_sz,
                    if active_weight > 0.5 { col_cyan } else { fg },
                ),
                _ => {}
            }

            let l_w = self.measure_text(label, 10.0 * font_scale);
            self.draw_text(
                tx + (tab_w - l_w) * 0.5,
                dock_y + 40.0 * dp_scale,
                label,
                10.0 * font_scale,
                fg,
            );

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

    unsafe fn draw_ui_batch(&self) {
        if self.ui_batch.indices.is_empty() {
            return;
        }

        glUseProgram(self.ui_program);

        let u_screen_size = glGetUniformLocation(self.ui_program, c_str(b"u_screen_size\0"));
        glUniform2f(u_screen_size, self.width as f32, self.height as f32);

        let u_font_texture = glGetUniformLocation(self.ui_program, c_str(b"u_font_texture\0"));
        glUniform1i(u_font_texture, 0);

        glBindTexture(GL_TEXTURE_2D, self.font_texture);

        // Upload vertex & index data
        glBindBuffer(GL_ARRAY_BUFFER, self.ui_vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            (self.ui_batch.vertices.len() * std::mem::size_of::<crate::glass_ui::UIVertex>())
                as isize,
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

        let a_pos = glGetAttribLocation(self.ui_program, c_str(b"a_pos\0")) as u32;
        let a_uv = glGetAttribLocation(self.ui_program, c_str(b"a_uv\0")) as u32;
        let a_color = glGetAttribLocation(self.ui_program, c_str(b"a_color\0")) as u32;
        let a_local_pos = glGetAttribLocation(self.ui_program, c_str(b"a_local_pos\0")) as u32;
        let a_size = glGetAttribLocation(self.ui_program, c_str(b"a_size\0")) as u32;
        let a_radius = glGetAttribLocation(self.ui_program, c_str(b"a_radius\0")) as u32;
        let a_border_width =
            glGetAttribLocation(self.ui_program, c_str(b"a_border_width\0")) as u32;
        let a_is_text = glGetAttribLocation(self.ui_program, c_str(b"a_is_text\0")) as u32;

        let stride = std::mem::size_of::<crate::glass_ui::UIVertex>() as i32;

        glEnableVertexAttribArray(a_pos);
        glVertexAttribPointer(a_pos, 2, GL_FLOAT, 0, stride, std::ptr::null());

        glEnableVertexAttribArray(a_uv);
        glVertexAttribPointer(a_uv, 2, GL_FLOAT, 0, stride, 8 as *const std::ffi::c_void);

        glEnableVertexAttribArray(a_color);
        glVertexAttribPointer(
            a_color,
            4,
            GL_FLOAT,
            0,
            stride,
            16 as *const std::ffi::c_void,
        );

        glEnableVertexAttribArray(a_local_pos);
        glVertexAttribPointer(
            a_local_pos,
            2,
            GL_FLOAT,
            0,
            stride,
            32 as *const std::ffi::c_void,
        );

        glEnableVertexAttribArray(a_size);
        glVertexAttribPointer(
            a_size,
            2,
            GL_FLOAT,
            0,
            stride,
            40 as *const std::ffi::c_void,
        );

        glEnableVertexAttribArray(a_radius);
        glVertexAttribPointer(
            a_radius,
            1,
            GL_FLOAT,
            0,
            stride,
            48 as *const std::ffi::c_void,
        );

        glEnableVertexAttribArray(a_border_width);
        glVertexAttribPointer(
            a_border_width,
            1,
            GL_FLOAT,
            0,
            stride,
            52 as *const std::ffi::c_void,
        );

        glEnableVertexAttribArray(a_is_text);
        glVertexAttribPointer(
            a_is_text,
            1,
            GL_FLOAT,
            0,
            stride,
            56 as *const std::ffi::c_void,
        );

        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

        glDrawElements(
            GL_TRIANGLES,
            self.ui_batch.indices.len() as i32,
            GL_UNSIGNED_SHORT,
            std::ptr::null(),
        );
    }

    unsafe fn create_program(vs_src: &str, fs_src: &str) -> Result<GLuint, String> {
        let vs = Self::compile_shader(GL_VERTEX_SHADER, vs_src)?;
        let fs = Self::compile_shader(GL_FRAGMENT_SHADER, fs_src)?;

        let program = glCreateProgram();
        glAttachShader(program, vs);
        glAttachShader(program, fs);
        glLinkProgram(program);

        let mut linked = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut linked);
        if linked == 0 {
            return Err("Shader program linking failed".into());
        }

        glDeleteShader(vs);
        glDeleteShader(fs);

        Ok(program)
    }

    unsafe fn compile_shader(type_: GLenum, source: &str) -> Result<GLuint, String> {
        let shader = glCreateShader(type_);
        let c_str = std::ffi::CString::new(source).map_err(|e| e.to_string())?;
        let ptr = c_str.as_ptr();
        glShaderSource(shader, 1, &ptr, std::ptr::null());
        glCompileShader(shader);

        let mut compiled = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut compiled);
        if compiled == 0 {
            return Err(format!("Shader compilation failed for type {type_}"));
        }

        Ok(shader)
    }
}
