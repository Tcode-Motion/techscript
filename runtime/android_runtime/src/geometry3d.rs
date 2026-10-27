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
    // ── Category 1: Fleet & Aerospace (7) ──────────────────────────────────
    Starship,     // Supersonic starfighter with delta wings
    Dreadnought,  // Heavy battlecruiser with tiered superstructure & bays
    CyberDrone,   // 4-rotor drone with sensor dome
    SpaceStation, // Toroidal spinning habitat ring with docking pylon
    MarsRover,    // Planetary exploration vehicle with mast & wheels
    StealthJet,   // Faceted radar-deflecting stealth airframe
    Satellite,    // Orbital communications satellite with solar arrays

    // ── Category 2: Cosmos & Astronomy (7) ─────────────────────────────────
    EarthGlobe,   // Moving Earth with continents & equatorial ring
    SolarSystem,  // Radiant sun with concentric orbital planet rings
    SpiralGalaxy, // Logarithmic 2-arm spiral nebula disk
    SaturnRings,  // Gas giant sphere with wide planetary rings
    BlackHole,    // Gravitational singularity & accretion disc
    Pulsar,       // Neutron core with polar plasma jets
    Meteoroid,    // Irregular cratered tumbling space asteroid

    // ── Category 3: Math, Tech & Crystals (8) ──────────────────────────────
    QuantumTorus, // Smooth 3D circular torus knot ring
    DNAHelix,     // Twisting double helix with base-pair rungs
    Tesseract,    // 4D hypercube projected lattice
    CyberTower,   // Tiered futuristic skyscraper
    TechCore,     // Geodesic 20-faced icosahedron core
    StarGem,      // 24-faced stellated jewel
    Octahedron,   // Classic 8-faced diamond
    HexPrism,     // Hexagonal prism column
    Crystal,      // Dual-hexagonal bipyramid emblem
}

impl MeshType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Starship => "Starship Fighter",
            Self::Dreadnought => "Dreadnought Cruiser",
            Self::CyberDrone => "Cyber Drone",
            Self::SpaceStation => "Space Station Habitat",
            Self::MarsRover => "Mars Rover",
            Self::StealthJet => "Stealth Jet",
            Self::Satellite => "Satellite",

            Self::EarthGlobe => "Moving Earth",
            Self::SolarSystem => "Solar System",
            Self::SpiralGalaxy => "Spiral Galaxy",
            Self::SaturnRings => "Saturn Rings",
            Self::BlackHole => "Black Hole",
            Self::Pulsar => "Pulsar Star",
            Self::Meteoroid => "Meteoroid",

            Self::QuantumTorus => "Quantum Torus",
            Self::DNAHelix => "DNA Helix",
            Self::Tesseract => "4D Hypercube",
            Self::CyberTower => "Cyber Tower",
            Self::TechCore => "TechScript Core",
            Self::StarGem => "Stellated Diamond",
            Self::Octahedron => "Octahedron Gem",
            Self::HexPrism => "Hexagonal Prism",
            Self::Crystal => "Crystal Emblem",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::Starship => Self::Dreadnought,
            Self::Dreadnought => Self::CyberDrone,
            Self::CyberDrone => Self::SpaceStation,
            Self::SpaceStation => Self::MarsRover,
            Self::MarsRover => Self::StealthJet,
            Self::StealthJet => Self::Satellite,
            Self::Satellite => Self::EarthGlobe,

            Self::EarthGlobe => Self::SolarSystem,
            Self::SolarSystem => Self::SpiralGalaxy,
            Self::SpiralGalaxy => Self::SaturnRings,
            Self::SaturnRings => Self::BlackHole,
            Self::BlackHole => Self::Pulsar,
            Self::Pulsar => Self::Meteoroid,
            Self::Meteoroid => Self::QuantumTorus,

            Self::QuantumTorus => Self::DNAHelix,
            Self::DNAHelix => Self::Tesseract,
            Self::Tesseract => Self::CyberTower,
            Self::CyberTower => Self::TechCore,
            Self::TechCore => Self::StarGem,
            Self::StarGem => Self::Octahedron,
            Self::Octahedron => Self::HexPrism,
            Self::HexPrism => Self::Crystal,
            Self::Crystal => Self::Starship,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Starship => Self::Crystal,
            Self::Dreadnought => Self::Starship,
            Self::CyberDrone => Self::Dreadnought,
            Self::SpaceStation => Self::CyberDrone,
            Self::MarsRover => Self::SpaceStation,
            Self::StealthJet => Self::MarsRover,
            Self::Satellite => Self::StealthJet,

            Self::EarthGlobe => Self::Satellite,
            Self::SolarSystem => Self::EarthGlobe,
            Self::SpiralGalaxy => Self::SolarSystem,
            Self::SaturnRings => Self::SpiralGalaxy,
            Self::BlackHole => Self::SaturnRings,
            Self::Pulsar => Self::BlackHole,
            Self::Meteoroid => Self::Pulsar,

            Self::QuantumTorus => Self::Meteoroid,
            Self::DNAHelix => Self::QuantumTorus,
            Self::Tesseract => Self::DNAHelix,
            Self::CyberTower => Self::Tesseract,
            Self::TechCore => Self::CyberTower,
            Self::StarGem => Self::TechCore,
            Self::Octahedron => Self::StarGem,
            Self::HexPrism => Self::Octahedron,
            Self::Crystal => Self::HexPrism,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Starship => 1,
            Self::Dreadnought => 2,
            Self::CyberDrone => 3,
            Self::SpaceStation => 4,
            Self::MarsRover => 5,
            Self::StealthJet => 6,
            Self::Satellite => 7,
            Self::EarthGlobe => 8,
            Self::SolarSystem => 9,
            Self::SpiralGalaxy => 10,
            Self::SaturnRings => 11,
            Self::BlackHole => 12,
            Self::Pulsar => 13,
            Self::Meteoroid => 14,
            Self::QuantumTorus => 15,
            Self::DNAHelix => 16,
            Self::Tesseract => 17,
            Self::CyberTower => 18,
            Self::TechCore => 19,
            Self::StarGem => 20,
            Self::Octahedron => 21,
            Self::HexPrism => 22,
            Self::Crystal => 23,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialPreset {
    TechScriptIndigo, // #6366F1
    LiquidCyan,       // #06B6D4
    CyanLaser,        // Alias
    AmethystPurple,   // #8B5CF6
    AmethystFire,     // Alias
    EmeraldChrome,    // #10B981
    EmeraldGem,       // Alias
    CoralFire,        // #F43F5E
    AmberGold,        // #F59E0B
    RubyLaser,        // #EF4444 Intense Crimson Laser
    CyberLime,        // #84CC16 Fluorescent Electric Lime
    PlasmaGold,       // #FBBF24 Solar Plasma Gold
    DiamondIce,       // #38BDF8 Ice Blue Diamond
}

impl MaterialPreset {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TechScriptIndigo => "Indigo Prism",
            Self::LiquidCyan | Self::CyanLaser => "Cyan Glass",
            Self::AmethystPurple | Self::AmethystFire => "Amethyst Glow",
            Self::EmeraldChrome | Self::EmeraldGem => "Emerald Chrome",
            Self::CoralFire => "Coral Rose",
            Self::AmberGold => "Amber Glow",
            Self::RubyLaser => "Ruby Laser",
            Self::CyberLime => "Cyber Lime",
            Self::PlasmaGold => "Plasma Gold",
            Self::DiamondIce => "Diamond Ice",
        }
    }

    pub fn base_color(&self) -> [f32; 4] {
        match self {
            Self::TechScriptIndigo => [0.26, 0.44, 0.98, 0.95],
            Self::LiquidCyan | Self::CyanLaser => [0.00, 0.90, 1.00, 0.95],
            Self::AmethystPurple | Self::AmethystFire => [0.72, 0.28, 0.98, 0.95],
            Self::EmeraldChrome | Self::EmeraldGem => [0.00, 0.90, 0.62, 0.95],
            Self::CoralFire => [0.95, 0.25, 0.38, 0.95],
            Self::AmberGold => [0.98, 0.65, 0.10, 0.95],
            Self::RubyLaser => [0.98, 0.14, 0.24, 0.95],
            Self::CyberLime => [0.55, 0.95, 0.05, 0.95],
            Self::PlasmaGold => [1.00, 0.80, 0.15, 0.95],
            Self::DiamondIce => [0.35, 0.85, 1.00, 0.95],
        }
    }

    pub fn specular_color(&self) -> [f32; 3] {
        match self {
            Self::TechScriptIndigo => [0.90, 0.95, 1.00],
            Self::LiquidCyan | Self::CyanLaser => [0.85, 1.00, 1.00],
            Self::AmethystPurple | Self::AmethystFire => [1.00, 0.88, 1.00],
            Self::EmeraldChrome | Self::EmeraldGem => [0.85, 1.00, 0.95],
            Self::CoralFire => [1.00, 0.85, 0.90],
            Self::AmberGold => [1.00, 0.95, 0.85],
            Self::RubyLaser => [1.00, 0.80, 0.85],
            Self::CyberLime => [0.95, 1.00, 0.70],
            Self::PlasmaGold => [1.00, 0.98, 0.80],
            Self::DiamondIce => [0.90, 0.98, 1.00],
        }
    }

    pub fn rim_color(&self) -> [f32; 3] {
        match self {
            Self::TechScriptIndigo => [0.35, 0.55, 1.00],
            Self::LiquidCyan | Self::CyanLaser => [0.00, 0.92, 1.00],
            Self::AmethystPurple | Self::AmethystFire => [0.85, 0.35, 1.00],
            Self::EmeraldChrome | Self::EmeraldGem => [0.00, 0.92, 0.65],
            Self::CoralFire => [1.00, 0.30, 0.50],
            Self::AmberGold => [1.00, 0.75, 0.15],
            Self::RubyLaser => [1.00, 0.20, 0.35],
            Self::CyberLime => [0.65, 1.00, 0.10],
            Self::PlasmaGold => [1.00, 0.85, 0.20],
            Self::DiamondIce => [0.40, 0.90, 1.00],
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::TechScriptIndigo => Self::LiquidCyan,
            Self::LiquidCyan | Self::CyanLaser => Self::AmethystPurple,
            Self::AmethystPurple | Self::AmethystFire => Self::EmeraldChrome,
            Self::EmeraldChrome | Self::EmeraldGem => Self::CoralFire,
            Self::CoralFire => Self::AmberGold,
            Self::AmberGold => Self::RubyLaser,
            Self::RubyLaser => Self::CyberLime,
            Self::CyberLime => Self::PlasmaGold,
            Self::PlasmaGold => Self::DiamondIce,
            Self::DiamondIce => Self::TechScriptIndigo,
        }
    }
}

/// Converts normalized HSV (0.0..1.0) to RGBA color for custom color picker
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 4] {
    let h6 = (h % 1.0) * 6.0;
    let i = h6.floor() as i32;
    let f = h6 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    [r, g, b, 0.95]
}

/// Generates a 3D mesh based on type and material preset
pub fn generate_mesh(mesh_type: MeshType, material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    match mesh_type {
        MeshType::Starship => generate_starship_mesh(material),
        MeshType::Dreadnought => generate_dreadnought_mesh(material),
        MeshType::CyberDrone => generate_cyber_drone_mesh(material),
        MeshType::SpaceStation => generate_space_station_mesh(material),
        MeshType::MarsRover => generate_mars_rover_mesh(material),
        MeshType::StealthJet => generate_stealth_jet_mesh(material),
        MeshType::Satellite => generate_satellite_mesh(material),

        MeshType::EarthGlobe => generate_earth_globe_mesh(material),
        MeshType::SolarSystem => generate_solar_system_mesh(material),
        MeshType::SpiralGalaxy => generate_spiral_galaxy_mesh(material),
        MeshType::SaturnRings => generate_saturn_rings_mesh(material),
        MeshType::BlackHole => generate_black_hole_mesh(material),
        MeshType::Pulsar => generate_pulsar_mesh(material),
        MeshType::Meteoroid => generate_meteoroid_mesh(material),

        MeshType::QuantumTorus => generate_torus_mesh(material),
        MeshType::DNAHelix => generate_dna_helix_mesh(material),
        MeshType::Tesseract => generate_tesseract_mesh(material),
        MeshType::CyberTower => generate_cyber_tower_mesh(material),
        MeshType::TechCore => generate_tech_core_mesh(material),
        MeshType::StarGem => generate_star_mesh(material),
        MeshType::Octahedron => generate_octahedron_mesh(material),
        MeshType::HexPrism => generate_hex_prism_mesh(material),
        MeshType::Crystal => generate_crystal_mesh(material),
    }
}

/// Helper to add a triangle with computed normal and individual vertex colors
fn add_tri(
    vertices: &mut Vec<Vertex3D>,
    indices: &mut Vec<u16>,
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    c0: [f32; 4],
    c1: [f32; 4],
    c2: [f32; 4],
) {
    let n = compute_normal(p0, p1, p2);
    let base = vertices.len() as u16;
    vertices.push(Vertex3D {
        pos: p0,
        normal: n,
        color: c0,
    });
    vertices.push(Vertex3D {
        pos: p1,
        normal: n,
        color: c1,
    });
    vertices.push(Vertex3D {
        pos: p2,
        normal: n,
        color: c2,
    });
    indices.push(base);
    indices.push(base + 1);
    indices.push(base + 2);
}

fn add_quad(
    vertices: &mut Vec<Vertex3D>,
    indices: &mut Vec<u16>,
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    p3: [f32; 3],
    c: [f32; 4],
) {
    add_tri(vertices, indices, p0, p1, p2, c, c, c);
    add_tri(vertices, indices, p0, p2, p3, c, c, c);
}

fn add_box(
    vertices: &mut Vec<Vertex3D>,
    indices: &mut Vec<u16>,
    min: [f32; 3],
    max: [f32; 3],
    col: [f32; 4],
) {
    let p000 = [min[0], min[1], min[2]];
    let p100 = [max[0], min[1], min[2]];
    let p110 = [max[0], max[1], min[2]];
    let p010 = [min[0], max[1], min[2]];
    let p001 = [min[0], min[1], max[2]];
    let p101 = [max[0], min[1], max[2]];
    let p111 = [max[0], max[1], max[2]];
    let p011 = [min[0], max[1], max[2]];

    add_quad(vertices, indices, p001, p101, p111, p011, col); // Front (+Z)
    add_quad(vertices, indices, p100, p000, p010, p110, col); // Back (-Z)
    add_quad(vertices, indices, p011, p111, p110, p010, col); // Top (+Y)
    add_quad(vertices, indices, p000, p100, p101, p001, col); // Bottom (-Y)
    add_quad(vertices, indices, p101, p100, p110, p111, col); // Right (+X)
    add_quad(vertices, indices, p000, p001, p011, p010, col); // Left (-X)
}

fn add_sphere(
    vertices: &mut Vec<Vertex3D>,
    indices: &mut Vec<u16>,
    center: [f32; 3],
    radius: f32,
    stacks: usize,
    sectors: usize,
    col: [f32; 4],
) {
    let pi = std::f32::consts::PI;
    for i in 0..stacks {
        let lat0 = pi * (-0.5 + (i as f32) / (stacks as f32));
        let z0 = radius * lat0.sin();
        let zr0 = radius * lat0.cos();

        let lat1 = pi * (-0.5 + ((i + 1) as f32) / (stacks as f32));
        let z1 = radius * lat1.sin();
        let zr1 = radius * lat1.cos();

        for j in 0..sectors {
            let lng0 = 2.0 * pi * (j as f32) / (sectors as f32);
            let lng1 = 2.0 * pi * ((j + 1) as f32) / (sectors as f32);

            let p0 = [
                center[0] + zr0 * lng0.cos(),
                center[1] + z0,
                center[2] + zr0 * lng0.sin(),
            ];
            let p1 = [
                center[0] + zr0 * lng1.cos(),
                center[1] + z0,
                center[2] + zr0 * lng1.sin(),
            ];
            let p2 = [
                center[0] + zr1 * lng1.cos(),
                center[1] + z1,
                center[2] + zr1 * lng1.sin(),
            ];
            let p3 = [
                center[0] + zr1 * lng0.cos(),
                center[1] + z1,
                center[2] + zr1 * lng0.sin(),
            ];

            add_quad(vertices, indices, p0, p1, p2, p3, col);
        }
    }
}

fn add_ring(
    vertices: &mut Vec<Vertex3D>,
    indices: &mut Vec<u16>,
    center: [f32; 3],
    r_inner: f32,
    r_outer: f32,
    segments: usize,
    col: [f32; 4],
) {
    let pi2 = std::f32::consts::PI * 2.0;
    for i in 0..segments {
        let a0 = pi2 * (i as f32) / (segments as f32);
        let a1 = pi2 * ((i + 1) as f32) / (segments as f32);

        let p0 = [
            center[0] + r_inner * a0.cos(),
            center[1],
            center[2] + r_inner * a0.sin(),
        ];
        let p1 = [
            center[0] + r_outer * a0.cos(),
            center[1],
            center[2] + r_outer * a0.sin(),
        ];
        let p2 = [
            center[0] + r_outer * a1.cos(),
            center[1],
            center[2] + r_outer * a1.sin(),
        ];
        let p3 = [
            center[0] + r_inner * a1.cos(),
            center[1],
            center[2] + r_inner * a1.sin(),
        ];

        add_quad(vertices, indices, p0, p1, p2, p3, col);
        add_quad(vertices, indices, p3, p2, p1, p0, col);
    }
}

fn add_strut(
    vertices: &mut Vec<Vertex3D>,
    indices: &mut Vec<u16>,
    p0: [f32; 3],
    p1: [f32; 3],
    thickness: f32,
    col: [f32; 4],
) {
    let dx = p1[0] - p0[0];
    let dy = p1[1] - p0[1];
    let dz = p1[2] - p0[2];
    let len = (dx * dx + dy * dy + dz * dz).sqrt().max(0.0001);
    let dir = [dx / len, dy / len, dz / len];

    let up = if dir[1].abs() < 0.9 {
        [0.0, 1.0, 0.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let side = [
        dir[1] * up[2] - dir[2] * up[1],
        dir[2] * up[0] - dir[0] * up[2],
        dir[0] * up[1] - dir[1] * up[0],
    ];
    let slen = (side[0] * side[0] + side[1] * side[1] + side[2] * side[2])
        .sqrt()
        .max(0.0001);
    let s = [
        side[0] / slen * thickness,
        side[1] / slen * thickness,
        side[2] / slen * thickness,
    ];
    let u = [
        (dir[1] * s[2] - dir[2] * s[1]),
        (dir[2] * s[0] - dir[0] * s[2]),
        (dir[0] * s[1] - dir[1] * s[0]),
    ];

    let v0 = [
        p0[0] - s[0] - u[0],
        p0[1] - s[1] - u[1],
        p0[2] - s[2] - u[2],
    ];
    let v1 = [
        p0[0] + s[0] - u[0],
        p0[1] + s[1] - u[1],
        p0[2] + s[2] - u[2],
    ];
    let v2 = [
        p0[0] + s[0] + u[0],
        p0[1] + s[1] + u[1],
        p0[2] + s[2] + u[2],
    ];
    let v3 = [
        p0[0] - s[0] + u[0],
        p0[1] - s[1] + u[1],
        p0[2] - s[2] + u[2],
    ];

    let v4 = [
        p1[0] - s[0] - u[0],
        p1[1] - s[1] - u[1],
        p1[2] - s[2] - u[2],
    ];
    let v5 = [
        p1[0] + s[0] - u[0],
        p1[1] + s[1] - u[1],
        p1[2] + s[2] - u[2],
    ];
    let v6 = [
        p1[0] + s[0] + u[0],
        p1[1] + s[1] + u[1],
        p1[2] + s[2] + u[2],
    ];
    let v7 = [
        p1[0] - s[0] + u[0],
        p1[1] - s[1] + u[1],
        p1[2] - s[2] + u[2],
    ];

    add_quad(vertices, indices, v0, v1, v5, v4, col);
    add_quad(vertices, indices, v1, v2, v6, v5, col);
    add_quad(vertices, indices, v2, v3, v7, v6, col);
    add_quad(vertices, indices, v3, v0, v4, v7, col);
}

/// Generates the high-tech 3D bounding transfer cube and internal coordinate lattice cage
pub fn generate_wireframe_cage() -> (Vec<Vertex3D>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let b = 1.12f32; // Cage boundary
    let th = 0.012f32;
    let col_edge = [0.00, 0.95, 1.00, 0.95];
    let col_grid = [0.00, 0.60, 0.85, 0.60];

    let c = [
        [-b, -b, -b],
        [b, -b, -b],
        [b, b, -b],
        [-b, b, -b],
        [-b, -b, b],
        [b, -b, b],
        [b, b, b],
        [-b, b, b],
    ];

    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    for (i0, i1) in edges {
        add_strut(&mut vertices, &mut indices, c[i0], c[i1], th, col_edge);
    }

    let grid_th = 0.006f32;
    add_strut(
        &mut vertices,
        &mut indices,
        [-b, 0.0, 0.0],
        [b, 0.0, 0.0],
        grid_th,
        col_grid,
    );
    add_strut(
        &mut vertices,
        &mut indices,
        [0.0, -b, 0.0],
        [0.0, b, 0.0],
        grid_th,
        col_grid,
    );
    add_strut(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, -b],
        [0.0, 0.0, b],
        grid_th,
        col_grid,
    );

    let l_len = 0.26f32;
    for &corner in &c {
        let sx = if corner[0] > 0.0 { -1.0 } else { 1.0 };
        let sy = if corner[1] > 0.0 { -1.0 } else { 1.0 };
        let sz = if corner[2] > 0.0 { -1.0 } else { 1.0 };
        add_strut(
            &mut vertices,
            &mut indices,
            corner,
            [corner[0] + sx * l_len, corner[1], corner[2]],
            th * 1.3,
            [1.0, 0.85, 0.20, 1.0],
        );
        add_strut(
            &mut vertices,
            &mut indices,
            corner,
            [corner[0], corner[1] + sy * l_len, corner[2]],
            th * 1.3,
            [1.0, 0.85, 0.20, 1.0],
        );
        add_strut(
            &mut vertices,
            &mut indices,
            corner,
            [corner[0], corner[1], corner[2] + sz * l_len],
            th * 1.3,
            [1.0, 0.85, 0.20, 1.0],
        );
    }

    (vertices, indices)
}

/// Generates a heavy capital Dreadnought Battlecruiser with tiered superstructure & engine cluster
fn generate_dreadnought_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_hull = [base[0] * 0.85, base[1] * 0.85, base[2] * 0.85, base[3]];
    let col_deck = [base[0] * 0.65, base[1] * 0.65, base[2] * 0.65, base[3]];
    let col_bridge = [0.0, 0.90, 1.0, 1.0];
    let col_plasma = [1.0, 0.70, 0.20, 1.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Main wedge hull
    add_box(
        &mut vertices,
        &mut indices,
        [-0.35, -0.12, -0.80],
        [0.35, 0.12, 0.30],
        col_hull,
    );
    // Forward chisel prow
    let prow_tip = [0.0, 0.0, 0.95];
    let prow_tl = [-0.35, 0.12, 0.30];
    let prow_tr = [0.35, 0.12, 0.30];
    let prow_bl = [-0.35, -0.12, 0.30];
    let prow_br = [0.35, -0.12, 0.30];
    add_tri(
        &mut vertices,
        &mut indices,
        prow_tl,
        prow_tr,
        prow_tip,
        col_deck,
        col_deck,
        col_deck,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        prow_bl,
        prow_tip,
        prow_br,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        prow_tl,
        prow_tip,
        prow_bl,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        prow_tr,
        prow_br,
        prow_tip,
        col_hull,
        col_hull,
        col_hull,
    );

    // Tier 1 Dorsal Superstructure
    add_box(
        &mut vertices,
        &mut indices,
        [-0.22, 0.12, -0.60],
        [0.22, 0.26, 0.10],
        col_deck,
    );
    // Tier 2 Command Bridge Tower
    add_box(
        &mut vertices,
        &mut indices,
        [-0.12, 0.26, -0.45],
        [0.12, 0.42, -0.15],
        col_bridge,
    );

    // Lateral Weapon Sponsons
    add_box(
        &mut vertices,
        &mut indices,
        [-0.52, -0.06, -0.30],
        [-0.35, 0.08, 0.10],
        col_deck,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [0.35, -0.06, -0.30],
        [0.52, 0.08, 0.10],
        col_deck,
    );

    // Triple Stern Plasma Nozzles
    add_box(
        &mut vertices,
        &mut indices,
        [-0.25, -0.06, -0.92],
        [-0.10, 0.06, -0.80],
        col_plasma,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [-0.07, -0.06, -0.92],
        [0.07, 0.06, -0.80],
        col_plasma,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [0.10, -0.06, -0.92],
        [0.25, 0.06, -0.80],
        col_plasma,
    );

    (vertices, indices)
}

/// Generates a rotating toroidal Space Station Habitat with docking pylon and radial spokes
fn generate_space_station_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_ring = [base[0], base[1], base[2], base[3]];
    let col_hub = [base[0] * 0.75, base[1] * 0.75, base[2] * 0.75, base[3]];
    let col_solar = [0.10, 0.60, 0.95, 0.95];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Central cylindrical docking hub
    add_box(
        &mut vertices,
        &mut indices,
        [-0.12, -0.45, -0.12],
        [0.12, 0.45, 0.12],
        col_hub,
    );

    // Outer Torus Habitat Ring (16-sided polygon tube)
    let num_seg = 16;
    let r_major = 0.82f32;
    let r_tube = 0.09f32;
    let pi2 = std::f32::consts::PI * 2.0;

    for i in 0..num_seg {
        let a0 = pi2 * (i as f32) / (num_seg as f32);
        let a1 = pi2 * ((i + 1) as f32) / (num_seg as f32);
        let p0 = [r_major * a0.cos(), 0.0, r_major * a0.sin()];
        let p1 = [r_major * a1.cos(), 0.0, r_major * a1.sin()];
        add_strut(&mut vertices, &mut indices, p0, p1, r_tube, col_ring);
    }

    // 4 Pressurized Transit Spokes
    for i in 0..4 {
        let ang = (i as f32) * std::f32::consts::PI * 0.5;
        let hub_pt = [0.12 * ang.cos(), 0.0, 0.12 * ang.sin()];
        let ring_pt = [r_major * ang.cos(), 0.0, r_major * ang.sin()];
        add_strut(&mut vertices, &mut indices, hub_pt, ring_pt, 0.035, col_hub);
    }

    // Dual Photovoltaic Solar Arrays mounted at top pylon
    add_box(
        &mut vertices,
        &mut indices,
        [-0.60, 0.40, -0.02],
        [-0.15, 0.48, 0.02],
        col_solar,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [0.15, 0.40, -0.02],
        [0.60, 0.48, 0.02],
        col_solar,
    );

    (vertices, indices)
}

/// Generates a Mars Rover with rocker-bogie chassis, camera mast, and 6 wheel assemblies
fn generate_mars_rover_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_body = [base[0] * 0.90, base[1] * 0.85, base[2] * 0.20, base[3]]; // Gold foil chassis
    let col_deck = [base[0] * 0.70, base[1] * 0.70, base[2] * 0.70, base[3]];
    let col_mast = [0.85, 0.90, 0.95, 1.0];
    let col_lens = [0.0, 0.90, 1.0, 1.0];
    let col_wheel = [0.20, 0.24, 0.32, 1.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Chassis avionics box
    add_box(
        &mut vertices,
        &mut indices,
        [-0.28, -0.10, -0.38],
        [0.28, 0.14, 0.38],
        col_body,
    );
    // Solar equipment deck
    add_box(
        &mut vertices,
        &mut indices,
        [-0.26, 0.14, -0.36],
        [0.26, 0.18, 0.36],
        col_deck,
    );

    // High Mast and Mastcam stereo sensor head
    add_strut(
        &mut vertices,
        &mut indices,
        [0.18, 0.18, 0.24],
        [0.18, 0.62, 0.24],
        0.025,
        col_mast,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [0.10, 0.60, 0.18],
        [0.26, 0.72, 0.30],
        col_mast,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [0.13, 0.63, 0.30],
        [0.17, 0.69, 0.34],
        col_lens,
    );
    add_box(
        &mut vertices,
        &mut indices,
        [0.19, 0.63, 0.30],
        [0.23, 0.69, 0.34],
        col_lens,
    );

    // Front Robotic Arm Manipulator
    add_strut(
        &mut vertices,
        &mut indices,
        [-0.14, 0.06, 0.38],
        [-0.14, -0.15, 0.65],
        0.02,
        col_mast,
    );

    // 6 Rocker-Bogie Wheel assemblies (Left & Right x 3)
    let wheel_z = [0.38, 0.0, -0.38];
    for &z in &wheel_z {
        // Left wheel
        add_box(
            &mut vertices,
            &mut indices,
            [-0.46, -0.32, z - 0.09],
            [-0.34, -0.14, z + 0.09],
            col_wheel,
        );
        add_strut(
            &mut vertices,
            &mut indices,
            [-0.28, -0.05, z],
            [-0.34, -0.23, z],
            0.02,
            col_deck,
        );

        // Right wheel
        add_box(
            &mut vertices,
            &mut indices,
            [0.34, -0.32, z - 0.09],
            [0.46, -0.14, z + 0.09],
            col_wheel,
        );
        add_strut(
            &mut vertices,
            &mut indices,
            [0.28, -0.05, z],
            [0.34, -0.23, z],
            0.02,
            col_deck,
        );
    }

    (vertices, indices)
}

/// Generates a low-RCS Stealth Jet with faceted airframe and canted vertical stabilizers
fn generate_stealth_jet_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_hull = [base[0] * 0.45, base[1] * 0.48, base[2] * 0.55, base[3]]; // Dark stealth matte
    let col_glass = [1.0, 0.80, 0.20, 0.95]; // Gold indium-tin oxide canopy

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let nose = [0.0, -0.02, 1.05];
    let ck_top = [0.0, 0.22, 0.25];
    let ck_l = [-0.18, 0.05, 0.25];
    let ck_r = [0.18, 0.05, 0.25];
    let spine_aft = [0.0, 0.12, -0.75];
    let belly_f = [0.0, -0.15, 0.20];
    let belly_aft = [0.0, -0.12, -0.75];

    let wing_l = [-1.02, 0.02, -0.55];
    let wing_r = [1.02, 0.02, -0.55];
    let wing_aft_l = [-0.28, 0.0, -0.75];
    let wing_aft_r = [0.28, 0.0, -0.75];

    let vfin_l = [-0.38, 0.45, -0.75];
    let vfin_r = [0.38, 0.45, -0.75];

    // Cockpit Facets
    add_tri(
        &mut vertices,
        &mut indices,
        nose,
        ck_l,
        ck_top,
        col_hull,
        col_glass,
        col_glass,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        nose,
        ck_top,
        ck_r,
        col_hull,
        col_glass,
        col_glass,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        ck_top,
        ck_l,
        spine_aft,
        col_glass,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        ck_top,
        spine_aft,
        ck_r,
        col_glass,
        col_hull,
        col_hull,
    );

    // Ventral Fuselage
    add_tri(
        &mut vertices,
        &mut indices,
        nose,
        belly_f,
        ck_l,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        nose,
        ck_r,
        belly_f,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        belly_f,
        belly_aft,
        ck_l,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        belly_f,
        ck_r,
        belly_aft,
        col_hull,
        col_hull,
        col_hull,
    );

    // Swept Diamond Wings
    add_tri(
        &mut vertices,
        &mut indices,
        ck_l,
        wing_l,
        wing_aft_l,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        ck_l,
        wing_aft_l,
        belly_aft,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        ck_r,
        wing_aft_r,
        wing_r,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        ck_r,
        belly_aft,
        wing_aft_r,
        col_hull,
        col_hull,
        col_hull,
    );

    // Canted V-Tail Rudders
    add_tri(
        &mut vertices,
        &mut indices,
        spine_aft,
        wing_aft_l,
        vfin_l,
        col_hull,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        spine_aft,
        vfin_r,
        wing_aft_r,
        col_hull,
        col_hull,
        col_hull,
    );

    (vertices, indices)
}

/// Generates an orbital Satellite with high-gain dish antenna and blue solar panels
fn generate_satellite_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_body = [base[0] * 0.95, base[1] * 0.80, base[2] * 0.25, base[3]]; // Gold thermal multi-layer insulation
    let col_dish = [0.90, 0.95, 1.0, 1.0];
    let col_solar = [0.08, 0.45, 0.90, 0.95];
    let col_frame = [0.30, 0.35, 0.45, 1.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Central Avionics Bus
    add_box(
        &mut vertices,
        &mut indices,
        [-0.22, -0.22, -0.22],
        [0.22, 0.22, 0.22],
        col_body,
    );

    // Forward High-Gain Parabolic Dish Antenna
    let dish_apex = [0.0, 0.0, 0.52];
    let num_d = 8;
    let r_d = 0.26f32;
    for i in 0..num_d {
        let a0 = (i as f32) * std::f32::consts::PI * 2.0 / (num_d as f32);
        let a1 = ((i + 1) as f32) * std::f32::consts::PI * 2.0 / (num_d as f32);
        let p0 = [r_d * a0.cos(), r_d * a0.sin(), 0.32];
        let p1 = [r_d * a1.cos(), r_d * a1.sin(), 0.32];
        add_tri(
            &mut vertices,
            &mut indices,
            dish_apex,
            p0,
            p1,
            col_dish,
            col_dish,
            col_dish,
        );
    }
    add_strut(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.22],
        dish_apex,
        0.02,
        col_frame,
    );

    // Port Photovoltaic Solar Array
    add_box(
        &mut vertices,
        &mut indices,
        [-0.98, -0.18, -0.015],
        [-0.26, 0.18, 0.015],
        col_solar,
    );
    add_strut(
        &mut vertices,
        &mut indices,
        [-0.26, 0.0, 0.0],
        [-0.22, 0.0, 0.0],
        0.025,
        col_frame,
    );

    // Starboard Photovoltaic Solar Array
    add_box(
        &mut vertices,
        &mut indices,
        [0.26, -0.18, -0.015],
        [0.98, 0.18, 0.015],
        col_solar,
    );
    add_strut(
        &mut vertices,
        &mut indices,
        [0.22, 0.0, 0.0],
        [0.26, 0.0, 0.0],
        0.025,
        col_frame,
    );

    (vertices, indices)
}

/// Generates the Moving Earth with continents, oceanic spheres, and atmosphere ring
fn generate_earth_globe_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_ocean = [0.05, 0.35, 0.88, 1.0];
    let col_land = [0.15, 0.78, 0.38, 1.0];
    let col_halo = [0.00, 0.90, 1.00, 0.60];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let radius = 0.72f32;
    let stacks = 12;
    let sectors = 16;
    let pi = std::f32::consts::PI;

    for i in 0..stacks {
        let lat0 = pi * (-0.5 + (i as f32) / (stacks as f32));
        let z0 = radius * lat0.sin();
        let zr0 = radius * lat0.cos();

        let lat1 = pi * (-0.5 + ((i + 1) as f32) / (stacks as f32));
        let z1 = radius * lat1.sin();
        let zr1 = radius * lat1.cos();

        for j in 0..sectors {
            let lng0 = 2.0 * pi * (j as f32) / (sectors as f32);
            let lng1 = 2.0 * pi * ((j + 1) as f32) / (sectors as f32);

            let p0 = [zr0 * lng0.cos(), z0, zr0 * lng0.sin()];
            let p1 = [zr0 * lng1.cos(), z0, zr0 * lng1.sin()];
            let p2 = [zr1 * lng1.cos(), z1, zr1 * lng1.sin()];
            let p3 = [zr1 * lng0.cos(), z1, zr1 * lng0.sin()];

            // Continental landmass heuristic based on spherical coordinates
            let is_land = (lat0 > -0.2
                && lat0 < 0.6
                && ((lng0 > 1.2 && lng0 < 2.8) || (lng0 > 4.2 && lng0 < 5.4)))
                || (lat0 > 0.1 && lat0 < 0.8 && (lng0 > 0.0 && lng0 < 1.0));
            let col = if is_land { col_land } else { col_ocean };

            add_quad(&mut vertices, &mut indices, p0, p1, p2, p3, col);
        }
    }

    // Equatorial Atmosphere Orbit Ring
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        radius * 1.12,
        radius * 1.18,
        20,
        col_halo,
    );

    (vertices, indices)
}

/// Generates the Heliocentric Solar System with radiant Sun and 3 planetary orbits
fn generate_solar_system_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_sun = [1.00, 0.85, 0.15, 1.0];
    let col_mercury = [0.75, 0.65, 0.55, 1.0];
    let col_earth = [0.10, 0.70, 1.00, 1.0];
    let col_mars = [0.95, 0.35, 0.20, 1.0];
    let col_orbit = [0.35, 0.45, 0.65, 0.40];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Central Sun
    add_sphere(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.28,
        8,
        12,
        col_sun,
    );

    // Orbit 1: Mercury
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.45,
        0.47,
        18,
        col_orbit,
    );
    add_sphere(
        &mut vertices,
        &mut indices,
        [0.46, 0.0, 0.0],
        0.055,
        6,
        8,
        col_mercury,
    );

    // Orbit 2: Earth
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.70,
        0.72,
        22,
        col_orbit,
    );
    let earth_pos = [-0.71 * 0.707, 0.0, 0.71 * 0.707];
    add_sphere(
        &mut vertices,
        &mut indices,
        earth_pos,
        0.08,
        6,
        8,
        col_earth,
    );

    // Orbit 3: Mars
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.94,
        0.96,
        26,
        col_orbit,
    );
    let mars_pos = [0.95 * 0.5, 0.0, -0.95 * 0.866];
    add_sphere(&mut vertices, &mut indices, mars_pos, 0.07, 6, 8, col_mars);

    (vertices, indices)
}

/// Generates a Logarithmic Spiral Galaxy with central core and 2 glowing nebula arms
fn generate_spiral_galaxy_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_core = [1.00, 0.95, 0.85, 1.0];
    let col_arm1 = [0.00, 0.85, 1.00, 0.85]; // Cyan star clusters
    let col_arm2 = [0.85, 0.35, 1.00, 0.85]; // Magenta stellar dust

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Galactic Core Bulge
    add_sphere(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.22,
        8,
        12,
        col_core,
    );

    // Two Logarithmic Spiral Arms
    let num_stars = 24;
    for i in 0..num_stars {
        let t = (i as f32) * 0.22;
        let r = 0.24 + 0.15 * t;

        // Arm 1
        let a1 = t * 2.4;
        let p1 = [r * a1.cos(), (t * 1.5).sin() * 0.04, r * a1.sin()];
        add_box(
            &mut vertices,
            &mut indices,
            [p1[0] - 0.04, p1[1] - 0.02, p1[2] - 0.04],
            [p1[0] + 0.04, p1[1] + 0.02, p1[2] + 0.04],
            col_arm1,
        );

        // Arm 2 (180 deg offset)
        let a2 = a1 + std::f32::consts::PI;
        let p2 = [r * a2.cos(), (t * 1.5).cos() * 0.04, r * a2.sin()];
        add_box(
            &mut vertices,
            &mut indices,
            [p2[0] - 0.04, p2[1] - 0.02, p2[2] - 0.04],
            [p2[0] + 0.04, p2[1] + 0.02, p2[2] + 0.04],
            col_arm2,
        );
    }

    (vertices, indices)
}

/// Generates Saturn with equatorial rings and Cassini division
fn generate_saturn_rings_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_planet = [0.95, 0.85, 0.60, 1.0]; // Golden gas giant bands
    let col_inner_ring = [0.90, 0.78, 0.45, 0.85];
    let col_outer_ring = [0.75, 0.85, 0.95, 0.75];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Central Oblate Gas Giant
    add_sphere(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.42,
        10,
        14,
        col_planet,
    );

    // Inner Ring
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.54,
        0.76,
        24,
        col_inner_ring,
    );
    // Outer Ring (Separated by Cassini division)
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.82,
        1.04,
        24,
        col_outer_ring,
    );

    (vertices, indices)
}

/// Generates a Relativistic Black Hole with singularity and swirling accretion disc
fn generate_black_hole_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_singularity = [0.01, 0.01, 0.03, 1.0]; // Event horizon
    let col_photon = [0.95, 0.98, 1.0, 1.0]; // Relativistic photon ring
    let col_inner_disc = [0.00, 0.90, 1.00, 0.95]; // Cyan plasma
    let col_outer_disc = [1.00, 0.40, 0.10, 0.75]; // Red shifted thermal fringe

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Event Horizon Sphere
    add_sphere(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.32,
        8,
        12,
        col_singularity,
    );

    // Photon Ring
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.35,
        0.42,
        20,
        col_photon,
    );

    // Accretion Disc (Tilted 20 degrees for gravitational lensing aesthetic)
    let num_seg = 20;
    let pi2 = std::f32::consts::PI * 2.0;
    for i in 0..num_seg {
        let a0 = pi2 * (i as f32) / (num_seg as f32);
        let a1 = pi2 * ((i + 1) as f32) / (num_seg as f32);

        let p0 = [0.45 * a0.cos(), 0.45 * a0.sin() * 0.35, 0.45 * a0.sin()];
        let p1 = [0.98 * a0.cos(), 0.98 * a0.sin() * 0.35, 0.98 * a0.sin()];
        let p2 = [0.98 * a1.cos(), 0.98 * a1.sin() * 0.35, 0.98 * a1.sin()];
        let p3 = [0.45 * a1.cos(), 0.45 * a1.sin() * 0.35, 0.45 * a1.sin()];

        add_quad(&mut vertices, &mut indices, p0, p1, p2, p3, col_inner_disc);
        add_quad(&mut vertices, &mut indices, p3, p2, p1, p0, col_outer_disc);
    }

    (vertices, indices)
}

/// Generates a Magnetized Pulsar Star with relativistic polar plasma jet cones
fn generate_pulsar_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_neutron = [0.85, 0.98, 1.0, 1.0]; // Superdense neutron core
    let col_jet = [0.75, 0.25, 1.0, 0.90]; // Relativistic magnetic beam
    let col_ring = [0.00, 0.90, 1.0, 0.60];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Neutron Core
    add_sphere(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.28,
        8,
        12,
        col_neutron,
    );

    // Equatorial Magnetic Field Ring
    add_ring(
        &mut vertices,
        &mut indices,
        [0.0, 0.0, 0.0],
        0.36,
        0.44,
        18,
        col_ring,
    );

    // North Polar Relativistic Plasma Beam Cone
    let p_n_apex = [0.0, 1.05, 0.0];
    let num_b = 8;
    for i in 0..num_b {
        let a0 = (i as f32) * std::f32::consts::PI * 2.0 / (num_b as f32);
        let a1 = ((i + 1) as f32) * std::f32::consts::PI * 2.0 / (num_b as f32);
        let p0 = [0.18 * a0.cos(), 0.28, 0.18 * a0.sin()];
        let p1 = [0.18 * a1.cos(), 0.28, 0.18 * a1.sin()];
        add_tri(
            &mut vertices,
            &mut indices,
            p_n_apex,
            p0,
            p1,
            col_jet,
            col_neutron,
            col_neutron,
        );
    }

    // South Polar Relativistic Plasma Beam Cone
    let p_s_apex = [0.0, -1.05, 0.0];
    for i in 0..num_b {
        let a0 = (i as f32) * std::f32::consts::PI * 2.0 / (num_b as f32);
        let a1 = ((i + 1) as f32) * std::f32::consts::PI * 2.0 / (num_b as f32);
        let p0 = [0.18 * a0.cos(), -0.28, 0.18 * a0.sin()];
        let p1 = [0.18 * a1.cos(), -0.28, 0.18 * a1.sin()];
        add_tri(
            &mut vertices,
            &mut indices,
            p_s_apex,
            p1,
            p0,
            col_jet,
            col_neutron,
            col_neutron,
        );
    }

    (vertices, indices)
}

/// Generates a Craggy Tumbling Meteoroid with cratered impact facets
fn generate_meteoroid_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_rock = [base[0] * 0.65, base[1] * 0.60, base[2] * 0.55, base[3]];
    let col_crater = [base[0] * 0.40, base[1] * 0.35, base[2] * 0.30, base[3]];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let stacks = 8;
    let sectors = 10;
    let pi = std::f32::consts::PI;

    for i in 0..stacks {
        let lat0 = pi * (-0.5 + (i as f32) / (stacks as f32));
        let lat1 = pi * (-0.5 + ((i + 1) as f32) / (stacks as f32));

        for j in 0..sectors {
            let lng0 = 2.0 * pi * (j as f32) / (sectors as f32);
            let lng1 = 2.0 * pi * ((j + 1) as f32) / (sectors as f32);

            // Procedural crater harmonic perturbation
            let r0 = 0.68 + 0.18 * ((lat0 * 3.0).sin() * (lng0 * 4.0).cos());
            let r1 = 0.68 + 0.18 * ((lat0 * 3.0).sin() * (lng1 * 4.0).cos());
            let r2 = 0.68 + 0.18 * ((lat1 * 3.0).sin() * (lng1 * 4.0).cos());
            let r3 = 0.68 + 0.18 * ((lat1 * 3.0).sin() * (lng0 * 4.0).cos());

            let p0 = [
                r0 * lat0.cos() * lng0.cos(),
                r0 * lat0.sin(),
                r0 * lat0.cos() * lng0.sin(),
            ];
            let p1 = [
                r1 * lat0.cos() * lng1.cos(),
                r1 * lat0.sin(),
                r1 * lat0.cos() * lng1.sin(),
            ];
            let p2 = [
                r2 * lat1.cos() * lng1.cos(),
                r2 * lat1.sin(),
                r2 * lat1.cos() * lng1.sin(),
            ];
            let p3 = [
                r3 * lat1.cos() * lng0.cos(),
                r3 * lat1.sin(),
                r3 * lat1.cos() * lng0.sin(),
            ];

            let col = if r0 < 0.65 { col_crater } else { col_rock };
            add_quad(&mut vertices, &mut indices, p0, p1, p2, p3, col);
        }
    }

    (vertices, indices)
}

/// Generates a Double-Helical DNA Strand with paired molecular base rungs
fn generate_dna_helix_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_strand_a = [0.00, 0.90, 1.00, 1.0]; // Cyan strand
    let col_strand_b = [1.00, 0.30, 0.60, 1.0]; // Magenta strand
    let col_rung_at = [1.00, 0.85, 0.15, 1.0]; // Adenine-Thymine (Gold)
    let col_rung_cg = [0.20, 0.90, 0.40, 1.0]; // Cytosine-Guanine (Emerald)

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let steps = 18;
    let total_h = 1.70f32;
    let r_helix = 0.42f32;

    for i in 0..steps {
        let t0 = (i as f32) / (steps as f32);
        let t1 = ((i + 1) as f32) / (steps as f32);

        let y0 = -total_h * 0.5 + t0 * total_h;
        let y1 = -total_h * 0.5 + t1 * total_h;

        let a0 = t0 * std::f32::consts::PI * 3.5;
        let a1 = t1 * std::f32::consts::PI * 3.5;

        // Strand A
        let pa0 = [r_helix * a0.cos(), y0, r_helix * a0.sin()];
        let pa1 = [r_helix * a1.cos(), y1, r_helix * a1.sin()];
        add_strut(&mut vertices, &mut indices, pa0, pa1, 0.035, col_strand_a);

        // Strand B (opposite side)
        let pb0 = [-r_helix * a0.cos(), y0, -r_helix * a0.sin()];
        let pb1 = [-r_helix * a1.cos(), y1, -r_helix * a1.sin()];
        add_strut(&mut vertices, &mut indices, pb0, pb1, 0.035, col_strand_b);

        // Horizontal Base Pair Rung
        let rung_col = if i % 2 == 0 { col_rung_at } else { col_rung_cg };
        add_strut(&mut vertices, &mut indices, pa0, pb0, 0.025, rung_col);
    }

    (vertices, indices)
}

/// Generates a 4D Hypercube (Tesseract) stereographic projection lattice
fn generate_tesseract_mesh(_material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let col_outer = [0.00, 0.92, 1.00, 0.95]; // Cyan outer cube
    let col_inner = [0.85, 0.35, 1.00, 0.95]; // Purple inner cube
    let col_diag = [1.00, 0.85, 0.20, 0.90]; // Gold 4D projection struts

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let ro = 0.76f32; // Outer cube half-width
    let ri = 0.38f32; // Inner cube half-width
    let th = 0.022f32;

    let outer = [
        [-ro, -ro, -ro],
        [ro, -ro, -ro],
        [ro, ro, -ro],
        [-ro, ro, -ro],
        [-ro, -ro, ro],
        [ro, -ro, ro],
        [ro, ro, ro],
        [-ro, ro, ro],
    ];
    let inner = [
        [-ri, -ri, -ri],
        [ri, -ri, -ri],
        [ri, ri, -ri],
        [-ri, ri, -ri],
        [-ri, -ri, ri],
        [ri, -ri, ri],
        [ri, ri, ri],
        [-ri, ri, ri],
    ];

    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    // 12 Outer Cube Struts
    for (i0, i1) in edges {
        add_strut(
            &mut vertices,
            &mut indices,
            outer[i0],
            outer[i1],
            th,
            col_outer,
        );
    }

    // 12 Inner Cube Struts
    for (i0, i1) in edges {
        add_strut(
            &mut vertices,
            &mut indices,
            inner[i0],
            inner[i1],
            th,
            col_inner,
        );
    }

    // 8 Connecting 4D Projection Struts linking corresponding vertices
    for i in 0..8 {
        add_strut(
            &mut vertices,
            &mut indices,
            outer[i],
            inner[i],
            th * 0.85,
            col_diag,
        );
    }

    (vertices, indices)
}

/// Generates a Futuristic Tiered Cyber Tower with architectural setbacks and spire
fn generate_cyber_tower_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_glass = [base[0] * 0.90, base[1] * 0.90, base[2] * 0.95, base[3]];
    let col_core = [base[0] * 0.60, base[1] * 0.60, base[2] * 0.65, base[3]];
    let col_beacon = [1.0, 0.20, 0.35, 1.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Tier 1 Base Podium
    add_box(
        &mut vertices,
        &mut indices,
        [-0.42, -0.95, -0.42],
        [0.42, -0.40, 0.42],
        col_core,
    );
    // Tier 2 Mid Tower
    add_box(
        &mut vertices,
        &mut indices,
        [-0.28, -0.40, -0.28],
        [0.28, 0.22, 0.28],
        col_glass,
    );
    // Tier 3 Crown Skyscraper
    add_box(
        &mut vertices,
        &mut indices,
        [-0.16, 0.22, -0.16],
        [0.16, 0.68, 0.16],
        col_glass,
    );

    // Spire Antenna Needle
    add_strut(
        &mut vertices,
        &mut indices,
        [0.0, 0.68, 0.0],
        [0.0, 1.05, 0.0],
        0.02,
        [0.9, 0.95, 1.0, 1.0],
    );
    // Apex Red Warning Beacon
    add_box(
        &mut vertices,
        &mut indices,
        [-0.03, 1.03, -0.03],
        [0.03, 1.09, 0.03],
        col_beacon,
    );

    (vertices, indices)
}

/// Generates a futuristic sci-fi supersonic Starship with delta wings, twin tail fins, and thrusters
fn generate_starship_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_cockpit = [0.05, 0.95, 1.0, 1.0]; // Radiant cockpit crystal
    let col_hull = [base[0] * 0.90, base[1] * 0.90, base[2] * 0.90, base[3]];
    let col_hull_dark = [base[0] * 0.65, base[1] * 0.65, base[2] * 0.65, base[3]];
    let col_plasma = [1.0, 0.80, 0.20, 1.0]; // Solar plasma thruster glow

    let mut vertices = Vec::with_capacity(120);
    let mut indices = Vec::with_capacity(120);
    let s = 0.65f32;
    let p_nose = [0.0f32, 0.0, 1.45 * s];
    let p_canopy = [0.0, 0.32 * s, 0.35 * s];
    let p_ck_l = [-0.26 * s, 0.12 * s, 0.30 * s];
    let p_ck_r = [0.26 * s, 0.12 * s, 0.30 * s];
    let p_belly_f = [0.0, -0.22 * s, 0.30 * s];
    let p_spine_aft = [0.0, 0.22 * s, -0.95 * s];
    let p_belly_aft = [0.0, -0.18 * s, -0.95 * s];

    let p_wrl_l = [-0.26 * s, 0.04 * s, 0.30 * s];
    let p_wrt_l = [-0.28 * s, 0.02 * s, -0.85 * s];
    let p_wt_l = [-1.45 * s, 0.06 * s, -0.75 * s];
    let p_wfin_l = [-1.45 * s, 0.35 * s, -0.85 * s];

    let p_wrl_r = [0.26 * s, 0.04 * s, 0.30 * s];
    let p_wrt_r = [0.28 * s, 0.02 * s, -0.85 * s];
    let p_wt_r = [1.45 * s, 0.06 * s, -0.75 * s];
    let p_wfin_r = [1.45 * s, 0.35 * s, -0.85 * s];

    let p_vfin_l = [-0.25 * s, 0.65 * s, -0.95 * s];
    let p_vfin_r = [0.25 * s, 0.65 * s, -0.95 * s];

    let p_eng_l = [-0.18 * s, 0.0, -1.25 * s];
    let p_eng_r = [0.18 * s, 0.0, -1.25 * s];

    // Cockpit canopy facets (luminous crystal glass)
    add_tri(
        &mut vertices,
        &mut indices,
        p_nose,
        p_ck_l,
        p_canopy,
        col_hull,
        col_cockpit,
        col_cockpit,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_nose,
        p_canopy,
        p_ck_r,
        col_hull,
        col_cockpit,
        col_cockpit,
    );

    // Forward fuselage belly
    add_tri(
        &mut vertices,
        &mut indices,
        p_nose,
        p_belly_f,
        p_ck_l,
        col_hull,
        col_hull_dark,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_nose,
        p_ck_r,
        p_belly_f,
        col_hull,
        col_hull,
        col_hull_dark,
    );

    // Dorsal & Ventral mid fuselage
    add_tri(
        &mut vertices,
        &mut indices,
        p_canopy,
        p_ck_l,
        p_spine_aft,
        col_cockpit,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_canopy,
        p_spine_aft,
        p_ck_r,
        col_cockpit,
        col_hull,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_belly_f,
        p_belly_aft,
        p_ck_l,
        col_hull_dark,
        col_hull_dark,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_belly_f,
        p_ck_r,
        p_belly_aft,
        col_hull_dark,
        col_hull,
        col_hull_dark,
    );

    // Left swept delta wing (top & bottom)
    add_tri(
        &mut vertices,
        &mut indices,
        p_wrl_l,
        p_wt_l,
        p_wrt_l,
        col_hull,
        col_hull,
        col_hull_dark,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_wrl_l,
        p_wrt_l,
        p_wt_l,
        col_hull_dark,
        col_hull_dark,
        col_hull_dark,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_wt_l,
        p_wfin_l,
        p_wrt_l,
        col_hull,
        col_cockpit,
        col_hull_dark,
    );

    // Right swept delta wing (top & bottom)
    add_tri(
        &mut vertices,
        &mut indices,
        p_wrl_r,
        p_wt_r,
        p_wrt_r,
        col_hull,
        col_hull_dark,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_wrl_r,
        p_wt_r,
        p_wrt_r,
        col_hull_dark,
        col_hull_dark,
        col_hull_dark,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_wt_r,
        p_wrt_r,
        p_wfin_r,
        col_hull,
        col_hull_dark,
        col_cockpit,
    );

    // Twin vertical stabilizers
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_l,
        p_spine_aft,
        p_vfin_l,
        col_hull,
        col_hull,
        col_cockpit,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_l,
        p_vfin_l,
        p_spine_aft,
        col_hull,
        col_cockpit,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_r,
        p_vfin_r,
        p_spine_aft,
        col_hull,
        col_cockpit,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_r,
        p_spine_aft,
        p_vfin_r,
        col_hull,
        col_hull,
        col_cockpit,
    );

    // Twin Plasma Thrusters & Aft Bulkhead
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_l,
        p_eng_l,
        p_spine_aft,
        col_hull,
        col_plasma,
        col_hull,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_l,
        p_belly_aft,
        p_eng_l,
        col_hull,
        col_hull_dark,
        col_plasma,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_r,
        p_spine_aft,
        p_eng_r,
        col_hull,
        col_hull,
        col_plasma,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_ck_r,
        p_eng_r,
        p_belly_aft,
        col_hull,
        col_plasma,
        col_hull_dark,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_spine_aft,
        p_eng_l,
        p_belly_aft,
        col_hull,
        col_plasma,
        col_hull_dark,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        p_spine_aft,
        p_belly_aft,
        p_eng_r,
        col_hull,
        col_hull_dark,
        col_plasma,
    );

    (vertices, indices)
}

/// Generates a futuristic quad-rotor surveillance drone with central spherical core and rotor discs
fn generate_cyber_drone_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let col_core = [base[0], base[1], base[2], base[3]];
    let col_optic = [1.0, 0.20, 0.35, 1.0]; // Ruby sensor optic
    let col_arm = [base[0] * 0.70, base[1] * 0.70, base[2] * 0.70, base[3]];
    let col_rotor = [0.0, 0.92, 1.0, 0.95]; // Cyan rotor energy ring

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Central core octahedron
    let r_core = 0.24f32;
    let top = [0.0, r_core, 0.0];
    let bot = [0.0, -r_core, 0.0];
    let forward_sensor = [0.0, 0.0, r_core * 1.35];

    let mid = [
        [r_core, 0.0, 0.0],
        [0.0, 0.0, r_core],
        [-r_core, 0.0, 0.0],
        [0.0, 0.0, -r_core],
    ];

    for i in 0..4 {
        let next = (i + 1) % 4;
        add_tri(
            &mut vertices,
            &mut indices,
            top,
            mid[i],
            mid[next],
            col_core,
            col_core,
            col_core,
        );
        add_tri(
            &mut vertices,
            &mut indices,
            bot,
            mid[next],
            mid[i],
            col_core,
            col_core,
            col_core,
        );
    }

    // Forward Optic Sensor cone
    add_tri(
        &mut vertices,
        &mut indices,
        top,
        forward_sensor,
        mid[1],
        col_core,
        col_optic,
        col_core,
    );
    add_tri(
        &mut vertices,
        &mut indices,
        bot,
        mid[1],
        forward_sensor,
        col_core,
        col_core,
        col_optic,
    );

    // 4 Quad Rotor Booms and Rotor Discs
    let angles = [
        std::f32::consts::PI * 0.25,
        std::f32::consts::PI * 0.75,
        std::f32::consts::PI * 1.25,
        std::f32::consts::PI * 1.75,
    ];

    let arm_r = 0.78f32;
    let rotor_r = 0.22f32;

    for &a in &angles {
        let cos_a = a.cos();
        let sin_a = a.sin();
        let arm_end = [arm_r * cos_a, 0.06, arm_r * sin_a];
        let arm_bot = [arm_r * cos_a, -0.05, arm_r * sin_a];

        // Boom strut from core
        let core_attach = [r_core * 0.8 * cos_a, 0.0, r_core * 0.8 * sin_a];
        add_tri(
            &mut vertices,
            &mut indices,
            core_attach,
            arm_end,
            arm_bot,
            col_arm,
            col_arm,
            col_arm,
        );

        // Rotor Disc (6-sided energy ring around motor hub)
        let hub_top = [arm_end[0], 0.16, arm_end[2]];
        let hub_bot = [arm_end[0], 0.01, arm_end[2]];

        for seg in 0..6 {
            let theta1 = (seg as f32) * std::f32::consts::PI / 3.0;
            let theta2 = ((seg + 1) as f32) * std::f32::consts::PI / 3.0;
            let p1 = [
                arm_end[0] + rotor_r * theta1.cos(),
                0.09,
                arm_end[2] + rotor_r * theta1.sin(),
            ];
            let p2 = [
                arm_end[0] + rotor_r * theta2.cos(),
                0.09,
                arm_end[2] + rotor_r * theta2.sin(),
            ];

            add_tri(
                &mut vertices,
                &mut indices,
                hub_top,
                p1,
                p2,
                col_rotor,
                col_rotor,
                col_rotor,
            );
            add_tri(
                &mut vertices,
                &mut indices,
                hub_bot,
                p2,
                p1,
                col_arm,
                col_rotor,
                col_rotor,
            );
        }
    }

    (vertices, indices)
}

/// Generates a smooth, curved Quantum Torus Knot ring with Blinn-Phong specular curvature
fn generate_torus_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let num_u = 24;
    let num_v = 12;
    let r_major = 0.68f32;
    let r_minor = 0.24f32;

    let mut vertices = Vec::with_capacity(num_u * num_v);
    let mut indices = Vec::with_capacity(num_u * num_v * 6);

    for u in 0..num_u {
        let theta = (u as f32) * 2.0 * std::f32::consts::PI / (num_u as f32);
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        for v in 0..num_v {
            let phi = (v as f32) * 2.0 * std::f32::consts::PI / (num_v as f32);
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let x = (r_major + r_minor * cos_phi) * cos_theta;
            let y = r_minor * sin_phi;
            let z = (r_major + r_minor * cos_phi) * sin_theta;

            let nx = cos_phi * cos_theta;
            let ny = sin_phi;
            let nz = cos_phi * sin_theta;

            let lum = 0.80 + 0.20 * sin_phi;
            let col = [
                (base[0] * lum).clamp(0.0, 1.0),
                (base[1] * lum).clamp(0.0, 1.0),
                (base[2] * lum).clamp(0.0, 1.0),
                base[3],
            ];

            vertices.push(Vertex3D {
                pos: [x, y, z],
                normal: [nx, ny, nz],
                color: col,
            });
        }
    }

    for u in 0..num_u {
        let next_u = (u + 1) % num_u;
        for v in 0..num_v {
            let next_v = (v + 1) % num_v;

            let i0 = (u * num_v + v) as u16;
            let i1 = (next_u * num_v + v) as u16;
            let i2 = (next_u * num_v + next_v) as u16;
            let i3 = (u * num_v + next_v) as u16;

            indices.push(i0);
            indices.push(i1);
            indices.push(i2);

            indices.push(i0);
            indices.push(i2);
            indices.push(i3);
        }
    }

    (vertices, indices)
}

/// Generates the futuristic 20-faced Geodesic Icosahedron Diamond Core
fn generate_tech_core_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let phi = 1.6180339887f32;
    let inv_norm = 1.0 / (1.0 + phi * phi).sqrt();
    let r = 1.12f32;

    let a = 1.0 * inv_norm * r;
    let b = phi * inv_norm * r;

    // 12 vertices of regular icosahedron
    let v = [
        [-a, b, 0.0],
        [a, b, 0.0],
        [-a, -b, 0.0],
        [a, -b, 0.0],
        [0.0, -a, b],
        [0.0, a, b],
        [0.0, -a, -b],
        [0.0, a, -b],
        [b, 0.0, -a],
        [b, 0.0, a],
        [-b, 0.0, -a],
        [-b, 0.0, a],
    ];

    // 20 triangular faces
    let faces = [
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];

    let mut vertices = Vec::with_capacity(60);
    let mut indices = Vec::with_capacity(60);

    for (f_idx, face) in faces.iter().enumerate() {
        let p0 = v[face[0]];
        let p1 = v[face[1]];
        let p2 = v[face[2]];
        let n = compute_normal(p0, p1, p2);

        let shade = 0.82 + 0.18 * ((f_idx as f32 * 1.618).sin() * 0.5 + 0.5);

        let base_idx = vertices.len() as u16;
        for &pt in &[p0, p1, p2] {
            let y_lum = (pt[1] / r) * 0.20;
            let col = [
                ((base[0] * shade + y_lum).clamp(0.0, 1.0)),
                ((base[1] * shade + y_lum).clamp(0.0, 1.0)),
                ((base[2] * shade + y_lum).clamp(0.0, 1.0)),
                base[3],
            ];
            vertices.push(Vertex3D {
                pos: pt,
                normal: n,
                color: col,
            });
        }

        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
    }

    (vertices, indices)
}

/// Generates the faceted 3D TechScript jewel crystal with radiant neon tips
fn generate_crystal_mesh(material: MaterialPreset) -> (Vec<Vertex3D>, Vec<u16>) {
    let base = material.base_color();
    let r_eq = 0.92f32;
    let mut eq = [[0.0f32; 3]; 6];
    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI / 3.0;
        eq[i] = [r_eq * angle.cos(), 0.0, r_eq * angle.sin()];
    }

    let top = [0.0f32, 1.35, 0.0];
    let bot = [0.0f32, -1.35, 0.0];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let tip_top = [
        (base[0] * 1.30).min(1.0),
        (base[1] * 1.30).min(1.0),
        (base[2] * 1.30).min(1.0),
        1.0,
    ];
    let tip_bot = [base[0] * 0.75, base[1] * 0.75, base[2] * 0.75, 1.0];

    // Upper 6 facets
    for i in 0..6 {
        let next = (i + 1) % 6;
        let v0 = top;
        let v1 = eq[i];
        let v2 = eq[next];
        let n = compute_normal(v0, v2, v1); // Outward facing normal

        let base_idx = vertices.len() as u16;
        let shade = 0.80 + 0.20 * ((i as f32 * 1.8).sin() * 0.5 + 0.5);
        let col_body = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];

        vertices.push(Vertex3D {
            pos: v0,
            normal: n,
            color: tip_top,
        });
        vertices.push(Vertex3D {
            pos: v2,
            normal: n,
            color: col_body,
        });
        vertices.push(Vertex3D {
            pos: v1,
            normal: n,
            color: col_body,
        });

        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
    }

    // Lower 6 facets
    for i in 0..6 {
        let next = (i + 1) % 6;
        let v0 = bot;
        let v1 = eq[i];
        let v2 = eq[next];
        let n = compute_normal(v0, v1, v2); // Outward facing normal

        let base_idx = vertices.len() as u16;
        let shade = 0.70 + 0.30 * ((i as f32 * 2.2).cos() * 0.5 + 0.5);
        let col_body = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];

        vertices.push(Vertex3D {
            pos: v0,
            normal: n,
            color: tip_bot,
        });
        vertices.push(Vertex3D {
            pos: v1,
            normal: n,
            color: col_body,
        });
        vertices.push(Vertex3D {
            pos: v2,
            normal: n,
            color: col_body,
        });

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
    let mid = [[r, 0.0, 0.0], [0.0, 0.0, r], [-r, 0.0, 0.0], [0.0, 0.0, -r]];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..4 {
        let next = (i + 1) % 4;
        // Upper
        let n = compute_normal(top, mid[i], mid[next]);
        let base_idx = vertices.len() as u16;
        let shade = 0.85 + 0.15 * (i as f32 * 0.25);
        let col = [base[0] * shade, base[1] * shade, base[2] * shade, base[3]];
        vertices.push(Vertex3D {
            pos: top,
            normal: n,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: mid[i],
            normal: n,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: mid[next],
            normal: n,
            color: col,
        });
        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);

        // Lower
        let n_bot = compute_normal(bot, mid[next], mid[i]);
        let base_idx2 = vertices.len() as u16;
        let shade2 = 0.70 + 0.15 * (i as f32 * 0.25);
        let col2 = [
            base[0] * shade2,
            base[1] * shade2,
            base[2] * shade2,
            base[3],
        ];
        vertices.push(Vertex3D {
            pos: bot,
            normal: n_bot,
            color: col2,
        });
        vertices.push(Vertex3D {
            pos: mid[next],
            normal: n_bot,
            color: col2,
        });
        vertices.push(Vertex3D {
            pos: mid[i],
            normal: n_bot,
            color: col2,
        });
        indices.push(base_idx2);
        indices.push(base_idx2 + 1);
        indices.push(base_idx2 + 2);
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

        vertices.push(Vertex3D {
            pos: top_pts[i],
            normal: n,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: bot_pts[i],
            normal: n,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: top_pts[next],
            normal: n,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: bot_pts[next],
            normal: n,
            color: col,
        });

        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
        indices.push(base_idx + 2);
        indices.push(base_idx + 1);
        indices.push(base_idx + 3);
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

        vertices.push(Vertex3D {
            pos: top,
            normal: n_top,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: pts[i],
            normal: n_top,
            color: col,
        });
        vertices.push(Vertex3D {
            pos: pts[next],
            normal: n_top,
            color: col,
        });

        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);

        let n_bot = compute_normal(bot, pts[next], pts[i]);
        let base_idx2 = vertices.len() as u16;
        let col2 = [
            base[0] * shade * 0.85,
            base[1] * shade * 0.85,
            base[2] * shade * 0.85,
            base[3],
        ];

        vertices.push(Vertex3D {
            pos: bot,
            normal: n_bot,
            color: col2,
        });
        vertices.push(Vertex3D {
            pos: pts[next],
            normal: n_bot,
            color: col2,
        });
        vertices.push(Vertex3D {
            pos: pts[i],
            normal: n_bot,
            color: col2,
        });

        indices.push(base_idx2);
        indices.push(base_idx2 + 1);
        indices.push(base_idx2 + 2);
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
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (far + near) * nf,
        -1.0,
        0.0,
        0.0,
        2.0 * far * near * nf,
        0.0,
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
        c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn mat4_rotate_x(rad: f32) -> Mat4 {
    let c = rad.cos();
    let s = rad.sin();
    [
        1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn mat4_translate(tx: f32, ty: f32, tz: f32) -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, tx, ty, tz, 1.0,
    ]
}

pub fn mat4_look_at(eye: [f32; 3], target: [f32; 3], up: [f32; 3]) -> Mat4 {
    let f = [target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]];
    let f_len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(0.0001);
    let f = [f[0] / f_len, f[1] / f_len, f[2] / f_len];

    let s = [
        f[1] * up[2] - f[2] * up[1],
        f[2] * up[0] - f[0] * up[2],
        f[0] * up[1] - f[1] * up[0],
    ];
    let s_len = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt().max(0.0001);
    let s = [s[0] / s_len, s[1] / s_len, s[2] / s_len];

    let u = [
        s[1] * f[2] - s[2] * f[1],
        s[2] * f[0] - s[0] * f[2],
        s[0] * f[1] - s[1] * f[0],
    ];

    [
        s[0],
        u[0],
        -f[0],
        0.0,
        s[1],
        u[1],
        -f[1],
        0.0,
        s[2],
        u[2],
        -f[2],
        0.0,
        -(s[0] * eye[0] + s[1] * eye[1] + s[2] * eye[2]),
        -(u[0] * eye[0] + u[1] * eye[1] + u[2] * eye[2]),
        (f[0] * eye[0] + f[1] * eye[1] + f[2] * eye[2]),
        1.0,
    ]
}

pub use mat4_look_at as look_at;
pub use mat4_multiply as mat4_mul;
pub use mat4_perspective as perspective;
pub use mat4_rotate_x as rotate_x;
pub use mat4_rotate_y as rotate_y;
