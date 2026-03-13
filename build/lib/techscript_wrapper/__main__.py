import os
import sys
import platform
import urllib.request
import subprocess

VERSION = "1.0.4.3"
REPO = "Tcode-Motion/techscript"

def download_binary():
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    # Define asset name based on OS Architecture
    asset_name = None
    if system == "windows":
        asset_name = "techscriptv1.0.4.3.exe"
    elif system == "linux":
        asset_name = "tech-linux-x64" # Placeholder for future linux release
    elif system == "darwin":
        asset_name = "tech-macos-x64" # Placeholder for future mac release
        
    if not asset_name:
        print(f"Unsupported system: {system} {machine}")
        sys.exit(1)
        
    url = f"https://github.com/{REPO}/releases/download/v{VERSION}/{asset_name}"
    
    bin_dir = os.path.join(os.path.expanduser("~"), ".techscript", "bin")
    os.makedirs(bin_dir, exist_ok=True)
    
    exe_path = os.path.join(bin_dir, "tech.exe" if system == "windows" else "tech")
    
    if not os.path.exists(exe_path):
        print(f"Downloading TechScript v{VERSION} for {system}...")
        try:
            urllib.request.urlretrieve(url, exe_path)
            if system != "windows":
                os.chmod(exe_path, 0o755)
            print("Download complete!")
        except Exception as e:
            print(f"Failed to download native binary: {e}")
            sys.exit(1)
            
    return exe_path

def main():
    exe_path = download_binary()
    try:
        sys.exit(subprocess.call([exe_path] + sys.argv[1:]))
    except KeyboardInterrupt:
        sys.exit(130)

if __name__ == "__main__":
    main()
