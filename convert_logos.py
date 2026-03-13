import sys
import subprocess
import os
from pathlib import Path
import time

def install_deps():
    try:
        from PIL import Image
    except ImportError:
        subprocess.check_call([sys.executable, "-m", "pip", "install", "Pillow"])

install_deps()
from PIL import Image

def find_browser():
    paths = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"
    ]
    for p in paths:
        if os.path.exists(p):
            return p
    return None

def convert():
    base_dir = Path(r"c:\Users\tanmoy\Documents\jast work on this now\techscript")
    logo_dir = base_dir / "logo" / "black backround logo"
    svg_file = logo_dir / "logo-dark.svg"
    
    out_dir = base_dir / "assets" / "icons"
    out_dir.mkdir(parents=True, exist_ok=True)
    
    print(f"Reading SVG: {svg_file}")
    if not svg_file.exists():
        print(f"Error: {svg_file} not found!")
        sys.exit(1)
        
    png_file = out_dir / "icon.png"
    ico_file = out_dir / "icon.ico"
    
    browser = find_browser()
    if not browser:
        print("Error: Could not find Edge or Chrome to render SVG.")
        sys.exit(1)
    
    print(f"Using browser: {browser}")
    html_content = f"""
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            body {{ margin: 0; padding: 0; overflow: hidden; background: transparent; display: flex; align-items: center; justify-content: center; height: 100vh; width: 100vw; }}
            img {{ width: 1024px; height: 1024px; object-fit: contain; }}
        </style>
    </head>
    <body style="background: transparent;">
        <img src="{svg_file.as_uri()}">
    </body>
    </html>
    """
    
    html_file = base_dir / "temp_render.html"
    html_file.write_text(html_content, encoding="utf-8")
    
    # We must quote paths and use the screenshot argument correctly
    print("Capturing screenshot...")
    
    # msedge --headless --screenshot="C:\path\to\out.png" --window-size=1024,1024 "file://C:/path/to/temp.html"
    # Actually, Chrome headless screenshot doesn't output transparency by default, but it's okay for an icon.
    cmd = [
        browser,
        "--headless",
        "--disable-gpu",
        "--hide-scrollbars",
        f"--window-size=1024,1024",
        f"--screenshot={png_file}",
        html_file.as_uri()
    ]
    subprocess.run(cmd, check=True)
    
    time.sleep(1) # wait for file writing
    html_file.unlink()
    
    if not png_file.exists():
        print("Error: Screenshot failed to generate PNG.")
        sys.exit(1)
        
    print(f"Post-processing {png_file}...")
    img = Image.open(png_file)
    # The screenshot might have a white background, setting transparent if possible is hard, but we accept opaque.
    
    print(f"Converting to {ico_file} and other resolutions...")
    # Generate ICO
    img.save(ico_file, format="ICO", sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
    
    # Generate specific sizes
    for size in [16, 32, 64, 128, 256, 512]:
        resized = img.resize((size, size), Image.Resampling.LANCZOS)
        resized.save(out_dir / f"icon-{size}.png")
        
    # Overwrite extension icon (128x128)
    ext_icon = base_dir / "vscode-extension" / "icon.png"
    resized = img.resize((128, 128), Image.Resampling.LANCZOS)
    resized.save(ext_icon)
    
    print("Logos successfully generated!")

if __name__ == "__main__":
    convert()
