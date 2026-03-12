# -*- mode: python ; coding: utf-8 -*-


a = Analysis(
    ['C:\\Users\\tanmoy\\Documents\\jast work on this now\\techscript\\installer_build\\_setup_main.py'],
    pathex=[],
    binaries=[],
    datas=[('C:\\Users\\tanmoy\\Documents\\jast work on this now\\techscript\\dist\\tech.exe', '.')],
    hiddenimports=[],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    optimize=0,
)
pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name='TechScript-Setup',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
