// =============================================================================
// TechScript AppForge — 3D Procedural Geometry & Lighting Engine
// runtime/android_runtime/src/geometry3d.rs
//
// Hardware-accelerated 3D engine for OpenGL ES 2.0:
//   - Multiple 3D meshes (Emblem Crystal, Octahedron Gem, Hex Prism, Stellated Star)
//   - Blinn-Phong specular lighting + Fresnel glass reflections
//   - Touch-controlled 3D rotation, zoom, and animation controls
// =============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex3D {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeshType {
    Crystal,    // Dual-hexagonal bipyramid emblem
    Octahedron, // Classic 8-faced diamond
    HexPrism,   // Hexagonal prism column
    StarGem,    // 24-faced stellated jewel
}

impl MeshType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Crystal    => "TechScript Crystal",
            Self::Octahedron => "Octahedron Gem",
            Self::HexPrism   => "Hexagonal Prism",
            Self::StarGem    => "Stellated Diamond",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::Crystal    => Self::Octahedron,
            Self::Octahedron => Self::HexPrism,
            Self::HexPrism   => Self::StarGem,
            Self::StarGem    => Self::Crystal,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialPreset {
    TechScriptIndigo, // #6366F1
    LiquidCyan,       // #06B6D4
    AmethystPurple,   // #8B5CF6
    EmeraldChrome,    // #10B981
}

impl MaterialPreset {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TechScriptIndigo => "Indigo Prism",
            Self::LiquidCyan       => "Cyan Glass",
            Self::AmethystPurple   => "Amethyst Glow",
            Self::EmeraldChrome    => "Emerald Chrome",
        }
    }

    pub fn base_color(&self) -> [f32; 4] {
        match self {
            Self::TechScriptIndigo => [0.388, 0.400, 0.945, 0.92],
            Self::LiquidCyan       => [0.024, 0.714, 0.831, 0.88],
            Self::AmethystPurple   => [0.545, 0.361, 0.965, 0.90],
            Self::EmeraldChrome    => [0.063, 0.725, 0.506, 0.92],
        }
    }

    pub fn specular_color(&self) -> [f32; 3] {
        match self {
            Self::TechScriptIndigo => [0.90, 0.92, 1.00],
            Self::LiquidCyan       => [0.75, 0.98, 1.00],
            Self::AmethystPurple   => [0.98, 0.85, 1.00],
            Self::EmeraldChrome    => [0.85, 1.00, 0.92],
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::TechScriptIndigo => Self::LiquidCyan,
            Self::LiquidCyan       => Self::AmethystPurple,
            Self::AmethystPurple   => Self::EmeraldChrome,
            Self::EmeraldChrome    => Self::TechScriptIndigo,
        }
    }
}

/// Generates a 3D mesh based on type and material preset
pub fn generate_mesh(mesh_type: MeshType, material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    match mesh_type {
        MeshType::Crystal    => generate_crystal_mesh(material),
        MeshType::Octahedron => generate_octahedron_mesh(material),
        MeshType::HexPrism   => generate_hex_prism_mesh(material),
        MeshType::StarGem    => generate_star_mesh(material),
    }
}

/// Generates the faceted 3D TechScript jewel crystal (12 triangular facets)
fn generate_crystal_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let r_eq = 0.90f32;
    let mut eq = [[0.0f32; 3]; 6];
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0;
        eq[i] = [r_eq * angle.cos(), 0.0, r_eq * angle.sin()];
    }

    let top = [0.0f32, 1.30, 0.0];
    let bot = [0.0f32, -1.30, 0.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Upper 6 facets
    for i in 0..6 {
        let next = (i + 1) % 6;
        let v0 = top;
        let v1 = eq[i];
        let v2 = eq[next];
        let n = compute_normal(v0, v1, v2);

        let base_idx = vertices.len() as u16;
        let shade = 0.82 + 0.18 * ((i as f32 * 1.6).sin() * 0.5 + 0.5);
        let col = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];

        vertices.push(Vertex3D { pos: v0, normal: n, color: col });
        vertices.push(Vertex3D { pos: v1, normal: n, color: col });
        vertices.push(Vertex3D { pos: v2, normal: n, color: col });

        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
    }

    // Lower 6 facets
    for i in 0..6 {
        let next = (i + 1) % 6;
        let v0 = bot;
        let v1 = eq[next];
        let v2 = eq[i];
        let n = compute_normal(v0, v1, v2);

        let base_idx = vertices.len() as u16;
        let shade = 0.68 + 0.22 * ((i as f32 * 2.1).cos() * 0.5 + 0.5);
        let col = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];

        vertices.push(Vertex3D { pos: v0, normal: n, color: col });
        vertices.push(Vertex3D { pos: v1, normal: n, color: col });
        vertices.push(Vertex3D { pos: v2, normal: n, color: col });

        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
    }

    (vertices, indices)
}

/// Generates a classic 8-faced octahedron gem
fn generate_octahedron_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let r = 1.05f32;
    let top = [0.0, r * 1.25, 0.0];
    let bot = [0.0, -r * 1.25, 0.0];
    let mid = [
        [r, 0.0, 0.0],
        [0.0, 0.0, r],
        [-r, 0.0, 0.0],
        [0.0, 0.0, -r],
    ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..4 {
        let next = (i + 1) % 4;
        // Upper
        let n = compute_normal(top, mid[i], mid[next]);
        let base_idx = vertices.len() as u16;
        let shade = 0.85 + 0.15 * (i as f32 * 0.25);
        let col = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];
        vertices.push(Vertex3D { pos: top, normal: n, color: col });
        vertices.push(Vertex3D { pos: mid[i], normal: n, color: col });
        vertices.push(Vertex3D { pos: mid[next], normal: n, color: col });
        indices.push(base_idx); indices.push(base_idx + 1); indices.push(base_idx + 2);

        // Lower
        let n_bot = compute_normal(bot, mid[next], mid[i]);
        let base_idx2 = vertices.len() as u16;
        let shade2 = 0.70 + 0.15 * (i as f32 * 0.25);
        let col2 = [base[0] * shade2, base[1] * shade2, base[2] * shade2, base[3]];
        vertices.push(Vertex3D { pos: bot, normal: n_bot, color: col2 });
        vertices.push(Vertex3D { pos: mid[next], normal: n_bot, color: col2 });
        vertices.push(Vertex3D { pos: mid[i], normal: n_bot, color: col2 });
        indices.push(base_idx2); indices.push(base_idx2 + 1); indices.push(base_idx2 + 2);
    }

    (vertices, indices)
}

/// Generates a hexagonal prism column
fn generate_hex_prism_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let r = 0.80f32;
    let h = 0.90f32;
    let mut top_pts = [[0.0f32; 3]; 6];
    let mut bot_pts = [[0.0f32; 3]; 6];

    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0;
        top_pts[i] = [r * angle.cos(), h, r * angle.sin()];
        bot_pts[i] = [r * angle.cos(), -h, r * angle.sin()];
    }

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 6 side quads (2 triangles each)
    for i in 0..6 {
        let next = (i + 1) % 6;
        let n = compute_normal(top_pts[i], bot_pts[i], top_pts[next]);
        let base_idx = vertices.len() as u16;
        let shade = 0.75 + 0.25 * ((i as f32 * 1.5).sin() * 0.5 + 0.5);
        let col = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];

        vertices.push(Vertex3D { pos: top_pts[i], normal: n, color: col });
        vertices.push(Vertex3D { pos: bot_pts[i], normal: n, color: col });
        vertices.push(Vertex3D { pos: top_pts[next], normal: n, color: col });
        vertices.push(Vertex3D { pos: bot_pts[next], normal: n, color: col });

        indices.push(base_idx); indices.push(base_idx + 1); indices.push(base_idx + 2);
        indices.push(base_idx + 2); indices.push(base_idx + 1); indices.push(base_idx + 3);
    }

    (vertices, indices)
}

/// Generates a 24-faced stellated diamond
fn generate_star_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let r_inner = 0.55f32;
    let r_outer = 1.15f32;
    let mut pts = [[0.0f32; 3]; 12];

    for i in 0..12 {
        let r = if i % 2 == 0 { r_outer } else { r_inner };
        let angle = (i as f32) * std::f32::consts::PI / 6.0;
        pts[i] = [r * angle.cos(), 0.0, r * angle.sin()];
    }

    let top = [0.0, 1.10, 0.0];
    let bot = [0.0, -1.10, 0.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..12 {
        let next = (i + 1) % 12;
        let n_top = compute_normal(top, pts[i], pts[next]);
        let base_idx = vertices.len() as u16;
        let shade = 0.80 + 0.20 * ((i as f32).sin() * 0.5 + 0.5);
        let col = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];

        vertices.push(Vertex3D { pos: top, normal: n_top, color: col });
        vertices.push(Vertex3D { pos: pts[i], normal: n_top, color: col });
        vertices.push(Vertex3D { pos: pts[next], normal: n_top, color: col });

        indices.push(base_idx); indices.push(base_idx + 1); indices.push(base_idx + 2);

        let n_bot = compute_normal(bot, pts[next], pts[i]);
        let base_idx2 = vertices.len() as u16;
        let col2 = [base[0] * shade * 0.85, base[1] * shade * 0.85, base[2] * shade * 0.85, base[3]];

        vertices.push(Vertex3D { pos: bot, normal: n_bot, color: col2 });
        vertices.push(Vertex3D { pos: pts[next], normal: n_bot, color: col2 });
        vertices.push(Vertex3D { pos: pts[i], normal: n_bot, color: col2 });

        indices.push(base_idx2); indices.push(base_idx2 + 1); indices.push(base_idx2 + 2);
    }

    (vertices, indices)
}

fn compute_normal(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    let u = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
    let v = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

    let nx = u[1] * v[2] - u[2] * v[1];
    let ny = u[2] * v[0] - u[0] * v[2];
    let nz = u[0] * v[1] - u[1] * v[0];

    let len = (nx * nx + ny * ny + nz * nz).sqrt().max(0.0001);
    [nx / len, ny / len, nz / len]
}

// ─────────────────────────────────────────────────────────────────────────────
// Matrix 4x4 operations
// ─────────────────────────────────────────────────────────────────────────────

pub type Mat4 = [f32; 16];

pub fn mat4_perspective(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_rad * 0.5).tan();
    let nf = 1.0 / (near - far);

    [
        f / aspect, 0.0, 0.0,                     0.0,
        0.0,        f,   0.0,                     0.0,
        0.0,        0.0, (far + near) * nf,      -1.0,
        0.0,        0.0, 2.0 * far * near * nf,   0.0,
    ]
}

pub fn mat4_multiply(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f32; 16];
    for col in 0..4 {
        for row in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + row] * b[col * 4 + k];
            }
            out[col * 4 + row] = sum;
        }
    }
    out
}

pub fn mat4_rotate_y(rad: f32) -> Mat4 {
    let c = rad.cos();
    let s = rad.sin();
    [
         c,   0.0,  -s,   0.0,
        0.0,  1.0, 0.0,   0.0,
         s,   0.0,   c,   0.0,
        0.0,  0.0, 0.0,   1.0,
    ]
}

pub fn mat4_rotate_x(rad: f32) -> Mat4 {
    let c = rad.cos();
    let s = rad.sin();
    [
        1.0,  0.0, 0.0,   0.0,
        0.0,   c,    s,   0.0,
        0.0,  -s,    c,   0.0,
        0.0,  0.0, 0.0,   1.0,
    ]
}

pub fn mat4_translate(tx: f32, ty: f32, tz: f32) -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
         tx,  ty,  tz, 1.0,
    ]
}
