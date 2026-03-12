"""TechScript Simple Windows Installer Builder - Step approach"""
import os, sys, subprocess, shutil

PROJECT_DIR = os.path.dirname(os.path.abspath(__file__))
SRC_MAIN = os.path.join(PROJECT_DIR, "src", "techscript", "__main__.py")

# Check for icon
ICON = os.path.join(PROJECT_DIR, "assets", "icons", "techscript.ico")
icon_args = ["--icon", ICON] if os.path.exists(ICON) else []

OUT = os.path.join(PROJECT_DIR, "installer_build")
os.makedirs(OUT, exist_ok=True)

print("\n[1/2] Fetching native tech.exe ...")
tech_exe = os.path.join(PROJECT_DIR, "dist", "tech.exe")
if not os.path.exists(tech_exe):
    print("tech.exe not found in dist/. Please run cargo build."); sys.exit(1)
print(f"  Found: {tech_exe}")

print("\n[2/2] Building TechScript-Setup.exe ...")

setup_py = os.path.join(OUT, "_setup_main.py")
code = r'''
import os, sys, shutil, subprocess, winreg, ctypes
from pathlib import Path

INSTALL_DIR = Path(os.environ.get("LOCALAPPDATA","C:/Users/Public")) / "TechScript"

def add_user_path(d):
    key = winreg.OpenKey(winreg.HKEY_CURRENT_USER, "Environment", 0, winreg.KEY_ALL_ACCESS)
    cur, _ = winreg.QueryValueEx(key, "Path")
    if str(d).lower() not in cur.lower():
        winreg.SetValueEx(key, "Path", 0, winreg.REG_EXPAND_SZ, str(d)+";"+cur)
    winreg.CloseKey(key)

def main():
    print("\n  ================================================")
    print("    TechScript v2 - Windows Installer")
    print("  ================================================\n")
    INSTALL_DIR.mkdir(parents=True, exist_ok=True)
    src = Path(sys.executable).parent / "tech.exe"
    if not src.exists():
        src = Path(sys._MEIPASS) / "tech.exe"
    dst = INSTALL_DIR / "tech.exe"
    print(f"  [1/3] Installing to {INSTALL_DIR} ...")
    shutil.copy2(src, dst)
    print("  [2/3] Updating PATH ...")
    add_user_path(INSTALL_DIR)
    print("  [3/3] Registering .txs files ...")
    subprocess.run(f'assoc .txs=TechScript.File', shell=True, capture_output=True)
    subprocess.run(f'ftype TechScript.File="{dst}" run "%1"', shell=True, capture_output=True)
    print("\n  ================================================")
    print("    Done! Open a new terminal and type:")
    print("      tech version")
    print("  ================================================\n")
    input("  Press Enter to close...")

if __name__ == "__main__":
    main()
'''
with open(setup_py, "w") as f:
    f.write(code)

cmd2 = [
    sys.executable, "-m", "PyInstaller",
    "--onefile", "--name", "TechScript-Setup",
    "--add-data", f"{tech_exe};.",
    "--distpath", OUT,
    "--workpath", os.path.join(OUT, "_work2"),
    "--specpath", OUT,
    "--console",
    setup_py
] + icon_args

r2 = subprocess.run(cmd2)
if r2.returncode != 0:
    print("Setup EXE build failed."); sys.exit(1)

setup_exe = os.path.join(OUT, "TechScript-Setup.exe")
if os.path.exists(setup_exe):
    size_mb = os.path.getsize(setup_exe)//1024//1024
    dest = os.path.join(PROJECT_DIR, "public-release", "setup.exe")
    shutil.copy2(setup_exe, dest)
    print(f"\n  ✓ setup.exe ({size_mb} MB)")
    print(f"  Saved to: {dest}")
else:
    print("Setup EXE not found"); sys.exit(1)
