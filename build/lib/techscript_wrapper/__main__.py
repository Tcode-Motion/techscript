import os
import sys
import platform
import urllib.request
import subprocess

VERSION = "1.0.4.5"
REPO = "Tcode-Motion/techscript"

def download_binary():
    """Attempts to download the high-performance native VM."""
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    # We only have native binaries for Windows currently
    asset_name = None
    if system == "windows":
        asset_name = "techscriptv1.0.4.3.exe"
    
    if not asset_name:
        return None
        
    url = f"https://github.com/{REPO}/releases/download/v1.0.4.3/{asset_name}"
    
    bin_dir = os.path.join(os.path.expanduser("~"), ".techscript", "bin")
    os.makedirs(bin_dir, exist_ok=True)
    
    exe_path = os.path.join(bin_dir, "tech.exe" if system == "windows" else "tech")
    
    if not os.path.exists(exe_path):
        print(f"Downloading TechScript Native Engine for peak performance...")
        try:
            urllib.request.urlretrieve(url, exe_path)
            print("Download complete!")
        except Exception:
            return None # Fail silently and use Python fallback
            
    return exe_path

def run_python_engine():
    """Fallback to the built-in Python interpreter."""
    try:
        # Use absolute import now that techscript is a top-level package
        from techscript.cli import main as python_main
        python_main()
    except ImportError:
        # If absolute import fails, try relative just in case
        try:
            from .techscript.cli import main as python_main
            python_main()
        except Exception as e:
            print(f"Error: Could not launch TechScript engine: {e}")
            sys.exit(1)
    except Exception as e:
        print(f"Error: Could not launch TechScript engine: {e}")
        sys.exit(1)

def main():
    system = platform.system().lower()
    
    # Try native engine on Windows
    if system == "windows":
        exe_path = download_binary()
        if exe_path and os.path.exists(exe_path):
            try:
                sys.exit(subprocess.call([exe_path] + sys.argv[1:]))
            except Exception:
                pass # Try python fallback if native fails
    
    # Fallback to Python engine for Android (Termux), Linux, Mac
    run_python_engine()

if __name__ == "__main__":
    main()
