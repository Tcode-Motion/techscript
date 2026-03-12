
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
