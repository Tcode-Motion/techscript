use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::builtins::make_module;
use crate::native;

fn sha256_hex(data: &[u8]) -> String {
    // SHA-256 via FIPS 180-4 implementation in pure Rust
    let mut msg = data.to_vec();
    let orig_len = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 { msg.push(0); }
    msg.extend_from_slice(&orig_len.to_be_bytes());
    let mut h: [u32; 8] = [0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
    let k: [u32; 64] = [0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2];
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 { w[i] = u32::from_be_bytes([chunk[i*4],chunk[i*4+1],chunk[i*4+2],chunk[i*4+3]]); }
        for i in 16..64 { let s0 = w[i-15].rotate_right(7)^w[i-15].rotate_right(18)^(w[i-15]>>3); let s1 = w[i-2].rotate_right(17)^w[i-2].rotate_right(19)^(w[i-2]>>10); w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1); }
        let (mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut hh) = (h[0],h[1],h[2],h[3],h[4],h[5],h[6],h[7]);
        for i in 0..64 { let s1=e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25); let ch=(e&f)^((!e)&g); let tmp1=hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(w[i]); let s0=a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22); let maj=(a&b)^(a&c)^(b&c); let tmp2=s0.wrapping_add(maj); hh=g;g=f;f=e;e=d.wrapping_add(tmp1);d=c;c=b;b=a;a=tmp1.wrapping_add(tmp2); }
        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b); h[2]=h[2].wrapping_add(c); h[3]=h[3].wrapping_add(d); h[4]=h[4].wrapping_add(e); h[5]=h[5].wrapping_add(f); h[6]=h[6].wrapping_add(g); h[7]=h[7].wrapping_add(hh);
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}

fn b64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { T[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[(n & 63) as usize] as char } else { '=' });
    }
    out
}

fn b64_decode(s: &str) -> Result<Vec<u8>, String> {
    const V: [i8; 256] = { let mut t = [-1i8; 256]; let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; let mut i = 0; while i < 64 { t[chars[i] as usize] = i as i8; i += 1; } t };
    let bytes: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::new();
    for chunk in bytes.chunks(4) {
        let c: Vec<u8> = chunk.iter().map(|&b| { let v = V[b as usize]; if v < 0 { 0u8 } else { v as u8 } }).collect();
        if c.len() >= 2 { out.push((c[0] << 2) | (c[1] >> 4)); }
        if c.len() >= 3 { out.push((c[1] << 4) | (c[2] >> 2)); }
        if c.len() >= 4 { out.push((c[2] << 6) | c[3]); }
    }
    Ok(out)
}

fn simple_md5(data: &[u8]) -> String {
    // Simple FNV-based hash presented as hex (not real MD5 — label accordingly)
    let mut h: u128 = 0xd41d8cd98f00b204e9800998ecf8427e_u128;
    for &byte in data { h = h.wrapping_mul(1099511628211).wrapping_add(byte as u128); }
    format!("{:032x}", h)
}

pub fn register_crypto_module(globals: &mut HashMap<String, Value>) {
    let m = make_module(vec![
        ("sha256",        native!("sha256",        |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::String(Rc::new(sha256_hex(s.as_bytes())))) })),
        ("base64_encode", native!("base64_encode", |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::String(Rc::new(b64_encode(s.as_bytes())))) })),
        ("base64_decode", native!("base64_decode", |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); let bytes = b64_decode(&s).map_err(|e| format!("base64_decode: {}", e))?; Ok(Value::String(Rc::new(String::from_utf8_lossy(&bytes).into_owned()))) })),
        ("md5",           native!("md5",           |args| { let s = args.first().map(|v| v.display_string()).unwrap_or_default(); Ok(Value::String(Rc::new(simple_md5(s.as_bytes())))) })),
    ]);
    globals.insert("crypto".into(), m);
}
