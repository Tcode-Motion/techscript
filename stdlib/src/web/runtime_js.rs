//! # WebForge Universal Client JavaScript Runtime
//!
//! Provides the client-side single-page router, reactive event bindings,
//! toast notifications, clipboard copying, SQLite dynamic records loader,
//! and native WebGL 2.0 / WebGL 1.0 procedural 3D engine with zero external dependencies.

pub fn get_runtime_javascript() -> &'static str {
    r#"
function escapeHtml(str) {
    if (!str) return '';
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

function webforgeToast(msg) {
    let c = document.getElementById('webforge-toast-container');
    if (!c) {
        c = document.createElement('div');
        c.id = 'webforge-toast-container';
        document.body.appendChild(c);
    }
    const t = document.createElement('div');
    t.className = 'webforge-toast';
    t.innerHTML = '<span style="width:8px;height:8px;border-radius:50%;background:#06B6D4;box-shadow:0 0 8px #06B6D4;"></span><span>' + escapeHtml(msg) + '</span>';
    c.appendChild(t);
    setTimeout(() => {
        t.style.animation = 'toastOut 0.25s forwards';
        setTimeout(() => t.remove(), 250);
    }, 3000);
}

function webforgeCopyCode(btn) {
    const container = btn.closest('.code-window') || btn.closest('.webforge-code-block') || btn.closest('pre');
    const codeEl = container ? container.querySelector('code') : null;
    const code = codeEl ? codeEl.innerText : (container ? container.innerText : '');
    navigator.clipboard.writeText(code).then(() => {
        const orig = btn.innerText;
        btn.innerText = 'Copied!';
        setTimeout(() => btn.innerText = orig, 2000);
    }).catch(() => {
        webforgeToast('Copied to clipboard');
    });
}

function webforgeInitTheme() {
    const saved = localStorage.getItem('webforge-theme') || 'light';
    document.documentElement.setAttribute('data-theme', saved);
    if (document.body) {
        document.body.classList.toggle('dark-theme', saved === 'dark');
        document.body.classList.toggle('light-theme', saved === 'light');
    }
}

function webforgeToggleTheme() {
    const current = document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'light';
    const next = current === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', next);
    if (document.body) {
        document.body.classList.toggle('dark-theme', next === 'dark');
        document.body.classList.toggle('light-theme', next === 'light');
    }
    localStorage.setItem('webforge-theme', next);
    webforgeToast(next === 'dark' ? 'Dark Theme' : 'Light Theme');
}

try { webforgeInitTheme(); } catch (_) {}

const webforgeRouter = {
    currentRoute: '/',
    getBasePath() {
        const path = window.location.pathname;
        if (path.startsWith('/techscript')) {
            return '/techscript';
        }
        return '';
    },
    init() {
        window.addEventListener('popstate', () => this.handleLocation());
        document.addEventListener('click', (e) => {
            const a = e.target.closest('a');
            if (a && a.href && a.origin === window.location.origin) {
                let path = a.getAttribute('href');
                if (path && (path.startsWith('/') || path.startsWith('#')) && !path.startsWith('//')) {
                    const base = this.getBasePath();
                    if (base && path.startsWith(base)) {
                        path = path.substring(base.length) || '/';
                    }
                    e.preventDefault();
                    this.navigate(path);
                }
            }
        });
        this.handleLocation();
    },
    navigate(path, replace) {
        let clean = path;
        if (clean.startsWith('#/')) clean = clean.substring(1);
        else if (clean.startsWith('#')) clean = '/' + clean.substring(1);
        if (clean === '/home') clean = '/';
        const base = this.getBasePath();
        const fullUrl = base + (clean.startsWith('/') ? clean : '/' + clean);
        if (replace) window.history.replaceState({}, '', fullUrl);
        else window.history.pushState({}, '', fullUrl);
        this.handleLocation();
    },
    handleLocation() {
        let path = window.location.pathname;
        const base = this.getBasePath();
        if (base && path.startsWith(base)) {
            path = path.substring(base.length) || '/';
        }
        if (window.location.hash && window.location.hash.startsWith('#/')) {
            path = window.location.hash.substring(1);
        } else if (window.location.hash && window.location.hash.length > 1) {
            path = '/' + window.location.hash.substring(1);
        }
        if (path === '/home') path = '/';
        path = path.replace(/\/index\.html$/, '') || '/';
        this.currentRoute = path;
        const views = document.querySelectorAll('.webforge-page-view');
        let matched = null;
        views.forEach(v => {
            const r = v.getAttribute('data-route');
            if (r === path || (path === '/' && (r === '/' || r === '/home'))) {
                matched = v;
            }
        });
        views.forEach(v => v.classList.remove('active-page'));
        if (matched) {
            matched.classList.add('active-page');
            const title = matched.getAttribute('data-title');
            if (title) document.title = title.includes('TechScript') ? title : 'TechScript WebForge | ' + title;
        } else {
            const p404 = document.getElementById('page-404');
            if (p404) p404.classList.add('active-page');
            document.title = 'TechScript WebForge | 404 Not Found';
        }
        document.querySelectorAll('nav a, .webforge-nav a').forEach(link => {
            let lr = link.getAttribute('data-route') || link.getAttribute('href');
            if (base && lr && lr.startsWith(base)) {
                lr = lr.substring(base.length) || '/';
            }
            if (lr === path || (path === '/' && (lr === '/' || lr === '/home'))) {
                link.classList.add('active');
            } else {
                link.classList.remove('active');
            }
        });
        window.scrollTo({ top: 0, behavior: 'smooth' });

        if (window.WebForge3D) {
            if (matched) {
                matched.querySelectorAll('.webforge-3d-scene').forEach(sc => {
                    const id = sc.id;
                    if (id && !window.WebForge3D.instances[id]) {
                        window.WebForge3D.mount(id, { geo: 'scifi', color: 'cyan', speed: 1.2 });
                    }
                });
            }
            if (window.WebForge3D.instances) {
                Object.values(window.WebForge3D.instances).forEach(inst => {
                    if (inst && inst.checkVisibility) inst.checkVisibility();
                });
            }
        }
    }
};

function webforgeLoadDatabase(tableName) {
    const container = document.getElementById(tableName + '-container');
    if (!container) return;
    const base = (typeof webforgeRouter !== 'undefined' && webforgeRouter.getBasePath) ? webforgeRouter.getBasePath() : '';
    fetch(base + '/api/' + encodeURIComponent(tableName))
        .then(r => r.json())
        .then(data => {
            if (!Array.isArray(data) || data.length === 0) {
                container.innerHTML = '<div class="card" style="grid-column: 1 / -1; text-align: center; color: var(--text-muted);"><p>No records found in database.</p></div>';
                return;
            }
            container.innerHTML = data.map(item => `
                <div class="card">
                    <div class="badge">#${escapeHtml(item.id)}</div>
                    <h3>${escapeHtml(item.title || item.name || 'Record')}</h3>
                    <p>${escapeHtml(item.content || item.message || '')}</p>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">${escapeHtml(item.created_at || '')}</div>
                </div>
            `).join('');
        })
        .catch(() => {
            container.innerHTML = '<div class="card" style="grid-column: 1 / -1; text-align: center; color: var(--text-muted);"><p>Database storage initialized. Submit data using the form above.</p></div>';
        });
}

function webforgeSubmitForm(btnId, endpoint, inputIds) {
    const payload = {};
    if (Array.isArray(inputIds)) {
        inputIds.forEach(id => {
            const el = document.getElementById(id);
            if (el) payload[id] = el.value;
        });
    }
    const base = (typeof webforgeRouter !== 'undefined' && webforgeRouter.getBasePath) ? webforgeRouter.getBasePath() : '';
    const url = endpoint.startsWith('http') ? endpoint : (base + endpoint);
    return fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
    }).then(r => r.json()).catch(() => ({ status: 'ok' }));
}

/* ==========================================================================
   WebForge3D: Native Zero-Dependency Procedural WebGL 3D Engine
   ========================================================================== */
window.WebForge3D = {
    instances: {},

    init(id, opts) {
        return this.mount(id, opts);
    },

    mount(id, opts) {
        opts = opts || {};
        const container = document.getElementById(id);
        if (!container) return;
        const canvas = document.getElementById(id + '-canvas') || container.querySelector('canvas');
        if (!canvas) return;

        if (this.instances[id]) {
            this.instances[id].dispose();
        }

        let gl = null;
        try {
            gl = canvas.getContext('webgl2', { antialias: true, alpha: true, powerPreference: 'high-performance' })
                || canvas.getContext('webgl', { antialias: true, alpha: true })
                || canvas.getContext('experimental-webgl');
        } catch (e) {}

        if (!gl) {
            this.instances[id] = this.initFallback2D(canvas, container, id, opts);
            return;
        }

        this.instances[id] = this.initWebGL(gl, canvas, container, id, opts);
    },

    mat4: {
        create() {
            return new Float32Array([1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1]);
        },
        perspective(out, fovy, aspect, near, far) {
            const f = 1.0 / Math.tan(fovy / 2);
            out[0] = f / aspect; out[1] = 0; out[2] = 0; out[3] = 0;
            out[4] = 0; out[5] = f; out[6] = 0; out[7] = 0;
            out[8] = 0; out[9] = 0; out[10] = (far + near) / (near - far); out[11] = -1;
            out[12] = 0; out[13] = 0; out[14] = (2 * far * near) / (near - far); out[15] = 0;
            return out;
        },
        lookAt(out, eye, center, up) {
            let x0, x1, x2, y0, y1, y2, z0, z1, z2, len;
            const eyex = eye[0], eyey = eye[1], eyez = eye[2];
            const upx = up[0], upy = up[1], upz = up[2];
            const centerx = center[0], centery = center[1], centerz = center[2];

            z0 = eyex - centerx; z1 = eyey - centery; z2 = eyez - centerz;
            len = 1 / Math.hypot(z0, z1, z2);
            z0 *= len; z1 *= len; z2 *= len;

            x0 = upy * z2 - upz * z1; x1 = upz * z0 - upx * z2; x2 = upx * z1 - upy * z0;
            len = 1 / Math.hypot(x0, x1, x2);
            x0 *= len; x1 *= len; x2 *= len;

            y0 = z1 * x2 - z2 * x1; y1 = z2 * x0 - z0 * x2; y2 = z0 * x1 - z1 * x0;
            len = 1 / Math.hypot(y0, y1, y2);
            y0 *= len; y1 *= len; y2 *= len;

            out[0] = x0; out[1] = y0; out[2] = z0; out[3] = 0;
            out[4] = x1; out[5] = y1; out[6] = z1; out[7] = 0;
            out[8] = x2; out[9] = y2; out[10] = z2; out[11] = 0;
            out[12] = -(x0 * eyex + x1 * eyey + x2 * eyez);
            out[13] = -(y0 * eyex + y1 * eyey + y2 * eyez);
            out[14] = -(z0 * eyex + z1 * eyey + z2 * eyez);
            out[15] = 1;
            return out;
        },
        multiply(out, a, b) {
            const a00 = a[0], a01 = a[1], a02 = a[2], a03 = a[3];
            const a10 = a[4], a11 = a[5], a12 = a[6], a13 = a[7];
            const a20 = a[8], a21 = a[9], a22 = a[10], a23 = a[11];
            const a30 = a[12], a31 = a[13], a32 = a[14], a33 = a[15];
            let b0 = b[0], b1 = b[1], b2 = b[2], b3 = b[3];
            out[0] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
            out[1] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
            out[2] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
            out[3] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
            b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
            out[4] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
            out[5] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
            out[6] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
            out[7] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
            b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
            out[8] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
            out[9] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
            out[10] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
            out[11] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
            b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
            out[12] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
            out[13] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
            out[14] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
            out[15] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
            return out;
        },
        rotateX(out, a, rad) {
            const s = Math.sin(rad), c = Math.cos(rad);
            const a10 = a[4], a11 = a[5], a12 = a[6], a13 = a[7];
            const a20 = a[8], a21 = a[9], a22 = a[10], a23 = a[11];
            out[4] = a10 * c + a20 * s; out[5] = a11 * c + a21 * s;
            out[6] = a12 * c + a22 * s; out[7] = a13 * c + a23 * s;
            out[8] = a20 * c - a10 * s; out[9] = a21 * c - a11 * s;
            out[10] = a22 * c - a12 * s; out[11] = a23 * c - a13 * s;
            return out;
        },
        rotateY(out, a, rad) {
            const s = Math.sin(rad), c = Math.cos(rad);
            const a00 = a[0], a01 = a[1], a02 = a[2], a03 = a[3];
            const a20 = a[8], a21 = a[9], a22 = a[10], a23 = a[11];
            out[0] = a00 * c - a20 * s; out[1] = a01 * c - a21 * s;
            out[2] = a02 * c - a22 * s; out[3] = a03 * c - a23 * s;
            out[8] = a00 * s + a20 * c; out[9] = a01 * s + a21 * c;
            out[10] = a02 * s + a22 * c; out[11] = a03 * s + a23 * c;
            return out;
        },
        rotateZ(out, a, rad) {
            const s = Math.sin(rad), c = Math.cos(rad);
            const a00 = a[0], a01 = a[1], a02 = a[2], a03 = a[3];
            const a10 = a[4], a11 = a[5], a12 = a[6], a13 = a[7];
            out[0] = a00 * c + a10 * s; out[1] = a01 * c + a11 * s;
            out[2] = a02 * c + a12 * s; out[3] = a03 * c + a13 * s;
            out[4] = a10 * c - a00 * s; out[5] = a11 * c - a01 * s;
            out[6] = a12 * c - a02 * s; out[7] = a13 * c - a03 * s;
            return out;
        },
        normalFromMat4(out, a) {
            const a00 = a[0], a01 = a[1], a02 = a[2];
            const a10 = a[4], a11 = a[5], a12 = a[6];
            const a20 = a[8], a21 = a[9], a22 = a[10];
            const b01 = a22 * a11 - a12 * a21;
            const b11 = -a22 * a10 + a12 * a20;
            const b21 = a21 * a10 - a11 * a20;
            let d = a00 * b01 + a01 * b11 + a02 * b21;
            if (!d) return null;
            d = 1.0 / d;
            out[0] = b01 * d; out[1] = (-a22 * a01 + a02 * a21) * d; out[2] = (a12 * a01 - a02 * a11) * d;
            out[3] = b11 * d; out[4] = (a22 * a00 - a02 * a20) * d; out[5] = (-a12 * a00 + a02 * a10) * d;
            out[6] = b21 * d; out[7] = (-a21 * a00 + a01 * a20) * d; out[8] = (a11 * a00 - a01 * a10) * d;
            return out;
        }
    },

    // Procedural Mesh Library
    createMeshes() {
        function computeNormals(positions, indices) {
            const normals = new Float32Array(positions.length);
            for (let i = 0; i < indices.length; i += 3) {
                const i0 = indices[i] * 3;
                const i1 = indices[i + 1] * 3;
                const i2 = indices[i + 2] * 3;
                const ax = positions[i1] - positions[i0];
                const ay = positions[i1 + 1] - positions[i0 + 1];
                const az = positions[i1 + 2] - positions[i0 + 2];
                const bx = positions[i2] - positions[i0];
                const by = positions[i2 + 1] - positions[i0 + 1];
                const bz = positions[i2 + 2] - positions[i0 + 2];
                const nx = ay * bz - az * by;
                const ny = az * bx - ax * bz;
                const nz = ax * by - ay * bx;
                normals[i0] += nx; normals[i0 + 1] += ny; normals[i0 + 2] += nz;
                normals[i1] += nx; normals[i1 + 1] += ny; normals[i1 + 2] += nz;
                normals[i2] += nx; normals[i2 + 1] += ny; normals[i2 + 2] += nz;
            }
            for (let i = 0; i < normals.length; i += 3) {
                const len = Math.hypot(normals[i], normals[i + 1], normals[i + 2]) || 1;
                normals[i] /= len; normals[i + 1] /= len; normals[i + 2] /= len;
            }
            return normals;
        }

        function buildWireIndices(indices) {
            const lines = [];
            for (let i = 0; i < indices.length; i += 3) {
                lines.push(indices[i], indices[i + 1]);
                lines.push(indices[i + 1], indices[i + 2]);
                lines.push(indices[i + 2], indices[i]);
            }
            return new Uint16Array(lines);
        }

        // 1. Quantum Trefoil Knot
        function makeKnot() {
            const tubularSegments = 120, radialSegments = 16, radius = 0.95, tube = 0.28, p = 2, q = 3;
            const positions = [];
            const indices = [];
            for (let i = 0; i <= tubularSegments; ++i) {
                const u = (i / tubularSegments) * p * Math.PI * 2;
                const r = 0.5 * (2 + Math.sin(q * (u / p)));
                const cx = radius * r * Math.cos(u);
                const cy = radius * r * Math.sin(u);
                const cz = radius * Math.sin(q * (u / p)) * 0.8;
                const uNext = ((i + 1) / tubularSegments) * p * Math.PI * 2;
                const rNext = 0.5 * (2 + Math.sin(q * (uNext / p)));
                const tx = radius * rNext * Math.cos(uNext) - cx;
                const ty = radius * rNext * Math.sin(uNext) - cy;
                const tz = radius * Math.sin(q * (uNext / p)) * 0.8 - cz;
                const tLen = Math.hypot(tx, ty, tz) || 1;
                const T = [tx / tLen, ty / tLen, tz / tLen];
                let N = [-T[1], T[0], 0];
                const nLen = Math.hypot(N[0], N[1], N[2]);
                if (nLen < 0.001) N = [1, 0, 0];
                else { N[0] /= nLen; N[1] /= nLen; N[2] /= nLen; }
                const B = [T[1] * N[2] - T[2] * N[1], T[2] * N[0] - T[0] * N[2], T[0] * N[1] - T[1] * N[0]];

                for (let j = 0; j <= radialSegments; ++j) {
                    const v = (j / radialSegments) * Math.PI * 2;
                    const cx2 = Math.cos(v) * tube;
                    const cy2 = Math.sin(v) * tube;
                    positions.push(
                        cx + (N[0] * cx2 + B[0] * cy2),
                        cy + (N[1] * cx2 + B[1] * cy2),
                        cz + (N[2] * cx2 + B[2] * cy2)
                    );
                }
            }
            for (let j = 1; j <= tubularSegments; j++) {
                for (let i = 1; i <= radialSegments; i++) {
                    const a = (radialSegments + 1) * (j - 1) + (i - 1);
                    const b = (radialSegments + 1) * j + (i - 1);
                    const c = (radialSegments + 1) * j + i;
                    const d = (radialSegments + 1) * (j - 1) + i;
                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }
            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 2. Sci-Fi Reactor Core: Central glowing core + orbital rings + node capacitors
        function makeSciFiCore() {
            const positions = [];
            const indices = [];

            function addBox(cx, cy, cz, sx, sy, sz) {
                const base = positions.length / 3;
                const hx = sx/2, hy = sy/2, hz = sz/2;
                const v = [
                    cx-hx, cy-hy, cz+hz,  cx+hx, cy-hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy-hy, cz-hz,
                    cx-hx, cy+hy, cz-hz,  cx-hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz-hz,
                    cx-hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz+hz,  cx-hx, cy-hy, cz+hz,
                    cx+hx, cy-hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy-hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy-hy, cz+hz,  cx-hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz-hz
                ];
                for (let k = 0; k < v.length; k++) positions.push(v[k]);
                for (let f = 0; f < 6; f++) {
                    const fb = base + f * 4;
                    indices.push(fb, fb+1, fb+2, fb, fb+2, fb+3);
                }
            }

            function addTorus(R, r, segs, tsegs, rotX, rotY) {
                const base = positions.length / 3;
                for (let i = 0; i <= segs; i++) {
                    const u = (i / segs) * Math.PI * 2;
                    for (let j = 0; j <= tsegs; j++) {
                        const v = (j / tsegs) * Math.PI * 2;
                        let x = (R + r * Math.cos(v)) * Math.cos(u);
                        let y = (R + r * Math.cos(v)) * Math.sin(u);
                        let z = r * Math.sin(v);
                        if (rotX) {
                            const y1 = y * Math.cos(rotX) - z * Math.sin(rotX);
                            const z1 = y * Math.sin(rotX) + z * Math.cos(rotX);
                            y = y1; z = z1;
                        }
                        if (rotY) {
                            const x1 = x * Math.cos(rotY) + z * Math.sin(rotY);
                            const z1 = -x * Math.sin(rotY) + z * Math.cos(rotY);
                            x = x1; z = z1;
                        }
                        positions.push(x, y, z);
                    }
                }
                for (let i = 0; i < segs; i++) {
                    for (let j = 0; j < tsegs; j++) {
                        const a = base + i * (tsegs + 1) + j;
                        const b = base + (i + 1) * (tsegs + 1) + j;
                        const c = base + (i + 1) * (tsegs + 1) + (j + 1);
                        const d = base + i * (tsegs + 1) + (j + 1);
                        indices.push(a, b, d);
                        indices.push(b, c, d);
                    }
                }
            }

            // Central core octahedron / box cluster
            addBox(0, 0, 0, 0.9, 0.9, 0.9);
            addBox(0, 0, 0, 1.3, 0.4, 0.4);
            addBox(0, 0, 0, 0.4, 1.3, 0.4);
            addBox(0, 0, 0, 0.4, 0.4, 1.3);

            // Orbital Rings
            addTorus(1.5, 0.05, 48, 8, Math.PI / 4, 0);
            addTorus(1.8, 0.04, 48, 8, -Math.PI / 4, Math.PI / 3);

            // 8 Capacitors
            const d = 1.05;
            for (let sx = -1; sx <= 1; sx += 2) {
                for (let sy = -1; sy <= 1; sy += 2) {
                    for (let sz = -1; sz <= 1; sz += 2) {
                        addBox(sx * d, sy * d, sz * d, 0.22, 0.22, 0.22);
                    }
                }
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 3. Mechanical Architectural Tower
        function makeMechTower() {
            const positions = [];
            const indices = [];

            function addCylinder(cy, rTop, rBot, height, segments) {
                const base = positions.length / 3;
                const hh = height / 2;
                for (let i = 0; i <= segments; i++) {
                    const th = (i / segments) * Math.PI * 2;
                    const cos = Math.cos(th), sin = Math.sin(th);
                    positions.push(cos * rTop, cy + hh, sin * rTop);
                    positions.push(cos * rBot, cy - hh, sin * rBot);
                }
                for (let i = 0; i < segments; i++) {
                    const i0 = base + i * 2;
                    const i1 = base + i * 2 + 1;
                    const i2 = base + (i + 1) * 2 + 1;
                    const i3 = base + (i + 1) * 2;
                    indices.push(i0, i1, i3);
                    indices.push(i1, i2, i3);
                }
            }

            function addPillar(px, pz, r, h) {
                addCylinder(0, r, r, h, 8);
                const last = positions.length - 8 * 2 * 3;
                for (let k = last; k < positions.length; k += 3) {
                    positions[k] += px;
                    positions[k + 2] += pz;
                }
            }

            // Base platform
            addCylinder(-1.1, 1.4, 1.5, 0.35, 12);
            // Middle ring
            addCylinder(-0.2, 1.1, 1.1, 0.3, 12);
            // Top observation ring
            addCylinder(0.8, 0.9, 0.7, 0.4, 8);
            // Central mast
            addCylinder(0, 0.35, 0.35, 2.5, 12);
            // 4 Structural Truss Pillars
            const pr = 0.95;
            for (let i = 0; i < 4; i++) {
                const a = (i / 4) * Math.PI * 2;
                addPillar(Math.cos(a) * pr, Math.sin(a) * pr, 0.08, 2.2);
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 4. Product Monolith: Sleek smartphone/chassis with beveled corners & display face
        function makeProductModel() {
            const positions = [];
            const indices = [];

            function addBox(cx, cy, cz, sx, sy, sz) {
                const base = positions.length / 3;
                const hx = sx/2, hy = sy/2, hz = sz/2;
                const v = [
                    cx-hx, cy-hy, cz+hz,  cx+hx, cy-hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy-hy, cz-hz,
                    cx-hx, cy+hy, cz-hz,  cx-hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz-hz,
                    cx-hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz+hz,  cx-hx, cy-hy, cz+hz,
                    cx+hx, cy-hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy-hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy-hy, cz+hz,  cx-hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz-hz
                ];
                for (let k = 0; k < v.length; k++) positions.push(v[k]);
                for (let f = 0; f < 6; f++) {
                    const fb = base + f * 4;
                    indices.push(fb, fb+1, fb+2, fb, fb+2, fb+3);
                }
            }

            // Chassis
            addBox(0, 0, 0, 1.4, 2.4, 0.18);
            // Display Screen Face
            addBox(0, 0, 0.10, 1.26, 2.22, 0.04);
            // Rear Camera Island
            addBox(-0.35, 0.72, -0.12, 0.52, 0.72, 0.08);
            // Camera Lenses
            addBox(-0.35, 0.85, -0.17, 0.18, 0.18, 0.04);
            addBox(-0.35, 0.58, -0.17, 0.18, 0.18, 0.04);

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 5. Crystal Octahedron Gem
        function makeCrystalGem() {
            const positions = [
                0, 1.45, 0,   1.1, 0, 0,    0, 0, 1.1,
                0, 1.45, 0,   0, 0, 1.1,    -1.1, 0, 0,
                0, 1.45, 0,   -1.1, 0, 0,   0, 0, -1.1,
                0, 1.45, 0,   0, 0, -1.1,   1.1, 0, 0,
                0, -1.45, 0,  0, 0, 1.1,    1.1, 0, 0,
                0, -1.45, 0,  -1.1, 0, 0,   0, 0, 1.1,
                0, -1.45, 0,  0, 0, -1.1,   -1.1, 0, 0,
                0, -1.45, 0,  1.1, 0, 0,    0, 0, -1.1,
                // Inner ring facets
                0.55, 0, 0.55,  -0.55, 0, 0.55,  -0.55, 0, -0.55,
                0.55, 0, 0.55,  -0.55, 0, -0.55,  0.55, 0, -0.55
            ];
            const indices = [];
            for (let i = 0; i < positions.length / 3; i++) indices.push(i);
            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 6. Quantum Geodesic Sphere
        function makeGeodesicSphere() {
            const positions = [];
            const indices = [];
            const t = (1.0 + Math.sqrt(5.0)) / 2.0;

            const baseVerts = [
                -1,  t,  0,   1,  t,  0,  -1, -t,  0,   1, -t,  0,
                 0, -1,  t,   0,  1,  t,   0, -1, -t,   0,  1, -t,
                 t,  0, -1,   t,  0,  1,  -t,  0, -1,  -t,  0,  1
            ];
            const baseFaces = [
                0, 11, 5,   0, 5, 1,    0, 1, 7,    0, 7, 10,   0, 10, 11,
                1, 5, 9,    5, 11, 4,   11, 10, 2,  10, 7, 6,   7, 1, 8,
                3, 9, 4,    3, 4, 2,    3, 2, 6,    3, 6, 8,    3, 8, 9,
                4, 9, 5,    2, 4, 11,   6, 2, 10,   8, 6, 7,    9, 8, 1
            ];

            const radius = 1.35;
            for (let i = 0; i < baseFaces.length; i += 3) {
                const iA = baseFaces[i] * 3;
                const iB = baseFaces[i + 1] * 3;
                const iC = baseFaces[i + 2] * 3;
                const vA = [baseVerts[iA], baseVerts[iA + 1], baseVerts[iA + 2]];
                const vB = [baseVerts[iB], baseVerts[iB + 1], baseVerts[iB + 2]];
                const vC = [baseVerts[iC], baseVerts[iC + 1], baseVerts[iC + 2]];

                const vAB = [(vA[0]+vB[0])/2, (vA[1]+vB[1])/2, (vA[2]+vB[2])/2];
                const vBC = [(vB[0]+vC[0])/2, (vB[1]+vC[1])/2, (vB[2]+vC[2])/2];
                const vCA = [(vC[0]+vA[0])/2, (vC[1]+vA[1])/2, (vC[2]+vA[2])/2];

                function pushTri(p1, p2, p3) {
                    const idxBase = positions.length / 3;
                    const tri = [p1, p2, p3];
                    for (let k = 0; k < 3; k++) {
                        const len = Math.hypot(tri[k][0], tri[k][1], tri[k][2]) || 1;
                        positions.push(
                            (tri[k][0] / len) * radius,
                            (tri[k][1] / len) * radius,
                            (tri[k][2] / len) * radius
                        );
                    }
                    indices.push(idxBase, idxBase + 1, idxBase + 2);
                }

                pushTri(vA, vAB, vCA);
                pushTri(vB, vBC, vAB);
                pushTri(vC, vCA, vBC);
                pushTri(vAB, vBC, vCA);
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 7. Quantum Gyroscope: 3 concentric gimbal rings with central core
        function makeGyroscope() {
            const positions = [];
            const indices = [];

            function addRing(R, r, segs, tsegs, rotX, rotY, rotZ) {
                const base = positions.length / 3;
                for (let i = 0; i <= segs; i++) {
                    const u = (i / segs) * Math.PI * 2;
                    for (let j = 0; j <= tsegs; j++) {
                        const v = (j / tsegs) * Math.PI * 2;
                        let x = (R + r * Math.cos(v)) * Math.cos(u);
                        let y = (R + r * Math.cos(v)) * Math.sin(u);
                        let z = r * Math.sin(v);
                        if (rotX) {
                            const y1 = y * Math.cos(rotX) - z * Math.sin(rotX);
                            const z1 = y * Math.sin(rotX) + z * Math.cos(rotX);
                            y = y1; z = z1;
                        }
                        if (rotY) {
                            const x1 = x * Math.cos(rotY) + z * Math.sin(rotY);
                            const z1 = -x * Math.sin(rotY) + z * Math.cos(rotY);
                            x = x1; z = z1;
                        }
                        if (rotZ) {
                            const x1 = x * Math.cos(rotZ) - y * Math.sin(rotZ);
                            const y1 = x * Math.sin(rotZ) + y * Math.cos(rotZ);
                            x = x1; y = y1;
                        }
                        positions.push(x, y, z);
                    }
                }
                for (let i = 0; i < segs; i++) {
                    for (let j = 0; j < tsegs; j++) {
                        const a = base + i * (tsegs + 1) + j;
                        const b = base + (i + 1) * (tsegs + 1) + j;
                        const c = base + (i + 1) * (tsegs + 1) + (j + 1);
                        const d = base + i * (tsegs + 1) + (j + 1);
                        indices.push(a, b, d);
                        indices.push(b, c, d);
                    }
                }
            }

            addRing(1.58, 0.065, 48, 12, 0, 0, 0);
            addRing(1.22, 0.06, 42, 10, Math.PI / 3, 0, 0);
            addRing(0.92, 0.055, 36, 10, 0, Math.PI / 2.2, 0);

            // Center core
            const cBase = positions.length / 3;
            const sSegs = 16;
            for (let i = 0; i <= sSegs; i++) {
                const phi = (i / sSegs) * Math.PI;
                for (let j = 0; j <= sSegs; j++) {
                    const theta = (j / sSegs) * Math.PI * 2;
                    const r = 0.42;
                    positions.push(
                        r * Math.sin(phi) * Math.cos(theta),
                        r * Math.cos(phi),
                        r * Math.sin(phi) * Math.sin(theta)
                    );
                }
            }
            for (let i = 0; i < sSegs; i++) {
                for (let j = 0; j < sSegs; j++) {
                    const a = cBase + i * (sSegs + 1) + j;
                    const b = cBase + (i + 1) * (sSegs + 1) + j;
                    const c = cBase + (i + 1) * (sSegs + 1) + (j + 1);
                    const d = cBase + i * (sSegs + 1) + (j + 1);
                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 8. 4D Tesseract Matrix Core: Outer cube connected via 8 struts to inner cube
        function makeTesseract() {
            const positions = [];
            const indices = [];

            function addStrut(p1, p2, r) {
                const dx = p2[0] - p1[0], dy = p2[1] - p1[1], dz = p2[2] - p1[2];
                const len = Math.hypot(dx, dy, dz) || 1;
                const dir = [dx/len, dy/len, dz/len];
                let perp = [-dir[1], dir[0], 0];
                if (Math.hypot(perp[0], perp[1]) < 0.01) perp = [0, -dir[2], dir[1]];
                const pLen = Math.hypot(perp[0], perp[1], perp[2]) || 1;
                perp = [perp[0]/pLen, perp[1]/pLen, perp[2]/pLen];
                const perp2 = [
                    dir[1]*perp[2] - dir[2]*perp[1],
                    dir[2]*perp[0] - dir[0]*perp[2],
                    dir[0]*perp[1] - dir[1]*perp[0]
                ];

                const base = positions.length / 3;
                const segs = 6;
                for (let t = 0; t <= 1; t++) {
                    const origin = (t === 0) ? p1 : p2;
                    for (let i = 0; i < segs; i++) {
                        const angle = (i / segs) * Math.PI * 2;
                        positions.push(
                            origin[0] + (perp[0] * Math.cos(angle) + perp2[0] * Math.sin(angle)) * r,
                            origin[1] + (perp[1] * Math.cos(angle) + perp2[1] * Math.sin(angle)) * r,
                            origin[2] + (perp[2] * Math.cos(angle) + perp2[2] * Math.sin(angle)) * r
                        );
                    }
                }
                for (let i = 0; i < segs; i++) {
                    const next = (i + 1) % segs;
                    indices.push(base + i, base + segs + i, base + next);
                    indices.push(base + segs + i, base + segs + next, base + next);
                }
            }

            const s1 = 1.35;
            const outer = [
                [-s1,-s1,-s1], [s1,-s1,-s1], [s1,s1,-s1], [-s1,s1,-s1],
                [-s1,-s1,s1],  [s1,-s1,s1],  [s1,s1,s1],  [-s1,s1,s1]
            ];
            const s2 = 0.65;
            const inner = [
                [-s2,-s2,-s2], [s2,-s2,-s2], [s2,s2,-s2], [-s2,s2,-s2],
                [-s2,-s2,s2],  [s2,-s2,s2],  [s2,s2,s2],  [-s2,s2,s2]
            ];

            const edges = [
                [0,1],[1,2],[2,3],[3,0], [4,5],[5,6],[6,7],[7,4],
                [0,4],[1,5],[2,6],[3,7]
            ];

            for (let e = 0; e < edges.length; e++) addStrut(outer[edges[e][0]], outer[edges[e][1]], 0.05);
            for (let e = 0; e < edges.length; e++) addStrut(inner[edges[e][0]], inner[edges[e][1]], 0.04);
            for (let i = 0; i < 8; i++) addStrut(outer[i], inner[i], 0.035);

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 9. DNA Double Helix
        function makeDNA() {
            const positions = [];
            const indices = [];

            function addSphere(cx, cy, cz, r, segs) {
                const base = positions.length / 3;
                for (let i = 0; i <= segs; i++) {
                    const phi = (i / segs) * Math.PI;
                    for (let j = 0; j <= segs; j++) {
                        const theta = (j / segs) * Math.PI * 2;
                        positions.push(
                            cx + r * Math.sin(phi) * Math.cos(theta),
                            cy + r * Math.cos(phi),
                            cz + r * Math.sin(phi) * Math.sin(theta)
                        );
                    }
                }
                for (let i = 0; i < segs; i++) {
                    for (let j = 0; j < segs; j++) {
                        const a = base + i * (segs + 1) + j;
                        const b = base + (i + 1) * (segs + 1) + j;
                        const c = base + (i + 1) * (segs + 1) + (j + 1);
                        const d = base + i * (segs + 1) + (j + 1);
                        indices.push(a, b, d);
                        indices.push(b, c, d);
                    }
                }
            }

            function addCyl(p1, p2, r) {
                const dx = p2[0] - p1[0], dy = p2[1] - p1[1], dz = p2[2] - p1[2];
                const len = Math.hypot(dx, dy, dz) || 1;
                const dir = [dx/len, dy/len, dz/len];
                let perp = [-dir[1], dir[0], 0];
                if (Math.hypot(perp[0], perp[1]) < 0.01) perp = [0, -dir[2], dir[1]];
                const pLen = Math.hypot(perp[0], perp[1], perp[2]) || 1;
                perp = [perp[0]/pLen, perp[1]/pLen, perp[2]/pLen];
                const perp2 = [
                    dir[1]*perp[2] - dir[2]*perp[1],
                    dir[2]*perp[0] - dir[0]*perp[2],
                    dir[0]*perp[1] - dir[1]*perp[0]
                ];

                const base = positions.length / 3;
                const segs = 6;
                for (let t = 0; t <= 1; t++) {
                    const origin = (t === 0) ? p1 : p2;
                    for (let i = 0; i < segs; i++) {
                        const angle = (i / segs) * Math.PI * 2;
                        positions.push(
                            origin[0] + (perp[0] * Math.cos(angle) + perp2[0] * Math.sin(angle)) * r,
                            origin[1] + (perp[1] * Math.cos(angle) + perp2[1] * Math.sin(angle)) * r,
                            origin[2] + (perp[2] * Math.cos(angle) + perp2[2] * Math.sin(angle)) * r
                        );
                    }
                }
                for (let i = 0; i < segs; i++) {
                    const next = (i + 1) % segs;
                    indices.push(base + i, base + segs + i, base + next);
                    indices.push(base + segs + i, base + segs + next, base + next);
                }
            }

            const steps = 28;
            const height = 3.6;
            const radius = 0.95;
            const strand1 = [];
            const strand2 = [];

            for (let i = 0; i <= steps; i++) {
                const t = (i / steps);
                const y = (t - 0.5) * height;
                const angle = t * Math.PI * 4;
                const p1 = [radius * Math.cos(angle), y, radius * Math.sin(angle)];
                const p2 = [radius * Math.cos(angle + Math.PI), y, radius * Math.sin(angle + Math.PI)];
                strand1.push(p1);
                strand2.push(p2);
                addSphere(p1[0], p1[1], p1[2], 0.11, 8);
                addSphere(p2[0], p2[1], p2[2], 0.11, 8);
                if (i % 2 === 0) addCyl(p1, p2, 0.045);
            }
            for (let i = 0; i < steps; i++) {
                addCyl(strand1[i], strand1[i+1], 0.055);
                addCyl(strand2[i], strand2[i+1], 0.055);
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 10. Orbital Deep Space Satellite with Solar Wings & Parabolic Dish
        function makeSatellite() {
            const positions = [];
            const indices = [];

            function addBox(cx, cy, cz, sx, sy, sz) {
                const base = positions.length / 3;
                const hx = sx/2, hy = sy/2, hz = sz/2;
                const v = [
                    cx-hx, cy-hy, cz+hz,  cx+hx, cy-hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy-hy, cz-hz,
                    cx-hx, cy+hy, cz-hz,  cx-hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz-hz,
                    cx-hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz+hz,  cx-hx, cy-hy, cz+hz,
                    cx+hx, cy-hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy-hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy-hy, cz+hz,  cx-hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz-hz
                ];
                for (let k = 0; k < v.length; k++) positions.push(v[k]);
                for (let f = 0; f < 6; f++) {
                    const fb = base + f * 4;
                    indices.push(fb, fb+1, fb+2, fb, fb+2, fb+3);
                }
            }

            addBox(0, 0, 0, 0.9, 1.4, 0.9);
            addBox(0, 0.82, 0, 0.55, 0.3, 0.55);
            addBox(0, -0.85, 0, 0.45, 0.35, 0.45);

            // Left & Right Solar Arrays
            addBox(-0.75, 0, 0, 0.6, 0.08, 0.08);
            addBox(-1.95, 0, 0, 1.8, 0.9, 0.04);
            addBox(0.75, 0, 0, 0.6, 0.08, 0.08);
            addBox(1.95, 0, 0, 1.8, 0.9, 0.04);

            // Parabolic Comms Dish
            const dBase = positions.length / 3;
            const dishSegs = 20;
            for (let i = 0; i <= dishSegs; i++) {
                const theta = (i / dishSegs) * Math.PI * 2;
                for (let j = 0; j <= 6; j++) {
                    const r = (j / 6) * 0.65;
                    const z = 0.52 + Math.pow(r / 0.65, 2) * 0.22;
                    positions.push(r * Math.cos(theta), 0.3 + r * Math.sin(theta), z);
                }
            }
            for (let i = 0; i < dishSegs; i++) {
                for (let j = 0; j < 6; j++) {
                    const a = dBase + i * 7 + j;
                    const b = dBase + (i + 1) * 7 + j;
                    const c = dBase + (i + 1) * 7 + (j + 1);
                    const d = dBase + i * 7 + (j + 1);
                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 11. Hyper-Torus Ring with Harmonic Ripple Waves
        function makeHyperTorus() {
            const tubularSegments = 54, radialSegments = 20, R = 1.15, tube = 0.42;
            const positions = [];
            const indices = [];

            for (let i = 0; i <= tubularSegments; i++) {
                const u = (i / tubularSegments) * Math.PI * 2;
                for (let j = 0; j <= radialSegments; j++) {
                    const v = (j / radialSegments) * Math.PI * 2;
                    const wave = 1.0 + 0.16 * Math.sin(5 * u) * Math.cos(3 * v);
                    const rEff = tube * wave;
                    positions.push(
                        (R + rEff * Math.cos(v)) * Math.cos(u),
                        (R + rEff * Math.cos(v)) * Math.sin(u),
                        rEff * Math.sin(v)
                    );
                }
            }
            for (let i = 0; i < tubularSegments; i++) {
                for (let j = 0; j < radialSegments; j++) {
                    const a = i * (radialSegments + 1) + j;
                    const b = (i + 1) * (radialSegments + 1) + j;
                    const c = (i + 1) * (radialSegments + 1) + (j + 1);
                    const d = i * (radialSegments + 1) + (j + 1);
                    indices.push(a, b, d);
                    indices.push(b, c, d);
                }
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 12. 12-Point Stellated Celestial Star Polyhedron
        function makeCelestialStar() {
            const positions = [];
            const indices = [];

            const t = (1.0 + Math.sqrt(5.0)) / 2.0;
            const baseVerts = [
                [-1,  t,  0], [ 1,  t,  0], [-1, -t,  0], [ 1, -t,  0],
                [ 0, -1,  t], [ 0,  1,  t], [ 0, -1, -t], [ 0,  1, -t],
                [ t,  0, -1], [ t,  0,  1], [-t,  0, -1], [-t,  0,  1]
            ];
            const faces = [
                0, 11, 5,   0, 5, 1,    0, 1, 7,    0, 7, 10,   0, 10, 11,
                1, 5, 9,    5, 11, 4,   11, 10, 2,  10, 7, 6,   7, 1, 8,
                3, 9, 4,    3, 4, 2,    3, 2, 6,    3, 6, 8,    3, 8, 9,
                4, 9, 5,    2, 4, 11,   6, 2, 10,   8, 6, 7,    9, 8, 1
            ];

            const spikeHeight = 1.95;
            const coreRadius = 0.85;

            function normV(v, r) {
                const l = Math.hypot(v[0], v[1], v[2]) || 1;
                return [v[0]/l * r, v[1]/l * r, v[2]/l * r];
            }

            for (let f = 0; f < faces.length; f += 3) {
                const iA = faces[f], iB = faces[f+1], iC = faces[f+2];
                const pA = normV(baseVerts[iA], coreRadius);
                const pB = normV(baseVerts[iB], coreRadius);
                const pC = normV(baseVerts[iC], coreRadius);

                const cx = (pA[0] + pB[0] + pC[0]) / 3;
                const cy = (pA[1] + pB[1] + pC[1]) / 3;
                const cz = (pA[2] + pB[2] + pC[2]) / 3;
                const apex = normV([cx, cy, cz], spikeHeight);

                const base = positions.length / 3;
                positions.push(pA[0], pA[1], pA[2]);
                positions.push(pB[0], pB[1], pB[2]);
                positions.push(pC[0], pC[1], pC[2]);
                positions.push(apex[0], apex[1], apex[2]);

                indices.push(base, base+1, base+3);
                indices.push(base+1, base+2, base+3);
                indices.push(base+2, base, base+3);
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 13. Quantum Processor Chip: Silicon substrate, microwave coplanar waveguides & transmon qubits
        function makeQuantumChip() {
            const positions = [];
            const indices = [];

            function addBox(cx, cy, cz, sx, sy, sz) {
                const base = positions.length / 3;
                const hx = sx/2, hy = sy/2, hz = sz/2;
                const v = [
                    cx-hx, cy-hy, cz+hz,  cx+hx, cy-hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy-hy, cz-hz,
                    cx-hx, cy+hy, cz-hz,  cx-hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy+hy, cz-hz,
                    cx-hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz-hz,  cx+hx, cy-hy, cz+hz,  cx-hx, cy-hy, cz+hz,
                    cx+hx, cy-hy, cz-hz,  cx+hx, cy+hy, cz-hz,  cx+hx, cy+hy, cz+hz,  cx+hx, cy-hy, cz+hz,
                    cx-hx, cy-hy, cz-hz,  cx-hx, cy-hy, cz+hz,  cx-hx, cy+hy, cz+hz,  cx-hx, cy+hy, cz-hz
                ];
                for (let k = 0; k < v.length; k++) positions.push(v[k]);
                for (let f = 0; f < 6; f++) {
                    const fb = base + f * 4;
                    indices.push(fb, fb+1, fb+2, fb, fb+2, fb+3);
                }
            }

            addBox(0, -0.1, 0, 2.6, 0.16, 2.6);
            addBox(0, 0.01, 0, 2.3, 0.05, 2.3);
            addBox(0, 0.06, 0.85, 0.12, 0.06, 0.8);
            addBox(0, 0.06, -0.85, 0.12, 0.06, 0.8);
            addBox(0.85, 0.06, 0, 0.8, 0.06, 0.12);
            addBox(-0.85, 0.06, 0, 0.8, 0.06, 0.12);

            for (let gx = -0.52; gx <= 0.52; gx += 0.26) {
                for (let gz = -0.52; gz <= 0.52; gz += 0.26) {
                    addBox(gx, 0.08, gz, 0.16, 0.1, 0.16);
                    addBox(gx, 0.14, gz, 0.06, 0.06, 0.06);
                }
            }

            for (let i = -3; i <= 3; i += 2) {
                addBox(i * 0.32, -0.02, 1.25, 0.14, 0.08, 0.18);
                addBox(i * 0.32, -0.02, -1.25, 0.14, 0.08, 0.18);
                addBox(1.25, -0.02, i * 0.32, 0.18, 0.08, 0.14);
                addBox(-1.25, -0.02, i * 0.32, 0.18, 0.08, 0.14);
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 14. Gravitational Singularity (Black Hole): Event horizon, warped accretion disk & relativistic jets
        function makeBlackHole() {
            const positions = [];
            const indices = [];

            const lats = 16, lons = 24, sR = 0.72;
            for (let i = 0; i <= lats; i++) {
                const theta = (i * Math.PI) / lats;
                const sinTheta = Math.sin(theta);
                const cosTheta = Math.cos(theta);
                for (let j = 0; j <= lons; j++) {
                    const phi = (j * 2 * Math.PI) / lons;
                    positions.push(sR * sinTheta * Math.cos(phi), sR * cosTheta, sR * sinTheta * Math.sin(phi));
                }
            }
            for (let i = 0; i < lats; i++) {
                for (let j = 0; j < lons; j++) {
                    const first = i * (lons + 1) + j;
                    const second = first + lons + 1;
                    indices.push(first, second, first + 1, second, second + 1, first + 1);
                }
            }

            const diskRings = 4;
            const diskSegs = 48;
            for (let r = 0; r < diskRings; r++) {
                const rIn = 0.95 + r * 0.38;
                const rOut = rIn + 0.34;
                const base = positions.length / 3;
                for (let s = 0; s <= diskSegs; s++) {
                    const a = (s * 2 * Math.PI) / diskSegs;
                    const ca = Math.cos(a), sa = Math.sin(a);
                    const warpIn = Math.sin(a * 2.0) * 0.18;
                    const warpOut = Math.sin(a * 2.0) * 0.28;
                    positions.push(ca * rIn, warpIn, sa * rIn);
                    positions.push(ca * rOut, warpOut, sa * rOut);
                }
                for (let s = 0; s < diskSegs; s++) {
                    const b = base + s * 2;
                    indices.push(b, b + 1, b + 2, b + 1, b + 3, b + 2);
                }
            }

            function addJet(dir) {
                const jetSegs = 14;
                const jetLen = 2.4;
                const base = positions.length / 3;
                for (let i = 0; i <= jetSegs; i++) {
                    const t = i / jetSegs;
                    const y = (0.7 + t * (jetLen - 0.7)) * dir;
                    const rad = 0.22 * (1.0 - t * 0.75);
                    for (let j = 0; j <= 8; j++) {
                        const a = (j * 2 * Math.PI) / 8;
                        positions.push(Math.cos(a) * rad, y, Math.sin(a) * rad);
                    }
                }
                for (let i = 0; i < jetSegs; i++) {
                    for (let j = 0; j < 8; j++) {
                        const b = base + i * 9 + j;
                        indices.push(b, b + 9, b + 1, b + 9, b + 10, b + 1);
                    }
                }
            }
            addJet(1);
            addJet(-1);

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 15. Parametric Möbius Strip
        function makeMobius() {
            const positions = [];
            const indices = [];
            const uSteps = 96;
            const vSteps = 6;
            const R = 1.35;
            const w = 0.42;

            for (let i = 0; i <= uSteps; i++) {
                const u = (i / uSteps) * Math.PI * 2;
                const cu = Math.cos(u), su = Math.sin(u);
                const cu2 = Math.cos(u / 2), su2 = Math.sin(u / 2);

                for (let j = 0; j <= vSteps; j++) {
                    const v = ((j / vSteps) - 0.5) * 2 * w;
                    const x = (R + v * cu2) * cu;
                    const z = (R + v * cu2) * su;
                    const y = v * su2;
                    positions.push(x, y, z);
                }
            }

            for (let i = 0; i < uSteps; i++) {
                for (let j = 0; j < vSteps; j++) {
                    const first = i * (vSteps + 1) + j;
                    const second = first + (vSteps + 1);
                    indices.push(first, second, first + 1, second, second + 1, first + 1);
                }
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        // 16. Dyson Swarm Orbital Megastructure: Ring conduit, stellar core & hexagonal solar collector arrays
        function makeDysonSwarm() {
            const positions = [];
            const indices = [];

            const lats = 14, lons = 20, sR = 0.68;
            for (let i = 0; i <= lats; i++) {
                const theta = (i * Math.PI) / lats;
                const sinTheta = Math.sin(theta);
                const cosTheta = Math.cos(theta);
                for (let j = 0; j <= lons; j++) {
                    const phi = (j * 2 * Math.PI) / lons;
                    positions.push(sR * sinTheta * Math.cos(phi), sR * cosTheta, sR * sinTheta * Math.sin(phi));
                }
            }
            for (let i = 0; i < lats; i++) {
                for (let j = 0; j < lons; j++) {
                    const first = i * (lons + 1) + j;
                    const second = first + lons + 1;
                    indices.push(first, second, first + 1, second, second + 1, first + 1);
                }
            }

            const ringSegs = 48, tubeSegs = 8, ringR = 1.65, tubeR = 0.08;
            const ringBase = positions.length / 3;
            for (let i = 0; i <= ringSegs; i++) {
                const u = (i / ringSegs) * Math.PI * 2;
                const cu = Math.cos(u), su = Math.sin(u);
                for (let j = 0; j <= tubeSegs; j++) {
                    const v = (j / tubeSegs) * Math.PI * 2;
                    const cv = Math.cos(v), sv = Math.sin(v);
                    positions.push((ringR + tubeR * cv) * cu, tubeR * sv, (ringR + tubeR * cv) * su);
                }
            }
            for (let i = 0; i < ringSegs; i++) {
                for (let j = 0; j < tubeSegs; j++) {
                    const a = ringBase + i * (tubeSegs + 1) + j;
                    const b = a + (tubeSegs + 1);
                    indices.push(a, b, a + 1, b, b + 1, a + 1);
                }
            }

            function addHexPanel(cx, cy, cz, size) {
                const base = positions.length / 3;
                positions.push(cx, cy, cz);
                for (let k = 0; k < 6; k++) {
                    const a = (k / 6) * Math.PI * 2;
                    positions.push(cx + Math.cos(a) * size, cy + Math.sin(a) * size * 0.4, cz + Math.sin(a) * size);
                }
                for (let k = 1; k <= 6; k++) {
                    const next = k === 6 ? 1 : k + 1;
                    indices.push(base, base + k, base + next);
                }
            }

            const swarmCount = 10;
            const swarmR = 2.05;
            for (let k = 0; k < swarmCount; k++) {
                const theta = (k / swarmCount) * Math.PI * 2;
                const px = Math.cos(theta) * swarmR;
                const py = Math.sin(theta) * swarmR * 0.65;
                const pz = Math.sin(theta) * swarmR * 0.75;
                addHexPanel(px, py, pz, 0.22);
            }

            const posArr = new Float32Array(positions);
            const idxArr = new Uint16Array(indices);
            return { positions: posArr, normals: computeNormals(posArr, idxArr), indices: idxArr, wireIndices: buildWireIndices(idxArr) };
        }

        function normalizeMesh(m) {
            let maxR = 0;
            for (let i = 0; i < m.positions.length; i += 3) {
                const r = Math.hypot(m.positions[i], m.positions[i+1], m.positions[i+2]);
                if (r > maxR) maxR = r;
            }
            if (maxR > 0.001) {
                const scale = 1.7 / maxR;
                for (let i = 0; i < m.positions.length; i++) {
                    m.positions[i] *= scale;
                }
            }
            return m;
        }

        const rawMeshes = {
            scifi: makeSciFiCore(),
            mech: makeMechTower(),
            quantum_chip: makeQuantumChip(),
            black_hole: makeBlackHole(),
            gyroscope: makeGyroscope(),
            tesseract: makeTesseract(),
            dna: makeDNA(),
            satellite: makeSatellite(),
            torus: makeHyperTorus(),
            star: makeCelestialStar(),
            mobius: makeMobius(),
            dyson_swarm: makeDysonSwarm(),
            crystal: makeCrystalGem(),
            geodesic: makeGeodesicSphere(),
            knot: makeKnot(),
            product: makeProductModel()
        };

        const meshes = {};
        for (const key in rawMeshes) {
            meshes[key] = normalizeMesh(rawMeshes[key]);
        }
        return meshes;
    },

    colors: {
        cyan: [0.024, 0.714, 0.831],     // #06B6D4 Cyber Cyan
        teal: [0.078, 0.722, 0.651],     // #14B8A6 Neon Teal
        purple: [0.659, 0.337, 0.988],   // #A855F7 Quantum Violet
        crimson: [0.957, 0.247, 0.369],  // #F43F5E Crimson Core
        gold: [0.961, 0.620, 0.043],     // #F59E0B Solar Gold
        emerald: [0.063, 0.725, 0.506],  // #10B981 Matrix Emerald
        ice: [0.220, 0.741, 0.973],      // #38BDF8 Ice Blue
        silver: [0.886, 0.910, 0.941],   // #E2E8F0 Titanium Silver
        amber: [0.984, 0.749, 0.141],    // #FBBF24 Amber Glow
        pink: [0.925, 0.282, 0.600],     // #EC4899 Galactic Pink
        orange: [0.976, 0.451, 0.086]    // #F97316 Golden Flame
    },

    initWebGL(gl, canvas, container, id, opts) {
        const VS_SOURCE = `
            attribute vec3 aPos;
            attribute vec3 aNorm;
            uniform mat4 uModel;
            uniform mat4 uView;
            uniform mat4 uProj;
            uniform mat3 uNormMat;
            varying vec3 vNorm;
            varying vec3 vPos;
            void main() {
                vNorm = normalize(uNormMat * aNorm);
                vec4 wPos = uModel * vec4(aPos, 1.0);
                vPos = wPos.xyz;
                gl_Position = uProj * uView * wPos;
            }
        `;

        const FS_SOURCE = `
            precision mediump float;
            varying vec3 vNorm;
            varying vec3 vPos;
            uniform vec3 uColor;
            uniform vec3 uColor2;
            uniform vec3 uLight1;
            uniform vec3 uLight2;
            uniform vec3 uEye;
            uniform float uMetal;
            uniform float uRough;
            uniform int uShading;
            uniform float uTime;
            void main() {
                vec3 N = normalize(vNorm);
                vec3 V = normalize(uEye - vPos);
                vec3 L1 = normalize(uLight1 - vPos);
                vec3 L2 = normalize(uLight2 - vPos);

                float diff1 = max(dot(N, L1), 0.0);
                float diff2 = max(dot(N, L2), 0.0);

                vec3 H1 = normalize(L1 + V);
                float spec1 = pow(max(dot(N, H1), 0.0), 32.0 * (1.0 - uRough * 0.7));
                vec3 specColor = vec3(1.0) * spec1 * (0.35 + 0.65 * uMetal);

                float rim = 1.0 - max(dot(V, N), 0.0);
                rim = smoothstep(0.4, 0.95, rim);
                vec3 rimColor = uColor * rim * 1.6;

                float grad = clamp((vPos.y + 1.4) / 2.8, 0.0, 1.0);
                vec3 baseColor = mix(uColor, uColor2, grad);

                vec3 ambient = 0.22 * baseColor;
                vec3 diffuse = (diff1 * 1.2 + diff2 * 0.5) * baseColor;

                if (uShading == 2) {
                    gl_FragColor = vec4(N * 0.5 + 0.5, 1.0);
                } else if (uShading == 1) {
                    gl_FragColor = vec4(ambient + rimColor + specColor, 1.0);
                } else if (uShading == 3) {
                    gl_FragColor = vec4(ambient * 1.4 + diffuse * 1.1 + specColor + rimColor * 0.8, 1.0);
                } else if (uShading == 4) {
                    float scan = sin(vPos.y * 36.0 + uTime * 3.5) * 0.18 + 0.82;
                    gl_FragColor = vec4((ambient + diffuse + specColor + rimColor) * scan, 1.0);
                } else {
                    gl_FragColor = vec4(ambient + diffuse + specColor + rimColor * 0.6, 1.0);
                }
            }
        `;

        function createShader(gl, type, source) {
            const s = gl.createShader(type);
            gl.shaderSource(s, source);
            gl.compileShader(s);
            if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {
                console.warn(gl.getShaderInfoLog(s));
                gl.deleteShader(s);
                return null;
            }
            return s;
        }

        const vs = createShader(gl, gl.VERTEX_SHADER, VS_SOURCE);
        const fs = createShader(gl, gl.FRAGMENT_SHADER, FS_SOURCE);
        const program = gl.createProgram();
        gl.attachShader(program, vs);
        gl.attachShader(program, fs);
        gl.linkProgram(program);

        const aPos = gl.getAttribLocation(program, 'aPos');
        const aNorm = gl.getAttribLocation(program, 'aNorm');
        const uModel = gl.getUniformLocation(program, 'uModel');
        const uView = gl.getUniformLocation(program, 'uView');
        const uProj = gl.getUniformLocation(program, 'uProj');
        const uNormMat = gl.getUniformLocation(program, 'uNormMat');
        const uColor = gl.getUniformLocation(program, 'uColor');
        const uColor2 = gl.getUniformLocation(program, 'uColor2');
        const uLight1 = gl.getUniformLocation(program, 'uLight1');
        const uLight2 = gl.getUniformLocation(program, 'uLight2');
        const uEye = gl.getUniformLocation(program, 'uEye');
        const uMetal = gl.getUniformLocation(program, 'uMetal');
        const uRough = gl.getUniformLocation(program, 'uRough');
        const uShading = gl.getUniformLocation(program, 'uShading');
        const uTime = gl.getUniformLocation(program, 'uTime');

        const meshes = this.createMeshes();
        const posBuf = gl.createBuffer();
        const normBuf = gl.createBuffer();
        const idxBuf = gl.createBuffer();
        const wireIdxBuf = gl.createBuffer();

        function hexToRgb(hex) {
            hex = (hex || '').replace(/^#/, '');
            if (hex.length === 3) hex = hex.split('').map(c => c + c).join('');
            if (hex.length !== 6) return [0.024, 0.714, 0.831];
            const num = parseInt(hex, 16);
            return [((num >> 16) & 255) / 255, ((num >> 8) & 255) / 255, (num & 255) / 255];
        }

        function getHarmonicComplement(rgb) {
            return [
                Math.max(0, Math.min(1, 1.0 - rgb[0] * 0.7 + rgb[2] * 0.3)),
                Math.max(0, Math.min(1, 1.0 - rgb[1] * 0.8 + rgb[0] * 0.2)),
                Math.max(0, Math.min(1, 1.0 - rgb[2] * 0.6 + rgb[1] * 0.4))
            ];
        }

        let currentGeoKey = opts.geo || 'scifi';
        if (!meshes[currentGeoKey]) currentGeoKey = 'scifi';

        let currentHex = '#06B6D4';
        if (opts.color && opts.color.startsWith('#')) {
            currentHex = opts.color.toUpperCase();
        } else if (opts.color && this.colors[opts.color]) {
            const c = this.colors[opts.color];
            const h = (v) => { const s = Math.round(v * 255).toString(16); return s.length === 1 ? '0' + s : s; };
            currentHex = ('#' + h(c[0]) + h(c[1]) + h(c[2])).toUpperCase();
        }
        let currentRgb = hexToRgb(currentHex);
        let currentComp = getHarmonicComplement(currentRgb);

        let isWireframe = false;
        let shadingMode = 0;
        let isPaused = false;
        let rotSpeed = opts.speed || 1.2;
        let orbitMode = 0;

        let rotX = 0.25;
        let rotY = 0.0;
        let zoom = 5.4;

        function loadMesh(m) {
            gl.bindBuffer(gl.ARRAY_BUFFER, posBuf);
            gl.bufferData(gl.ARRAY_BUFFER, m.positions, gl.STATIC_DRAW);
            gl.bindBuffer(gl.ARRAY_BUFFER, normBuf);
            gl.bufferData(gl.ARRAY_BUFFER, m.normals, gl.STATIC_DRAW);
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, idxBuf);
            gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, m.indices, gl.STATIC_DRAW);
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, wireIdxBuf);
            gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, m.wireIndices, gl.STATIC_DRAW);
        }

        loadMesh(meshes[currentGeoKey]);

        // State & FPS measurement
        let animId = null;
        let isVisible = true;
        let frameCount = 0;
        let lastFpsUpdate = performance.now();
        const fpsElem = document.getElementById(id + '-fps');
        const meshInfoElem = document.getElementById(id + '-mesh-info');

        const nameMap = {
            scifi: 'Sci-Fi Core', mech: 'Mech Spire', quantum_chip: 'Quantum Chip',
            black_hole: 'Singularity', gyroscope: 'Quantum Gyro', tesseract: '4D Tesseract',
            dna: 'DNA Helix', satellite: 'Satellite', torus: 'Ripple Torus',
            star: 'Stellated Star', mobius: 'Möbius Strip', dyson_swarm: 'Dyson Ring',
            crystal: 'Prism Gem', geodesic: 'Geodesic', knot: 'Trefoil Knot',
            product: 'Monolith'
        };

        function updateMeshInfo() {
            if (meshInfoElem && meshes[currentGeoKey]) {
                const polyCount = Math.round(meshes[currentGeoKey].indices.length / 3);
                const pretty = nameMap[currentGeoKey] || currentGeoKey;
                meshInfoElem.textContent = pretty + ' • ' + polyCount.toLocaleString() + ' Polys';
            }
        }
        updateMeshInfo();

        const studioContainer = document.getElementById(id + '-studio') || container;
        const liveCodeElem = document.getElementById(id + '-live-code');
        const copyBtnElem = document.getElementById(id + '-copy-btn');
        const hexInputElem = document.getElementById(id + '-hex-input');
        const colorPickerElem = document.getElementById(id + '-custom-picker');
        const swatchPreviewElem = document.getElementById(id + '-swatch-preview');
        const speedNumElem = document.getElementById(id + '-speed-num');
        const zoomNumElem = document.getElementById(id + '-zoom-num');
        const speedRangeElem = document.getElementById(id + '-speed-range');
        const zoomRangeElem = document.getElementById(id + '-zoom-range');

        function getTxsSnippet() {
            const mName = currentGeoKey;
            const spdStr = (Math.round(rotSpeed * 10) / 10).toFixed(1);

            return `# Clean & Short TechScript 3D Syntax\nshow 3D "${mName}":\n    color is "${currentHex}"\n    speed is ${spdStr}\nend\n\n# Or 1-line shorthand:\n# show 3D "${mName}" color "${currentHex}"`;
        }

        function updateLiveCode() {
            if (!liveCodeElem) return;
            const raw = getTxsSnippet();
            const lines = raw.split('\n').map(line => {
                if (line.trim().startsWith('#')) {
                    const clean = line.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
                    return `<span class="tok-com">${clean}</span>`;
                }
                let l = line.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
                l = l.replace(/"([^"]*)"/g, '<span class="tok-str">"$1"</span>');
                l = l.replace(/\b(show|model3D|scene3d|scene|called|add|end|enable|is|color|speed|shading|wireframe)\b/g, '<span class="tok-kw">$1</span>');
                l = l.replace(/\b(true|false)\b/g, '<span class="tok-bool">$1</span>');
                l = l.replace(/\b([0-9]+\.[0-9]+|[0-9]+)\b/g, '<span class="tok-num">$1</span>');
                return l;
            });
            liveCodeElem.innerHTML = lines.join('\n');
        }

        updateLiveCode();

        const mModel = this.mat4.create();
        const mView = this.mat4.create();
        const mProj = this.mat4.create();
        const mNorm = new Float32Array(9);

        function resize() {
            const rect = canvas.getBoundingClientRect();
            const w = Math.max(300, Math.floor(rect.width));
            const h = Math.max(260, Math.floor(rect.height || 480));
            const dpr = Math.min(window.devicePixelRatio || 1, 2);
            if (canvas.width !== w * dpr || canvas.height !== h * dpr) {
                canvas.width = w * dpr;
                canvas.height = h * dpr;
                gl.viewport(0, 0, canvas.width, canvas.height);
            }
        }

        resize();
        window.addEventListener('resize', resize);

        function renderFrame(now) {
            if (!isVisible) return;
            animId = requestAnimationFrame(renderFrame);

            // Real measured FPS
            frameCount++;
            if (now - lastFpsUpdate >= 400) {
                const fps = Math.round((frameCount * 1000) / (now - lastFpsUpdate));
                if (fpsElem) fpsElem.textContent = fps;
                frameCount = 0;
                lastFpsUpdate = now;
            }

            if (!isPaused && !isDragging) {
                rotY += 0.009 * rotSpeed;
                if (orbitMode === 1) {
                    rotX += 0.006 * rotSpeed;
                } else if (orbitMode === 2) {
                    rotX = 0.25 + Math.sin(now * 0.0015) * 0.35;
                } else {
                    rotX += (0.0025 + Math.sin(now * 0.001) * 0.0015) * rotSpeed;
                }
            }

            gl.enable(gl.DEPTH_TEST);
            gl.enable(gl.CULL_FACE);
            gl.clearColor(0.035, 0.045, 0.065, 1.0);
            gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

            gl.useProgram(program);

            const aspect = (canvas.width || 700) / (canvas.height || 480);
            window.WebForge3D.mat4.perspective(mProj, Math.PI / 4, aspect, 0.1, 100.0);
            const floatOffset = Math.sin(now * 0.0018) * 0.06;
            window.WebForge3D.mat4.lookAt(mView, [0, floatOffset, zoom], [0, 0, 0], [0, 1, 0]);

            // Model rotation
            window.WebForge3D.mat4.create();
            mModel.fill(0);
            mModel[0] = 1; mModel[5] = 1; mModel[10] = 1; mModel[15] = 1;
            window.WebForge3D.mat4.rotateX(mModel, mModel, rotX);
            window.WebForge3D.mat4.rotateY(mModel, mModel, rotY);
            window.WebForge3D.mat4.normalFromMat4(mNorm, mModel);

            gl.uniformMatrix4fv(uModel, false, mModel);
            gl.uniformMatrix4fv(uView, false, mView);
            gl.uniformMatrix4fv(uProj, false, mProj);
            gl.uniformMatrix3fv(uNormMat, false, mNorm);

            gl.uniform3f(uColor, currentRgb[0], currentRgb[1], currentRgb[2]);
            gl.uniform3f(uColor2, currentComp[0], currentComp[1], currentComp[2]);

            gl.uniform3f(uLight1, 5.0, 7.0, 6.0);
            gl.uniform3f(uLight2, -6.0, -4.0, -4.0);
            gl.uniform3f(uEye, 0.0, 0.0, zoom);
            gl.uniform1f(uMetal, 0.88);
            gl.uniform1f(uRough, 0.18);
            gl.uniform1i(uShading, shadingMode);
            gl.uniform1f(uTime, now * 0.001);

            gl.bindBuffer(gl.ARRAY_BUFFER, posBuf);
            gl.vertexAttribPointer(aPos, 3, gl.FLOAT, false, 0, 0);
            gl.enableVertexAttribArray(aPos);

            gl.bindBuffer(gl.ARRAY_BUFFER, normBuf);
            gl.vertexAttribPointer(aNorm, 3, gl.FLOAT, false, 0, 0);
            gl.enableVertexAttribArray(aNorm);

            const m = meshes[currentGeoKey];
            if (isWireframe) {
                gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, wireIdxBuf);
                gl.drawElements(gl.LINES, m.wireIndices.length, gl.UNSIGNED_SHORT, 0);
            } else {
                gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, idxBuf);
                gl.drawElements(gl.TRIANGLES, m.indices.length, gl.UNSIGNED_SHORT, 0);
            }
        }

        // Pointer Drag Interactions
        let isDragging = false;
        let prevX = 0, prevY = 0;
        canvas.addEventListener('pointerdown', (e) => {
            isDragging = true;
            prevX = e.clientX;
            prevY = e.clientY;
            canvas.style.cursor = 'grabbing';
            canvas.setPointerCapture(e.pointerId);
        });
        canvas.addEventListener('pointermove', (e) => {
            if (!isDragging) return;
            const dx = e.clientX - prevX;
            const dy = e.clientY - prevY;
            rotY += dx * 0.008;
            rotX += dy * 0.008;
            prevX = e.clientX;
            prevY = e.clientY;
        });
        const stopDrag = (e) => {
            if (isDragging) {
                isDragging = false;
                canvas.style.cursor = 'grab';
                try { canvas.releasePointerCapture(e.pointerId); } catch(err) {}
            }
        };
        canvas.addEventListener('pointerup', stopDrag);
        canvas.addEventListener('pointercancel', stopDrag);

        // Zoom Scroll
        canvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            zoom = Math.max(2.5, Math.min(8.5, zoom + e.deltaY * 0.003));
            if (zoomRangeElem) zoomRangeElem.value = Math.round(zoom * 10).toString();
            if (zoomNumElem) zoomNumElem.textContent = zoom.toFixed(1);
        }, { passive: false });

        // Copy Code Button
        if (copyBtnElem) {
            copyBtnElem.addEventListener('click', (e) => {
                e.stopPropagation();
                const code = getTxsSnippet();
                const markCopied = () => {
                    copyBtnElem.classList.add('copied');
                    copyBtnElem.innerHTML = '<span class="btn-copy-text">✓ Copied!</span>';
                    setTimeout(() => {
                        copyBtnElem.classList.remove('copied');
                        copyBtnElem.innerHTML = '<span class="btn-copy-text">📋 Copy Code</span>';
                    }, 2000);
                };
                if (navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard.writeText(code).then(markCopied).catch(markCopied);
                } else {
                    const ta = document.createElement('textarea');
                    ta.value = code;
                    document.body.appendChild(ta);
                    ta.select();
                    document.execCommand('copy');
                    document.body.removeChild(ta);
                    markCopied();
                }
            });
        }

        // Custom Color Picker native input
        if (colorPickerElem) {
            const onColorChange = (e) => {
                currentHex = (e.target.value || '#06B6D4').toUpperCase();
                currentRgb = hexToRgb(currentHex);
                currentComp = getHarmonicComplement(currentRgb);
                if (hexInputElem) hexInputElem.value = currentHex;
                if (swatchPreviewElem) swatchPreviewElem.style.background = currentHex;
                studioContainer.querySelectorAll('.color-swatch').forEach(s => s.classList.remove('active'));
                updateLiveCode();
            };
            colorPickerElem.addEventListener('input', onColorChange);
            colorPickerElem.addEventListener('change', onColorChange);
        }

        // Hex Text Input
        if (hexInputElem) {
            hexInputElem.addEventListener('input', (e) => {
                let val = e.target.value.trim();
                if (!val.startsWith('#')) val = '#' + val;
                if (/^#[0-9A-Fa-f]{6}$/.test(val)) {
                    currentHex = val.toUpperCase();
                    currentRgb = hexToRgb(currentHex);
                    currentComp = getHarmonicComplement(currentRgb);
                    if (colorPickerElem) colorPickerElem.value = currentHex;
                    if (swatchPreviewElem) swatchPreviewElem.style.background = currentHex;
                    studioContainer.querySelectorAll('.color-swatch').forEach(s => s.classList.remove('active'));
                    updateLiveCode();
                }
            });
        }

        // Preset Color Swatches
        studioContainer.querySelectorAll('.color-swatch').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                studioContainer.querySelectorAll('.color-swatch').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                const hex = (btn.getAttribute('data-hex') || '#06B6D4').toUpperCase();
                currentHex = hex;
                currentRgb = hexToRgb(currentHex);
                currentComp = getHarmonicComplement(currentRgb);
                if (colorPickerElem) colorPickerElem.value = currentHex;
                if (hexInputElem) hexInputElem.value = currentHex;
                if (swatchPreviewElem) swatchPreviewElem.style.background = currentHex;
                updateLiveCode();
            });
        });

        // Procedural 3D Model Buttons
        studioContainer.querySelectorAll('.scene-model-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                studioContainer.querySelectorAll('.scene-model-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                const g = btn.getAttribute('data-geo');
                if (meshes[g]) {
                    currentGeoKey = g;
                    loadMesh(meshes[g]);
                    updateMeshInfo();
                    updateLiveCode();
                }
            });
        });

        // GLSL Shading Style Pills
        studioContainer.querySelectorAll('.shading-pill').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                studioContainer.querySelectorAll('.shading-pill').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                shadingMode = parseInt(btn.getAttribute('data-shading') || '0', 10);
                updateLiveCode();
            });
        });

        // Sliders
        if (speedRangeElem) {
            speedRangeElem.addEventListener('input', (e) => {
                const val = parseInt(e.target.value, 10);
                rotSpeed = val / 10;
                if (speedNumElem) speedNumElem.textContent = rotSpeed.toFixed(1) + 'x';
                updateLiveCode();
            });
        }

        if (zoomRangeElem) {
            zoomRangeElem.addEventListener('input', (e) => {
                const val = parseInt(e.target.value, 10);
                zoom = val / 10;
                if (zoomNumElem) zoomNumElem.textContent = zoom.toFixed(1);
            });
        }

        // Topbar Controls
        studioContainer.querySelectorAll('[data-toggle="wireframe"]').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                isWireframe = !isWireframe;
                btn.classList.toggle('active', isWireframe);
                updateLiveCode();
            });
        });

        studioContainer.querySelectorAll('[data-toggle="pause"]').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                isPaused = !isPaused;
                btn.textContent = isPaused ? '▶ Play' : '⏸ Pause';
                btn.classList.toggle('active', isPaused);
            });
        });

        studioContainer.querySelectorAll('[data-toggle="reset"]').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                rotX = 0.25;
                rotY = 0.0;
                zoom = 5.4;
                rotSpeed = 1.2;
                orbitMode = 0;
                if (zoomRangeElem) zoomRangeElem.value = '54';
                if (zoomNumElem) zoomNumElem.textContent = '5.4';
                if (speedRangeElem) speedRangeElem.value = '12';
                if (speedNumElem) speedNumElem.textContent = '1.2x';
                updateLiveCode();
            });
        });

        // Copy Code Button
        const copyBtn = document.getElementById(id + '-copy-btn');
        if (copyBtn) {
            copyBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                const snippet = getTxsSnippet();
                const textSpan = copyBtn.querySelector('.btn-copy-text') || copyBtn;
                const orig = textSpan.textContent;
                function onCopied() {
                    copyBtn.classList.add('copied');
                    textSpan.textContent = '✓ Copied!';
                    setTimeout(() => {
                        copyBtn.classList.remove('copied');
                        textSpan.textContent = orig;
                    }, 1800);
                }
                if (navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard.writeText(snippet).then(onCopied).catch(() => {
                        const ta = document.createElement('textarea');
                        ta.value = snippet;
                        document.body.appendChild(ta);
                        ta.select();
                        document.execCommand('copy');
                        document.body.removeChild(ta);
                        onCopied();
                    });
                } else {
                    const ta = document.createElement('textarea');
                    ta.value = snippet;
                    document.body.appendChild(ta);
                    ta.select();
                    document.execCommand('copy');
                    document.body.removeChild(ta);
                    onCopied();
                }
            });
        }

        // Visibility checking on route changes
        function checkVisibility() {
            const pageView = container.closest('.webforge-page-view');
            const shown = !pageView || pageView.classList.contains('active-page');
            if (shown && !isVisible) {
                isVisible = true;
                resize();
                animId = requestAnimationFrame(renderFrame);
            } else if (!shown && isVisible) {
                isVisible = false;
                if (animId) cancelAnimationFrame(animId);
            }
        }

        // Start render loop immediately
        animId = requestAnimationFrame(renderFrame);

        return {
            checkVisibility,
            dispose() {
                isVisible = false;
                if (animId) cancelAnimationFrame(animId);
                window.removeEventListener('resize', resize);
            }
        };
    },

    // Canvas 2D Fallback for environments lacking WebGL support
    initFallback2D(canvas, container, id, opts) {
        const ctx = canvas.getContext('2d');
        let animId = null;
        let rot = 0;
        let isVisible = true;

        const statusElem = document.getElementById(id + '-status');
        if (statusElem) statusElem.textContent = 'Canvas 2D • Procedural Engine';

        function draw() {
            if (!isVisible) return;
            animId = requestAnimationFrame(draw);
            rot += 0.02;

            const w = canvas.width = container.clientWidth || 600;
            const h = canvas.height = container.clientHeight || 480;

            ctx.fillStyle = '#080B10';
            ctx.fillRect(0, 0, w, h);

            ctx.save();
            ctx.translate(w / 2, h / 2);

            // Draw glowing isometric geometry
            ctx.strokeStyle = '#06B6D4';
            ctx.lineWidth = 2;
            ctx.shadowColor = '#06B6D4';
            ctx.shadowBlur = 12;

            const r = 120;
            ctx.beginPath();
            for (let i = 0; i < 6; i++) {
                const a = rot + (i / 6) * Math.PI * 2;
                const x = Math.cos(a) * r;
                const y = Math.sin(a) * (r * 0.55);
                if (i === 0) ctx.moveTo(x, y);
                else ctx.lineTo(x, y);
            }
            ctx.closePath();
            ctx.stroke();

            // Inner core
            ctx.strokeStyle = '#14B8A6';
            ctx.beginPath();
            for (let i = 0; i < 6; i++) {
                const a = -rot * 1.5 + (i / 6) * Math.PI * 2;
                const x = Math.cos(a) * (r * 0.5);
                const y = Math.sin(a) * (r * 0.28);
                if (i === 0) ctx.moveTo(x, y);
                else ctx.lineTo(x, y);
            }
            ctx.closePath();
            ctx.stroke();

            ctx.restore();
        }

        animId = requestAnimationFrame(draw);

        return {
            checkVisibility() {},
            dispose() {
                isVisible = false;
                if (animId) cancelAnimationFrame(animId);
            }
        };
    }
};

function webforgeSelectTab(btn, panelId) {
    const parent = btn.closest('.webforge-tabs');
    if (!parent) return;
    parent.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    parent.querySelectorAll('.tab-panel').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    const target = parent.querySelector('#' + panelId);
    if (target) target.classList.add('active');
}

// ============================================================================
// WebForge Constructive Solid Geometry (CSG) Engine
// Real boolean union, subtraction, and intersection on Three.js BufferGeometry
// ============================================================================
const WebForgeCSG = {
    EPSILON: 1e-5,
    fromMesh(mesh) {
        mesh.updateMatrix();
        let geom = mesh.geometry;
        if (geom.index) geom = geom.toNonIndexed();
        const pos = geom.attributes.position.array;
        const norm = geom.attributes.normal ? geom.attributes.normal.array : null;
        const uvs = geom.attributes.uv ? geom.attributes.uv.array : null;
        const m = mesh.matrix;
        const polys = [];
        for (let i = 0; i < pos.length; i += 9) {
            const vertices = [];
            for (let j = 0; j < 3; j++) {
                const idx = i + j * 3;
                const v = new THREE.Vector3(pos[idx], pos[idx+1], pos[idx+2]).applyMatrix4(m);
                let n = norm ? new THREE.Vector3(norm[idx], norm[idx+1], norm[idx+2]).transformDirection(m) : new THREE.Vector3(0, 1, 0);
                const uv = uvs ? new THREE.Vector2(uvs[(i/3 + j)*2], uvs[(i/3 + j)*2 + 1]) : new THREE.Vector2(0, 0);
                vertices.push({ pos: v, normal: n, uv: uv });
            }
            const plane = WebForgeCSG.planeFromPoints(vertices[0].pos, vertices[1].pos, vertices[2].pos);
            if (plane) polys.push({ vertices, plane });
        }
        return new WebForgeCSG.Node(polys);
    },
    planeFromPoints(a, b, c) {
        const ab = new THREE.Vector3().subVectors(b, a);
        const ac = new THREE.Vector3().subVectors(c, a);
        const n = new THREE.Vector3().crossVectors(ab, ac).normalize();
        if (n.lengthSq() < 0.0001) return null;
        return { normal: n, w: n.dot(a) };
    },
    toMesh(node, material) {
        const polys = [];
        function collect(n) {
            if (!n) return;
            for (let p of n.polygons) polys.push(p);
            collect(n.front);
            collect(n.back);
        }
        collect(node);
        const positions = [];
        const normals = [];
        const uvs = [];
        for (let p of polys) {
            for (let i = 2; i < p.vertices.length; i++) {
                const v0 = p.vertices[0];
                const v1 = p.vertices[i-1];
                const v2 = p.vertices[i];
                positions.push(v0.pos.x, v0.pos.y, v0.pos.z);
                positions.push(v1.pos.x, v1.pos.y, v1.pos.z);
                positions.push(v2.pos.x, v2.pos.y, v2.pos.z);
                normals.push(v0.normal.x, v0.normal.y, v0.normal.z);
                normals.push(v1.normal.x, v1.normal.y, v1.normal.z);
                normals.push(v2.normal.x, v2.normal.y, v2.normal.z);
                uvs.push(v0.uv.x, v0.uv.y, v1.uv.x, v1.uv.y, v2.uv.x, v2.uv.y);
            }
        }
        const geom = new THREE.BufferGeometry();
        geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
        geom.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3));
        geom.setAttribute('uv', new THREE.Float32BufferAttribute(uvs, 2));
        geom.computeVertexNormals();
        return new THREE.Mesh(geom, material);
    },
    Node(polygons) {
        this.plane = null;
        this.front = null;
        this.back = null;
        this.polygons = [];
        if (polygons) this.build(polygons);
    }
};

WebForgeCSG.Node.prototype = {
    clone() {
        const node = new WebForgeCSG.Node();
        node.plane = this.plane ? { normal: this.plane.normal.clone(), w: this.plane.w } : null;
        node.front = this.front ? this.front.clone() : null;
        node.back = this.back ? this.back.clone() : null;
        node.polygons = this.polygons.map(p => ({
            plane: p.plane ? { normal: p.plane.normal.clone(), w: p.plane.w } : null,
            vertices: p.vertices.map(v => ({ pos: v.pos.clone(), normal: v.normal.clone(), uv: v.uv.clone() }))
        }));
        return node;
    },
    invert() {
        for (let p of this.polygons) {
            p.plane.normal.negate();
            p.plane.w = -p.plane.w;
            p.vertices.reverse();
            for (let v of p.vertices) v.normal.negate();
        }
        if (this.plane) {
            this.plane.normal.negate();
            this.plane.w = -this.plane.w;
        }
        if (this.front) this.front.invert();
        if (this.back) this.back.invert();
        const temp = this.front;
        this.front = this.back;
        this.back = temp;
    },
    clipPolygons(polygons) {
        if (!this.plane) return polygons.slice();
        let front = [], back = [];
        for (let p of polygons) {
            this.splitPolygon(p, front, back, front, back);
        }
        if (this.front) front = this.front.clipPolygons(front);
        if (this.back) back = this.back.clipPolygons(back);
        else back = [];
        return front.concat(back);
    },
    clipTo(bsp) {
        this.polygons = bsp.clipPolygons(this.polygons);
        if (this.front) this.front.clipTo(bsp);
        if (this.back) this.back.clipTo(bsp);
    },
    allPolygons() {
        let list = this.polygons.slice();
        if (this.front) list = list.concat(this.front.allPolygons());
        if (this.back) list = list.concat(this.back.allPolygons());
        return list;
    },
    build(polygons) {
        if (!polygons.length) return;
        if (!this.plane) {
            this.plane = { normal: polygons[0].plane.normal.clone(), w: polygons[0].plane.w };
        }
        const front = [], back = [];
        for (let p of polygons) {
            this.splitPolygon(p, this.polygons, this.polygons, front, back);
        }
        if (front.length) {
            if (!this.front) this.front = new WebForgeCSG.Node();
            this.front.build(front);
        }
        if (back.length) {
            if (!this.back) this.back = new WebForgeCSG.Node();
            this.back.build(back);
        }
    },
    splitPolygon(polygon, coplanarFront, coplanarBack, front, back) {
        const COPLANAR = 0, FRONT = 1, BACK = 2, SPANNING = 3;
        let polygonType = 0;
        const types = [];
        for (let v of polygon.vertices) {
            const t = this.plane.normal.dot(v.pos) - this.plane.w;
            const type = (t < -WebForgeCSG.EPSILON) ? BACK : (t > WebForgeCSG.EPSILON) ? FRONT : COPLANAR;
            polygonType |= type;
            types.push(type);
        }
        switch (polygonType) {
            case COPLANAR:
                (this.plane.normal.dot(polygon.plane.normal) > 0 ? coplanarFront : coplanarBack).push(polygon);
                break;
            case FRONT:
                front.push(polygon);
                break;
            case BACK:
                back.push(polygon);
                break;
            case SPANNING:
                const f = [], b = [];
                for (let i = 0; i < polygon.vertices.length; i++) {
                    const j = (i + 1) % polygon.vertices.length;
                    const ti = types[i], tj = types[j];
                    const vi = polygon.vertices[i], vj = polygon.vertices[j];
                    if (ti !== BACK) f.push(vi);
                    if (ti !== FRONT) b.push(vi);
                    if ((ti | tj) === SPANNING) {
                        const denom = this.plane.normal.dot(new THREE.Vector3().subVectors(vj.pos, vi.pos));
                        const t = Math.abs(denom) > 1e-6 ? (this.plane.w - this.plane.normal.dot(vi.pos)) / denom : 0.5;
                        const v = {
                            pos: new THREE.Vector3().lerpVectors(vi.pos, vj.pos, t),
                            normal: new THREE.Vector3().lerpVectors(vi.normal, vj.normal, t).normalize(),
                            uv: new THREE.Vector2().lerpVectors(vi.uv, vj.uv, t)
                        };
                        f.push(v);
                        b.push(v);
                    }
                }
                if (f.length >= 3) front.push({ vertices: f, plane: polygon.plane });
                if (b.length >= 3) back.push({ vertices: b, plane: polygon.plane });
                break;
        }
    }
};

WebForgeCSG.subtract = function(a, b, mat) {
    const aTree = WebForgeCSG.fromMesh(a);
    const bTree = WebForgeCSG.fromMesh(b);
    aTree.invert();
    aTree.clipTo(bTree);
    bTree.clipTo(aTree);
    bTree.invert();
    bTree.clipTo(aTree);
    bTree.invert();
    aTree.build(bTree.allPolygons());
    aTree.invert();
    return WebForgeCSG.toMesh(aTree, mat || a.material);
};

WebForgeCSG.union = function(a, b, mat) {
    const aTree = WebForgeCSG.fromMesh(a);
    const bTree = WebForgeCSG.fromMesh(b);
    aTree.clipTo(bTree);
    bTree.clipTo(aTree);
    bTree.invert();
    bTree.clipTo(aTree);
    bTree.invert();
    aTree.build(bTree.allPolygons());
    return WebForgeCSG.toMesh(aTree, mat || a.material);
};

WebForgeCSG.intersect = function(a, b, mat) {
    const aTree = WebForgeCSG.fromMesh(a);
    const bTree = WebForgeCSG.fromMesh(b);
    aTree.invert();
    bTree.clipTo(aTree);
    bTree.invert();
    aTree.clipTo(bTree);
    bTree.clipTo(aTree);
    aTree.build(bTree.allPolygons());
    aTree.invert();
    return WebForgeCSG.toMesh(aTree, mat || a.material);
};

// ============================================================================
// WebForge Custom 3D Creation Studio & Visual CAD Workbench
// ============================================================================
const WebForgeCustom3D = {
    scene: null,
    camera: null,
    renderer: null,
    controls: null,
    containerId: null,
    modelGroup: null,
    objects: [],
    selectedId: null,
    animPlaying: false,
    animSpeed: 1.0,
    animMode: 'turntable',
    animTime: 0,
    animLoop: true,
    lastFrameTime: 0,
    fps: 60,
    gridHelper: null,
    axesHelper: null,
    wireframe: false,
    flatShading: false,

    init(containerId) {
        if (typeof THREE === 'undefined') {
            setTimeout(() => this.init(containerId), 150);
            return;
        }

        this.containerId = containerId;
        const canvas = document.getElementById(containerId + '-viewport-canvas') || document.querySelector('.workbench-canvas');
        if (!canvas) return;
        const container = canvas.parentElement;

        const width = container.clientWidth || 800;
        const height = container.clientHeight || 500;

        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x07090e);

        this.camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
        this.camera.position.set(12, 14, 20);

        this.renderer = new THREE.WebGLRenderer({ canvas: canvas, antialias: true, alpha: true, preserveDrawingBuffer: true });
        this.renderer.setSize(width, height);
        this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
        this.renderer.shadowMap.enabled = true;
        this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;

        if (typeof THREE.OrbitControls !== 'undefined') {
            this.controls = new THREE.OrbitControls(this.camera, canvas);
            this.controls.enableDamping = true;
            this.controls.dampingFactor = 0.05;
            this.controls.target.set(0, 2, 0);
        }

        // Lighting
        const ambLight = new THREE.AmbientLight(0xffffff, 0.65);
        this.scene.add(ambLight);

        const dirLight = new THREE.DirectionalLight(0xffffff, 1.2);
        dirLight.position.set(15, 25, 18);
        dirLight.castShadow = true;
        dirLight.shadow.mapSize.width = 1024;
        dirLight.shadow.mapSize.height = 1024;
        this.scene.add(dirLight);

        const fillLight = new THREE.DirectionalLight(0x06b6d4, 0.45);
        fillLight.position.set(-15, 10, -15);
        this.scene.add(fillLight);

        // Ground Grid & Axes
        this.gridHelper = new THREE.GridHelper(24, 24, 0x06b6d4, 0x1e293b);
        this.gridHelper.position.y = -0.01;
        this.scene.add(this.gridHelper);

        this.axesHelper = new THREE.AxesHelper(3);
        this.scene.add(this.axesHelper);

        this.modelGroup = new THREE.Group();
        this.scene.add(this.modelGroup);

        // Viewport click selection via raycasting
        const raycaster = new THREE.Raycaster();
        const mouse = new THREE.Vector2();
        canvas.addEventListener('pointerdown', (e) => {
            const rect = canvas.getBoundingClientRect();
            mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
            mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;
            raycaster.setFromCamera(mouse, this.camera);
            const intersects = raycaster.intersectObjects(this.modelGroup.children, true);
            if (intersects.length > 0) {
                let hit = intersects[0].object;
                while (hit && hit.parent && hit.parent !== this.modelGroup) {
                    hit = hit.parent;
                }
                if (hit && hit.userData && hit.userData.id) {
                    this.selectObject(hit.userData.id);
                }
            }
        });

        // Window resize
        window.addEventListener('resize', () => {
            if (!container || !this.renderer || !this.camera) return;
            const w = container.clientWidth;
            const h = container.clientHeight;
            this.camera.aspect = w / h;
            this.camera.updateProjectionMatrix();
            this.renderer.setSize(w, h);
        });

        // Load Default Starter Project: Modular House
        this.loadTemplate('house');

        // Start Animation Loop
        this.lastFrameTime = performance.now();
        const animate = (time) => {
            requestAnimationFrame(animate);

            const dt = (time - this.lastFrameTime) / 1000;
            this.lastFrameTime = time;
            if (dt > 0) {
                this.fps = Math.round(0.9 * this.fps + 0.1 * (1 / dt));
                const fpsEl = document.getElementById('vp-fps');
                if (fpsEl) fpsEl.innerText = this.fps;
            }

            if (this.controls) this.controls.update();

            // Timeline Motion
            if (this.animPlaying) {
                this.animTime += dt * this.animSpeed;
                if (this.animTime > 5.0) {
                    if (this.animLoop) {
                        this.animTime = 0;
                    } else {
                        this.animTime = 5.0;
                        this.animPlaying = false;
                        const pb = document.getElementById('tl-play-pause-btn');
                        if (pb) pb.innerText = '▶ Play';
                    }
                }
                this.applyAnimationAtTime(this.animTime);

                const scrubber = document.getElementById('tl-scrubber');
                if (scrubber) scrubber.value = (this.animTime / 5.0) * 100;
                const tc = document.getElementById('tl-timecode');
                if (tc) tc.innerText = '00:0' + this.animTime.toFixed(1);
            }

            this.renderer.render(this.scene, this.camera);
        };
        requestAnimationFrame(animate);
    },

    createMaterial(preset, color) {
        const hex = color || '#06b6d4';
        switch (preset) {
            case 'metal':
                return new THREE.MeshStandardMaterial({ color: hex, roughness: 0.2, metalness: 0.85, wireframe: this.wireframe, flatShading: this.flatShading });
            case 'stone':
                return new THREE.MeshStandardMaterial({ color: hex, roughness: 0.85, metalness: 0.08, wireframe: this.wireframe, flatShading: this.flatShading });
            case 'glass':
                return new THREE.MeshStandardMaterial({ color: hex, roughness: 0.08, metalness: 0.1, transparent: true, opacity: 0.55, wireframe: this.wireframe });
            case 'wood':
                return new THREE.MeshStandardMaterial({ color: hex, roughness: 0.7, metalness: 0.05, wireframe: this.wireframe, flatShading: this.flatShading });
            case 'plastic':
                return new THREE.MeshStandardMaterial({ color: hex, roughness: 0.45, metalness: 0.15, wireframe: this.wireframe, flatShading: this.flatShading });
            case 'glow':
                return new THREE.MeshStandardMaterial({ color: hex, emissive: hex, emissiveIntensity: 0.65, roughness: 0.3, metalness: 0.4, wireframe: this.wireframe });
            default:
                return new THREE.MeshStandardMaterial({ color: hex, roughness: 0.3, metalness: 0.5, wireframe: this.wireframe, flatShading: this.flatShading });
        }
    },

    createRoofGeometry(w, h, d) {
        // Gabled pitched roof prism
        const geom = new THREE.BufferGeometry();
        const hw = w / 2, hd = d / 2;
        const vertices = [
            // Front triangle
            -hw, 0, hd,   hw, 0, hd,   0, h, hd,
            // Back triangle
            hw, 0, -hd,  -hw, 0, -hd,  0, h, -hd,
            // Right slope
            hw, 0, hd,   hw, 0, -hd,  0, h, -hd,
            hw, 0, hd,   0, h, -hd,   0, h, hd,
            // Left slope
            -hw, 0, -hd, -hw, 0, hd,   0, h, hd,
            -hw, 0, -hd, 0, h, hd,    0, h, -hd,
            // Bottom base
            -hw, 0, -hd, hw, 0, -hd,  hw, 0, hd,
            -hw, 0, -hd, hw, 0, hd,   -hw, 0, hd
        ];
        geom.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
        geom.computeVertexNormals();
        return geom;
    },

    loadTemplate(type) {
        document.querySelectorAll('.template-btn').forEach(b => {
            b.classList.toggle('active', b.getAttribute('data-template') === type);
        });

        // Clear scene
        while (this.modelGroup.children.length > 0) {
            this.modelGroup.remove(this.modelGroup.children[0]);
        }
        this.objects = [];
        this.selectedId = null;

        if (type === 'house') {
            this.addPart({ id: 'Foundation', name: 'Foundation', type: 'box', pos: [0, 0.2, 0], rot: [0, 0, 0], scale: [12, 0.4, 10], color: '#334155', material: 'stone', roughness: 0.85, metalness: 0.1 });
            this.addPart({ id: 'FrontWall', name: 'FrontWall', type: 'wall', pos: [0, 2.2, 4.85], rot: [0, 0, 0], scale: [12, 3.6, 0.3], color: '#f1f5f9', material: 'stone', roughness: 0.8, metalness: 0.05 });
            this.addPart({ id: 'BackWall', name: 'BackWall', type: 'wall', pos: [0, 2.2, -4.85], rot: [0, 0, 0], scale: [12, 3.6, 0.3], color: '#f1f5f9', material: 'stone', roughness: 0.8, metalness: 0.05 });
            this.addPart({ id: 'LeftWall', name: 'LeftWall', type: 'wall', pos: [-5.85, 2.2, 0], rot: [0, 0, 0], scale: [0.3, 3.6, 9.4], color: '#e2e8f0', material: 'stone', roughness: 0.8, metalness: 0.05 });
            this.addPart({ id: 'RightWall', name: 'RightWall', type: 'wall', pos: [5.85, 2.2, 0], rot: [0, 0, 0], scale: [0.3, 3.6, 9.4], color: '#e2e8f0', material: 'stone', roughness: 0.8, metalness: 0.05 });
            this.addPart({ id: 'GableRoof', name: 'GableRoof', type: 'roof', pos: [0, 4.0, 0], rot: [0, 0, 0], scale: [13, 2.8, 11], color: '#b91c1c', material: 'wood', roughness: 0.65, metalness: 0.05 });
            this.addPart({ id: 'MainDoor', name: 'MainDoor', type: 'door', pos: [0, 1.3, 5.02], rot: [0, 0, 0], scale: [1.6, 2.2, 0.15], color: '#78350f', material: 'wood', roughness: 0.5, metalness: 0.05 });
            this.addPart({ id: 'WindowLeft', name: 'WindowLeft', type: 'window', pos: [-3.2, 2.3, 5.02], rot: [0, 0, 0], scale: [1.8, 1.4, 0.12], color: '#38bdf8', material: 'glass', roughness: 0.1, metalness: 0.1 });
            this.addPart({ id: 'WindowRight', name: 'WindowRight', type: 'window', pos: [3.2, 2.3, 5.02], rot: [0, 0, 0], scale: [1.8, 1.4, 0.12], color: '#38bdf8', material: 'glass', roughness: 0.1, metalness: 0.1 });
            this.addPart({ id: 'Chimney', name: 'Chimney', type: 'chimney', pos: [3.8, 5.2, -2.2], rot: [0, 0, 0], scale: [1.0, 3.0, 1.0], color: '#991b1b', material: 'stone', roughness: 0.9, metalness: 0.05 });
        } else if (type === 'logo') {
            // Extruded & Beveled Custom Logo
            const shape = new THREE.Shape();
            shape.moveTo(0, 3.2);
            shape.lineTo(2.8, 0);
            shape.lineTo(0, -3.2);
            shape.lineTo(-2.8, 0);
            shape.closePath();

            const hole = new THREE.Path();
            hole.moveTo(0, 1.4);
            hole.lineTo(-1.2, 0);
            hole.lineTo(0, -1.4);
            hole.lineTo(1.2, 0);
            hole.closePath();
            shape.holes.push(hole);

            const extrudeSettings = { depth: 0.45, bevelEnabled: true, bevelThickness: 0.06, bevelSize: 0.05, bevelSegments: 3 };
            const geom = new THREE.ExtrudeGeometry(shape, extrudeSettings);
            geom.center();

            const mat = this.createMaterial('metal', '#9AEA39');
            const mesh = new THREE.Mesh(geom, mat);
            mesh.position.set(0, 3.5, 0);
            mesh.castShadow = true;
            mesh.receiveShadow = true;
            mesh.userData = { id: 'TechScriptLogo', name: 'TechScriptLogo', type: 'extrude', color: '#9AEA39', material: 'metal', depth: 0.45, bevelThick: 0.06 };
            this.modelGroup.add(mesh);
            this.objects.push(mesh.userData);

            // Add Pedestal
            this.addPart({ id: 'LogoPedestal', name: 'LogoPedestal', type: 'cylinder', pos: [0, 0.4, 0], rot: [0, 0, 0], scale: [4.5, 0.8, 4.5], color: '#0f172a', material: 'metal', roughness: 0.3, metalness: 0.8 });
        } else if (type === 'mech') {
            this.addPart({ id: 'Torso', name: 'Torso', type: 'box', pos: [0, 3.5, 0], rot: [0, 0, 0], scale: [2.6, 3.2, 1.8], color: '#0284c7', material: 'metal', roughness: 0.25, metalness: 0.85 });
            this.addPart({ id: 'Head', name: 'Head', type: 'box', pos: [0, 5.6, 0], rot: [0, 0, 0], scale: [1.6, 1.2, 1.5], color: '#0f172a', material: 'metal', roughness: 0.2, metalness: 0.9 });
            this.addPart({ id: 'Visor', name: 'Visor', type: 'box', pos: [0, 5.6, 0.8], rot: [0, 0, 0], scale: [1.4, 0.35, 0.2], color: '#06b6d4', material: 'glow', roughness: 0.1, metalness: 0.5 });
            this.addPart({ id: 'LeftArm', name: 'LeftArm', type: 'cylinder', pos: [-2.1, 3.5, 0], rot: [0, 0, 0], scale: [0.45, 3.2, 0.45], color: '#f59e0b', material: 'metal', roughness: 0.3, metalness: 0.8 });
            this.addPart({ id: 'RightArm', name: 'RightArm', type: 'cylinder', pos: [2.1, 3.5, 0], rot: [0, 0, 0], scale: [0.45, 3.2, 0.45], color: '#f59e0b', material: 'metal', roughness: 0.3, metalness: 0.8 });
            this.addPart({ id: 'Thruster', name: 'Thruster', type: 'cone', pos: [0, 1.0, 0], rot: [Math.PI, 0, 0], scale: [1.6, 2.0, 1.6], color: '#334155', material: 'metal', roughness: 0.4, metalness: 0.8 });
        } else if (type === 'csg') {
            // Machined cube with cylindrical bore cut through it using CSG
            const baseMesh = new THREE.Mesh(new THREE.BoxGeometry(4.2, 4.2, 4.2), this.createMaterial('metal', '#06b6d4'));
            baseMesh.position.set(0, 2.1, 0);
            const cutterMesh = new THREE.Mesh(new THREE.CylinderGeometry(1.4, 1.4, 6.0, 32), this.createMaterial('metal', '#f43f5e'));
            cutterMesh.position.set(0, 2.1, 0);

            const resultMesh = WebForgeCSG.subtract(baseMesh, cutterMesh, this.createMaterial('metal', '#06b6d4'));
            resultMesh.position.set(0, 0, 0);
            resultMesh.castShadow = true;
            resultMesh.receiveShadow = true;
            resultMesh.userData = { id: 'BoredCylinderPart', name: 'BoredCylinderPart', type: 'csg_subtract', color: '#06b6d4', material: 'metal', roughness: 0.25, metalness: 0.85 };
            this.modelGroup.add(resultMesh);
            this.objects.push(resultMesh.userData);

            webforgeToast('CSG Boolean Subtraction: Cylindrical bore cut through metal chassis');
        }

        if (this.objects.length > 0) {
            this.selectObject(this.objects[0].id);
        } else {
            this.updateHierarchyUI();
            this.updateStats();
            this.generateTxsFromScene();
        }
    },

    addPart(config) {
        let geom;
        const [w, h, d] = config.scale || [1, 1, 1];
        switch (config.type) {
            case 'box':
            case 'wall':
            case 'door':
            case 'window':
            case 'chimney':
                geom = new THREE.BoxGeometry(w, h, d);
                break;
            case 'cylinder':
                geom = new THREE.CylinderGeometry(w / 2, w / 2, h, 32);
                break;
            case 'sphere':
                geom = new THREE.SphereGeometry(w / 2, 32, 24);
                break;
            case 'cone':
                geom = new THREE.ConeGeometry(w / 2, h, 32);
                break;
            case 'roof':
                geom = this.createRoofGeometry(w, h, d);
                break;
            default:
                geom = new THREE.BoxGeometry(w, h, d);
        }

        const mat = this.createMaterial(config.material || 'stone', config.color || '#06b6d4');
        const mesh = new THREE.Mesh(geom, mat);
        if (config.pos) mesh.position.set(...config.pos);
        if (config.rot) mesh.rotation.set(...config.rot);
        mesh.castShadow = true;
        mesh.receiveShadow = true;

        mesh.userData = {
            id: config.id || ('Part_' + Date.now().toString(36)),
            name: config.name || config.id,
            type: config.type || 'box',
            color: config.color || '#06b6d4',
            material: config.material || 'stone',
            roughness: config.roughness !== undefined ? config.roughness : 0.4,
            metalness: config.metalness !== undefined ? config.metalness : 0.5,
            scaleW: w,
            scaleH: h,
            scaleD: d
        };

        this.modelGroup.add(mesh);
        this.objects.push(mesh.userData);
        this.updateHierarchyUI();
        this.updateStats();
        this.generateTxsFromScene();
        return mesh;
    },

    addPrimitive(type) {
        const id = type.charAt(0).toUpperCase() + type.slice(1) + '_' + (this.objects.length + 1);
        const yPos = type === 'wall' || type === 'box' ? 2 : 1.5;
        this.addPart({
            id: id,
            name: id,
            type: type,
            pos: [0, yPos, 0],
            rot: [0, 0, 0],
            scale: type === 'wall' ? [6, 3, 0.3] : (type === 'roof' ? [6, 2, 4] : [2, 2, 2]),
            color: type === 'window' ? '#38bdf8' : (type === 'door' ? '#78350f' : '#06b6d4'),
            material: type === 'window' ? 'glass' : (type === 'door' ? 'wood' : 'metal'),
            roughness: 0.3,
            metalness: 0.6
        });
        this.selectObject(id);
        webforgeToast('Added new ' + type + ': ' + id);
    },

    applyCSG(operation) {
        if (this.modelGroup.children.length < 2) {
            webforgeToast('CSG Boolean requires at least 2 objects in the scene.');
            return;
        }

        let cutter = this.getSelectedMesh();
        let base = null;
        for (let child of this.modelGroup.children) {
            if (child !== cutter) {
                base = child;
                break;
            }
        }

        if (!base || !cutter) {
            webforgeToast('Select the cutter object first.');
            return;
        }

        try {
            let resultMesh;
            if (operation === 'subtract') {
                resultMesh = WebForgeCSG.subtract(base, cutter);
            } else if (operation === 'union') {
                resultMesh = WebForgeCSG.union(base, cutter);
            } else if (operation === 'intersect') {
                resultMesh = WebForgeCSG.intersect(base, cutter);
            }

            resultMesh.castShadow = true;
            resultMesh.receiveShadow = true;
            const newId = base.userData.id + '_csg';
            resultMesh.userData = {
                id: newId,
                name: newId,
                type: 'csg_' + operation,
                color: base.userData.color,
                material: base.userData.material,
                roughness: base.userData.roughness,
                metalness: base.userData.metalness
            };

            // Remove original meshes
            this.modelGroup.remove(base);
            this.modelGroup.remove(cutter);
            this.objects = this.objects.filter(o => o.id !== base.userData.id && o.id !== cutter.userData.id);

            this.modelGroup.add(resultMesh);
            this.objects.push(resultMesh.userData);

            this.selectObject(newId);
            webforgeToast('CSG ' + operation.toUpperCase() + ' modified geometry successfully!');
        } catch (err) {
            console.error('CSG Operation Error:', err);
            webforgeToast('CSG operation warning: non-manifold intersection.');
        }
    },

    handleSvgUpload(e) {
        const file = e.target.files[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (event) => {
            const svgText = event.target.result;
            try {
                let shapes = [];
                if (typeof THREE.SVGLoader !== 'undefined') {
                    const loader = new THREE.SVGLoader();
                    const svgData = loader.parse(svgText);
                    for (let path of svgData.paths) {
                        const pathShapes = THREE.SVGLoader.createShapes(path);
                        shapes = shapes.concat(pathShapes);
                    }
                } else {
                    // Fallback diamond outline with inner hole
                    const s = new THREE.Shape();
                    s.moveTo(0, 3); s.lineTo(2.5, 0); s.lineTo(0, -3); s.lineTo(-2.5, 0); s.closePath();
                    const h = new THREE.Path();
                    h.moveTo(0, 1.2); h.lineTo(-1, 0); h.lineTo(0, -1.2); h.lineTo(1, 0); h.closePath();
                    s.holes.push(h);
                    shapes.push(s);
                }

                if (shapes.length > 0) {
                    const extrudeSettings = { depth: 0.5, bevelEnabled: true, bevelThickness: 0.06, bevelSize: 0.05, bevelSegments: 3 };
                    const geom = new THREE.ExtrudeGeometry(shapes, extrudeSettings);
                    geom.center();

                    const id = 'SvgModel_' + Date.now().toString(36);
                    const mat = this.createMaterial('metal', '#9AEA39');
                    const mesh = new THREE.Mesh(geom, mat);
                    mesh.position.set(0, 3, 0);
                    mesh.castShadow = true;
                    mesh.receiveShadow = true;
                    mesh.userData = { id: id, name: id, type: 'extrude', color: '#9AEA39', material: 'metal', depth: 0.5, bevelThick: 0.06 };

                    this.modelGroup.add(mesh);
                    this.objects.push(mesh.userData);
                    this.selectObject(id);
                    webforgeToast('Imported and extruded SVG outline successfully!');
                }
            } catch (err) {
                console.error(err);
                webforgeToast('Failed to parse SVG file.');
            }
        };
        reader.readAsText(file);
    },

    handleTxsUpload(e) {
        const file = e.target.files[0];
        if (!file) return;
        const reader = new FileReader();
        reader.onload = (event) => {
            const code = event.target.result;
            const editor = document.getElementById('live-txs-editor');
            if (editor) editor.value = code;
            this.compileFromEditor();
            webforgeToast('Loaded TechScript source successfully!');
        };
        reader.readAsText(file);
    },

    getSelectedMesh() {
        if (!this.selectedId) return null;
        return this.modelGroup.children.find(c => c.userData && c.userData.id === this.selectedId);
    },

    selectObject(id) {
        this.selectedId = id;
        this.updateHierarchyUI();

        const mesh = this.getSelectedMesh();
        const noSel = document.getElementById('inspector-no-selection');
        const cont = document.getElementById('inspector-content');

        if (!mesh) {
            if (noSel) noSel.style.display = 'block';
            if (cont) cont.style.display = 'none';
            return;
        }

        if (noSel) noSel.style.display = 'none';
        if (cont) cont.style.display = 'block';

        const nameInput = document.getElementById('prop-obj-name');
        if (nameInput) nameInput.value = mesh.userData.name || id;

        const typeBadge = document.getElementById('prop-obj-type');
        if (typeBadge) typeBadge.innerText = mesh.userData.type || 'box';

        const px = document.getElementById('prop-pos-x'); if (px) px.value = mesh.position.x.toFixed(2);
        const py = document.getElementById('prop-pos-y'); if (py) py.value = mesh.position.y.toFixed(2);
        const pz = document.getElementById('prop-pos-z'); if (pz) pz.value = mesh.position.z.toFixed(2);

        const rx = document.getElementById('prop-rot-x'); if (rx) rx.value = Math.round(THREE.MathUtils.radToDeg(mesh.rotation.x));
        const ry = document.getElementById('prop-rot-y'); if (ry) ry.value = Math.round(THREE.MathUtils.radToDeg(mesh.rotation.y));
        const rz = document.getElementById('prop-rot-z'); if (rz) rz.value = Math.round(THREE.MathUtils.radToDeg(mesh.rotation.z));

        const sx = document.getElementById('prop-scl-x'); if (sx) sx.value = (mesh.userData.scaleW || mesh.scale.x).toFixed(2);
        const sy = document.getElementById('prop-scl-y'); if (sy) sy.value = (mesh.userData.scaleH || mesh.scale.y).toFixed(2);
        const sz = document.getElementById('prop-scl-z'); if (sz) sz.value = (mesh.userData.scaleD || mesh.scale.z).toFixed(2);

        const matPreset = document.getElementById('prop-mat-preset');
        if (matPreset) matPreset.value = mesh.userData.material || 'metal';

        const colHex = mesh.userData.color || '#06b6d4';
        const colPick = document.getElementById('prop-mat-color-picker'); if (colPick) colPick.value = colHex;
        const colText = document.getElementById('prop-mat-color-hex'); if (colText) colText.value = colHex;

        const roughInput = document.getElementById('prop-mat-roughness');
        if (roughInput && mesh.material) {
            roughInput.value = mesh.material.roughness !== undefined ? mesh.material.roughness : 0.3;
            const rVal = document.getElementById('prop-rough-val');
            if (rVal) rVal.innerText = Number(roughInput.value).toFixed(2);
        }

        const metalInput = document.getElementById('prop-mat-metalness');
        if (metalInput && mesh.material) {
            metalInput.value = mesh.material.metalness !== undefined ? mesh.material.metalness : 0.5;
            const mVal = document.getElementById('prop-metal-val');
            if (mVal) mVal.innerText = Number(metalInput.value).toFixed(2);
        }

        const extrudeGroup = document.getElementById('extrude-bevel-group');
        if (extrudeGroup) {
            extrudeGroup.style.display = mesh.userData.type === 'extrude' ? 'block' : 'none';
        }
    },

    updateHierarchyUI() {
        const tree = document.getElementById('scene-hierarchy-tree');
        if (!tree) return;
        tree.innerHTML = '';

        const countEl = document.getElementById('scene-part-count');
        if (countEl) countEl.innerText = this.objects.length;

        for (let obj of this.objects) {
            const node = document.createElement('div');
            node.className = 'tree-node' + (obj.id === this.selectedId ? ' active' : '');
            node.onclick = () => this.selectObject(obj.id);

            const icon = obj.type === 'roof' ? '⛺' : (obj.type === 'wall' ? '🧱' : (obj.type === 'door' ? '🚪' : (obj.type === 'window' ? '🪟' : (obj.type === 'cylinder' ? '🥫' : (obj.type === 'sphere' ? '🌐' : (obj.type === 'extrude' ? '⚡' : '📦'))))));

            node.innerHTML = `
                <div class="tree-node-title">
                    <span>${icon}</span>
                    <span>${escapeHtml(obj.name || obj.id)}</span>
                </div>
                <div class="tree-node-actions">
                    <button type="button" class="tree-mini-btn" title="Duplicate Part" onclick="event.stopPropagation();WebForgeCustom3D.duplicateObject('${obj.id}')">📑</button>
                    <button type="button" class="tree-mini-btn" title="Delete Part" onclick="event.stopPropagation();WebForgeCustom3D.deleteObject('${obj.id}')">🗑️</button>
                </div>
            `;
            tree.appendChild(node);
        }
    },

    updateStats() {
        let totalVerts = 0;
        let totalPolys = 0;
        for (let child of this.modelGroup.children) {
            if (child.geometry) {
                const pos = child.geometry.attributes.position;
                if (pos) {
                    const verts = pos.count;
                    totalVerts += verts;
                    totalPolys += child.geometry.index ? child.geometry.index.count / 3 : verts / 3;
                }
            }
        }
        const polysEl = document.getElementById('vp-polys');
        if (polysEl) polysEl.innerText = Math.round(totalPolys).toLocaleString();
        const vertsEl = document.getElementById('vp-verts');
        if (vertsEl) vertsEl.innerText = totalVerts.toLocaleString();
    },

    updateTransform(prop, val) {
        const mesh = this.getSelectedMesh();
        if (!mesh) return;
        const num = parseFloat(val) || 0;
        if (prop === 'posX') mesh.position.x = num;
        else if (prop === 'posY') mesh.position.y = num;
        else if (prop === 'posZ') mesh.position.z = num;
        else if (prop === 'rotX') mesh.rotation.x = THREE.MathUtils.degToRad(num);
        else if (prop === 'rotY') mesh.rotation.y = THREE.MathUtils.degToRad(num);
        else if (prop === 'rotZ') mesh.rotation.z = THREE.MathUtils.degToRad(num);
        else if (prop === 'sclX') { mesh.scale.x = Math.max(0.01, num); mesh.userData.scaleW = num; }
        else if (prop === 'sclY') { mesh.scale.y = Math.max(0.01, num); mesh.userData.scaleH = num; }
        else if (prop === 'sclZ') { mesh.scale.z = Math.max(0.01, num); mesh.userData.scaleD = num; }

        this.generateTxsFromScene();
    },

    updateMaterialPreset(preset) {
        const mesh = this.getSelectedMesh();
        if (!mesh) return;
        mesh.userData.material = preset;
        mesh.material = this.createMaterial(preset, mesh.userData.color);
        this.generateTxsFromScene();
    },

    updateColor(color) {
        const mesh = this.getSelectedMesh();
        if (!mesh) return;
        mesh.userData.color = color;
        if (mesh.material) {
            mesh.material.color.set(color);
            if (mesh.material.emissive) mesh.material.emissive.set(color);
        }
        const colPick = document.getElementById('prop-mat-color-picker'); if (colPick) colPick.value = color;
        const colText = document.getElementById('prop-mat-color-hex'); if (colText) colText.value = color;
        this.generateTxsFromScene();
    },

    updateMaterialProp(prop, val) {
        const mesh = this.getSelectedMesh();
        if (!mesh || !mesh.material) return;
        const num = parseFloat(val) || 0;
        if (prop === 'roughness') {
            mesh.material.roughness = num;
            mesh.userData.roughness = num;
            const rVal = document.getElementById('prop-rough-val'); if (rVal) rVal.innerText = num.toFixed(2);
        } else if (prop === 'metalness') {
            mesh.material.metalness = num;
            mesh.userData.metalness = num;
            const mVal = document.getElementById('prop-metal-val'); if (mVal) mVal.innerText = num.toFixed(2);
        }
    },

    updateExtrude(prop, val) {
        const mesh = this.getSelectedMesh();
        if (!mesh || mesh.userData.type !== 'extrude') return;
        const num = parseFloat(val) || 0;
        if (prop === 'depth') {
            mesh.userData.depth = num;
            const dVal = document.getElementById('prop-extrude-val'); if (dVal) dVal.innerText = num.toFixed(2);
        } else if (prop === 'bevelThick') {
            mesh.userData.bevelThick = num;
            const bVal = document.getElementById('prop-bevel-val'); if (bVal) bVal.innerText = num.toFixed(2);
        }
        // Re-extrude logo shape
        const shape = new THREE.Shape();
        shape.moveTo(0, 3.2); shape.lineTo(2.8, 0); shape.lineTo(0, -3.2); shape.lineTo(-2.8, 0); shape.closePath();
        const hole = new THREE.Path();
        hole.moveTo(0, 1.4); hole.lineTo(-1.2, 0); hole.lineTo(0, -1.4); hole.lineTo(1.2, 0); hole.closePath();
        shape.holes.push(hole);

        mesh.geometry.dispose();
        mesh.geometry = new THREE.ExtrudeGeometry(shape, { depth: mesh.userData.depth || 0.45, bevelEnabled: true, bevelThickness: mesh.userData.bevelThick || 0.05, bevelSize: 0.04, bevelSegments: 3 });
        mesh.geometry.center();
        this.generateTxsFromScene();
    },

    updateSelectedName(newName) {
        const mesh = this.getSelectedMesh();
        if (!mesh) return;
        mesh.userData.name = newName;
        const item = this.objects.find(o => o.id === mesh.userData.id);
        if (item) item.name = newName;
        this.updateHierarchyUI();
        this.generateTxsFromScene();
    },

    duplicateObject(id) {
        const original = this.modelGroup.children.find(c => c.userData && c.userData.id === id);
        if (!original) return;
        const newId = original.userData.type + '_' + Date.now().toString(36);
        this.addPart({
            id: newId,
            name: newId,
            type: original.userData.type,
            pos: [original.position.x + 1.2, original.position.y, original.position.z + 1.2],
            rot: [original.rotation.x, original.rotation.y, original.rotation.z],
            scale: [original.userData.scaleW || original.scale.x, original.userData.scaleH || original.scale.y, original.userData.scaleD || original.scale.z],
            color: original.userData.color,
            material: original.userData.material,
            roughness: original.userData.roughness,
            metalness: original.userData.metalness
        });
        this.selectObject(newId);
        webforgeToast('Duplicated ' + id);
    },

    deleteObject(id) {
        const mesh = this.modelGroup.children.find(c => c.userData && c.userData.id === id);
        if (mesh) {
            this.modelGroup.remove(mesh);
            if (mesh.geometry) mesh.geometry.dispose();
        }
        this.objects = this.objects.filter(o => o.id !== id);
        if (this.selectedId === id) {
            this.selectedId = this.objects.length > 0 ? this.objects[0].id : null;
        }
        this.selectObject(this.selectedId);
        this.updateStats();
        this.generateTxsFromScene();
        webforgeToast('Deleted object.');
    },

    switchRightTab(tab) {
        const btnInsp = document.getElementById('tab-btn-inspector');
        const btnCode = document.getElementById('tab-btn-code');
        const tabInsp = document.getElementById('right-tab-inspector');
        const tabCode = document.getElementById('right-tab-code');
        if (tab === 'inspector') {
            if (btnInsp) btnInsp.classList.add('active');
            if (btnCode) btnCode.classList.remove('active');
            if (tabInsp) tabInsp.classList.add('active');
            if (tabCode) tabCode.classList.remove('active');
        } else {
            if (btnInsp) btnInsp.classList.remove('active');
            if (btnCode) btnCode.classList.add('active');
            if (tabInsp) tabInsp.classList.remove('active');
            if (tabCode) tabCode.classList.add('active');
            this.generateTxsFromScene();
        }
    },

    generateTxsFromScene() {
        let lines = ['make 3D "Custom3DModel":'];
        for (let obj of this.objects) {
            const mesh = this.modelGroup.children.find(c => c.userData && c.userData.id === obj.id);
            if (!mesh) continue;
            const px = mesh.position.x.toFixed(1);
            const py = mesh.position.y.toFixed(1);
            const pz = mesh.position.z.toFixed(1);
            const sx = (obj.scaleW || mesh.scale.x).toFixed(1);
            const sy = (obj.scaleH || mesh.scale.y).toFixed(1);
            const sz = (obj.scaleD || mesh.scale.z).toFixed(1);

            lines.push(`    add ${obj.type} "${obj.name || obj.id}" at (${px}, ${py}, ${pz}) size (${sx}, ${sy}, ${sz}) color "${obj.color}" material "${obj.material}"`);
        }
        lines.push('end');
        const code = lines.join('\n');

        const editor = document.getElementById('live-txs-editor');
        if (editor && document.activeElement !== editor) {
            editor.value = code;
        }
        const statusMsg = document.getElementById('code-status-msg');
        if (statusMsg) statusMsg.innerText = 'TechScript In Sync (' + this.objects.length + ' parts)';
        return code;
    },

    compileFromEditor() {
        const editor = document.getElementById('live-txs-editor');
        if (!editor) return;
        const code = editor.value;

        try {
            const lines = code.split('\n');
            const newParts = [];

            for (let line of lines) {
                const trimmed = line.trim();
                if (trimmed.startsWith('add ')) {
                    const rest = trimmed.substring(4);
                    const parts = rest.split(' ');
                    const type = parts[0] || 'box';
                    const nameMatch = rest.match(/"([^"]+)"/);
                    const name = nameMatch ? nameMatch[1] : ('Part_' + Date.now().toString(36));

                    let pos = [0, 2, 0];
                    const posMatch = rest.match(/at\s*\(([^)]+)\)/);
                    if (posMatch) {
                        const coords = posMatch[1].split(',').map(n => parseFloat(n.trim()));
                        if (coords.length >= 3) pos = coords;
                    }

                    let scale = [2, 2, 2];
                    const sizeMatch = rest.match(/size\s*\(([^)]+)\)/);
                    if (sizeMatch) {
                        const dims = sizeMatch[1].split(',').map(n => parseFloat(n.trim()));
                        if (dims.length >= 3) scale = dims;
                    }

                    let color = '#06b6d4';
                    const colMatch = rest.match(/color\s*"([^"]+)"/);
                    if (colMatch) color = colMatch[1];

                    let mat = 'metal';
                    const matMatch = rest.match(/material\s*"([^"]+)"/);
                    if (matMatch) mat = matMatch[1];

                    newParts.push({ id: name, name: name, type: type, pos: pos, scale: scale, color: color, material: mat });
                }
            }

            if (newParts.length > 0) {
                while (this.modelGroup.children.length > 0) {
                    this.modelGroup.remove(this.modelGroup.children[0]);
                }
                this.objects = [];
                for (let p of newParts) {
                    this.addPart(p);
                }
                this.selectObject(newParts[0].id);
                webforgeToast('TechScript recompiled: ' + newParts.length + ' parts rendered in 3D');
            } else {
                webforgeToast('No valid "add" declarations found.');
            }
        } catch (err) {
            console.error('Compiler error:', err);
            webforgeToast('Compiler syntax error.');
        }
    },

    copyCode() {
        const editor = document.getElementById('live-txs-editor');
        if (editor) {
            navigator.clipboard.writeText(editor.value).then(() => {
                webforgeToast('TechScript source copied to clipboard!');
            });
        }
    },

    toggleWireframe() {
        this.wireframe = !this.wireframe;
        this.modelGroup.traverse(child => {
            if (child.material) child.material.wireframe = this.wireframe;
        });
        const btn = document.getElementById('btn-wireframe');
        if (btn) btn.classList.toggle('active', this.wireframe);
    },

    toggleGrid() {
        const vis = !this.gridHelper.visible;
        this.gridHelper.visible = vis;
        this.axesHelper.visible = vis;
        const btn = document.getElementById('btn-grid');
        if (btn) btn.classList.toggle('active', vis);
    },

    toggleFlatShading() {
        this.flatShading = !this.flatShading;
        this.modelGroup.traverse(child => {
            if (child.material) {
                child.material.flatShading = this.flatShading;
                child.material.needsUpdate = true;
            }
        });
        const btn = document.getElementById('btn-flat');
        if (btn) btn.classList.toggle('active', this.flatShading);
    },

    resetCamera() {
        if (this.camera && this.controls) {
            this.camera.position.set(12, 14, 20);
            this.controls.target.set(0, 2, 0);
            this.controls.update();
        }
    },

    resetScene() {
        this.loadTemplate('house');
        this.resetCamera();
        webforgeToast('Scene reset to default.');
    },

    // Animation Engine & Timeline
    togglePlayPause() {
        this.animPlaying = !this.animPlaying;
        const btn = document.getElementById('tl-play-pause-btn');
        if (btn) btn.innerText = this.animPlaying ? '⏸ Pause' : '▶ Play';
    },

    rewindAnimation() {
        this.animTime = 0;
        this.applyAnimationAtTime(0);
        const scrubber = document.getElementById('tl-scrubber');
        if (scrubber) scrubber.value = 0;
        const tc = document.getElementById('tl-timecode');
        if (tc) tc.innerText = '00:00.0';
    },

    toggleLoop() {
        this.animLoop = !this.animLoop;
        const btn = document.getElementById('tl-loop-btn');
        if (btn) btn.innerText = '🔁 Loop: ' + (this.animLoop ? 'ON' : 'OFF');
    },

    setAnimSpeed(speed) {
        this.animSpeed = parseFloat(speed) || 1.0;
    },

    setAnimMode(mode) {
        this.animMode = mode;
    },

    seekScrubber(val) {
        this.animTime = (parseFloat(val) / 100) * 5.0;
        this.applyAnimationAtTime(this.animTime);
        const tc = document.getElementById('tl-timecode');
        if (tc) tc.innerText = '00:0' + this.animTime.toFixed(1);
    },

    applyAnimationAtTime(t) {
        if (!this.modelGroup) return;
        const phase = (t / 5.0) * Math.PI * 2;

        if (this.animMode === 'turntable') {
            this.modelGroup.rotation.y = phase;
            this.modelGroup.position.y = 0;
            this.modelGroup.scale.set(1, 1, 1);
        } else if (this.animMode === 'float') {
            this.modelGroup.rotation.y = phase * 0.5;
            this.modelGroup.position.y = Math.sin(phase * 2) * 0.45;
            this.modelGroup.scale.set(1, 1, 1);
        } else if (this.animMode === 'pulse') {
            const s = 1.0 + Math.sin(phase * 2) * 0.08;
            this.modelGroup.scale.set(s, s, s);
            this.modelGroup.rotation.y = phase * 0.25;
            this.modelGroup.position.y = 0;
        } else if (this.animMode === 'walk') {
            this.modelGroup.position.y = Math.abs(Math.sin(phase * 4)) * 0.35;
            this.modelGroup.rotation.z = Math.sin(phase * 4) * 0.04;
            this.modelGroup.rotation.y = phase * 0.5;
        }
    },

    // Exporters
    exportGLTF() {
        if (typeof THREE.GLTFExporter !== 'undefined') {
            const exporter = new THREE.GLTFExporter();
            exporter.parse(this.modelGroup, (gltf) => {
                const blob = new Blob([JSON.stringify(gltf, null, 2)], { type: 'application/json' });
                this.downloadBlob(blob, 'techscript_model.gltf');
                webforgeToast('Exported GLTF model successfully!');
            }, { binary: false });
        } else {
            webforgeToast('GLTF Exporter not loaded.');
        }
    },

    exportOBJ() {
        if (typeof THREE.OBJExporter !== 'undefined') {
            const exporter = new THREE.OBJExporter();
            const result = exporter.parse(this.modelGroup);
            const blob = new Blob([result], { type: 'text/plain' });
            this.downloadBlob(blob, 'techscript_model.obj');
            webforgeToast('Exported OBJ mesh successfully!');
        } else {
            webforgeToast('OBJ Exporter not loaded.');
        }
    },

    exportSTL() {
        if (typeof THREE.STLExporter !== 'undefined') {
            const exporter = new THREE.STLExporter();
            const result = exporter.parse(this.modelGroup);
            const blob = new Blob([result], { type: 'text/plain' });
            this.downloadBlob(blob, 'techscript_model.stl');
            webforgeToast('Exported STL for 3D printing successfully!');
        } else {
            webforgeToast('STL Exporter not loaded.');
        }
    },

    exportTXS() {
        const code = this.generateTxsFromScene();
        const blob = new Blob([code], { type: 'text/plain' });
        this.downloadBlob(blob, 'model.txs');
        webforgeToast('Exported TechScript source (.txs) successfully!');
    },

    downloadBlob(blob, filename) {
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }
};

document.addEventListener('DOMContentLoaded', () => {
    webforgeRouter.init();
    document.querySelectorAll('.webforge-data-section').forEach(sec => {
        const id = sec.querySelector('[id$="-container"]')?.id;
        if (id) {
            const table = id.replace(/-container$/, '');
            webforgeLoadDatabase(table);
        }
    });
});
"#
}
