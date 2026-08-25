use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability, error::RuntimeError, error::RuntimeErrorKind, value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_compress(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "zip".to_string(),
            Rc::new(StdFunction {
                name: "zip".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let src_dir = args[0].try_into_string()?;
                    let archive_path = args[1].try_into_string()?;
                    zip_dir(&src_dir, &archive_path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("ZIP error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "unzip".to_string(),
            Rc::new(StdFunction {
                name: "unzip".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let archive_path = args[0].try_into_string()?;
                    let dest_dir = args[1].try_into_string()?;
                    unzip_archive(&archive_path, &dest_dir).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("UNZIP error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "tar".to_string(),
            Rc::new(StdFunction {
                name: "tar".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let src_dir = args[0].try_into_string()?;
                    let archive_path = args[1].try_into_string()?;
                    tar_dir(&src_dir, &archive_path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("TAR error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "untar".to_string(),
            Rc::new(StdFunction {
                name: "untar".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let archive_path = args[0].try_into_string()?;
                    let dest_dir = args[1].try_into_string()?;
                    untar_archive(&archive_path, &dest_dir).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("UNTAR error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "gzip".to_string(),
            Rc::new(StdFunction {
                name: "gzip".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let src_file = args[0].try_into_string()?;
                    let archive_path = args[1].try_into_string()?;
                    gzip_file(&src_file, &archive_path).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("GZIP error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "gunzip".to_string(),
            Rc::new(StdFunction {
                name: "gunzip".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::FileSystem) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: FileSystem capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let archive_path = args[0].try_into_string()?;
                    let dest_file = args[1].try_into_string()?;
                    gunzip_archive(&archive_path, &dest_file).map_err(|e| {
                        RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(format!("GUNZIP error: {}", e)),
                            None,
                            None,
                        )
                    })?;
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.compress",
            StdlibModule {
                name: "std.compress".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::FileSystem],
            },
        );
    }
}

pub fn zip_dir(src_dir: &str, dst_file: &str) -> std::io::Result<()> {
    let file = File::create(dst_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let walkdir = std::fs::read_dir(src_dir)?;
    for entry in walkdir {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            let name = entry_path.file_name().unwrap().to_string_lossy();
            zip.start_file(name, options)?;
            let mut f = File::open(entry_path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    zip.finish()?;
    Ok(())
}

pub fn unzip_archive(archive_path: &str, dest_dir: &str) -> std::io::Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    std::fs::create_dir_all(dest_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(dest_dir).join(path.to_owned()),
            None => continue,
        };

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

pub fn tar_dir(src_dir: &str, dst_file: &str) -> std::io::Result<()> {
    let file = File::create(dst_file)?;
    let mut a = tar::Builder::new(file);
    a.append_dir_all(".", src_dir)?;
    a.finish()?;
    Ok(())
}

pub fn untar_archive(archive_path: &str, dest_dir: &str) -> std::io::Result<()> {
    let file = File::open(archive_path)?;
    let mut a = tar::Archive::new(file);

    // The tar crate's `unpack_in` method already has built-in directory traversal
    // protections which prevent absolute paths and parent directory traversals
    // from escaping the destination directory. Therefore, we revert the manual
    // path validation that caused a regression with uncanonicalized relative paths.

    a.unpack(dest_dir)?;
    Ok(())
}

pub fn gzip_file(src_file: &str, dst_file: &str) -> std::io::Result<()> {
    let mut input = File::open(src_file)?;
    let output = File::create(dst_file)?;
    let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::default());
    std::io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

pub fn gunzip_archive(archive_path: &str, dst_file: &str) -> std::io::Result<()> {
    let input = File::open(archive_path)?;
    let mut decoder = flate2::read::GzDecoder::new(input);
    let mut output = File::create(dst_file)?;
    std::io::copy(&mut decoder, &mut output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_unzip_archive_invalid_path() {
        let result = unzip_archive("non_existent_archive.zip", "dest_dir");
        assert!(result.is_err());
    }

    #[test]
    fn test_unzip_archive_corrupted_zip() {
        let temp_dir = std::env::temp_dir().join("techscript_unzip_test");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let corrupted_zip_path = temp_dir.join("corrupted.zip");
        std::fs::write(&corrupted_zip_path, b"not a valid zip file").unwrap();

        let dest_dir = temp_dir.join("dest");

        let result = unzip_archive(corrupted_zip_path.to_str().unwrap(), dest_dir.to_str().unwrap());
        assert!(result.is_err());

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}
