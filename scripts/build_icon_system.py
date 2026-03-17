import os
import sys
import subprocess

def ensure_deps():
    try:
        import fitz  # PyMuPDF
        from PIL import Image
    except ImportError:
        print("Installing PyMuPDF and Pillow...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", "PyMuPDF", "Pillow"])
        os.execv(sys.executable, ['python'] + sys.argv)

ensure_deps()

import fitz
from PIL import Image

svg_path = "assets/logo.svg"
png_sizes = [16, 32, 48, 64, 128, 256, 512]

os.makedirs("assets/icons", exist_ok=True)
os.makedirs("vscode-extension/icons", exist_ok=True)

def upgrade_image(img):
    """Adds auto-crop, glow, and color boost."""
    # Convert to RGBA
    img = img.convert("RGBA")
    
    # Auto-crop empty space
    bbox = img.getbbox()
    if bbox:
        img = img.crop(bbox)
    
    # Create padding for glow (10% on each side)
    w, h = img.size
    pad_w, pad_h = int(w * 0.1), int(h * 0.1)
    new_img = Image.new("RGBA", (w + pad_w*2, h + pad_h*2), (0,0,0,0))
    new_img.paste(img, (pad_w, pad_h))
    img = new_img
    
    # Multi-pass Bloom Effect
    from PIL import ImageFilter, ImageEnhance
    
    # 1. Base glow mask (extract alpha)
    glow_mask = img.split()[3]
    
    # 2. Bloom layers
    bloom = Image.new("RGBA", img.size, (0,0,0,0))
    
    # Color references from the SVG
    azure = (0, 147, 237, 255)  # #0493ED
    crimson = (173, 20, 46, 255) # #AD142E
    
    # Create a gradient tint for the glow
    tint = Image.new("RGBA", img.size, (0,0,0,0))
    from PIL import ImageDraw
    draw = ImageDraw.Draw(tint)
    draw.rectangle([0, 0, img.size[0]//2, img.size[1]], fill=azure)
    draw.rectangle([img.size[0]//2, 0, img.size[0], img.size[1]], fill=crimson)
    
    # Add blurred layers
    for radius in [10, 25, 45]:
        layer = tint.copy()
        mask = glow_mask.filter(ImageFilter.GaussianBlur(radius))
        layer.putalpha(mask)
        bloom = Image.alpha_composite(bloom, layer)
    
    # 3. Combine bloom with boosted original
    enhancer = ImageEnhance.Color(img)
    img = enhancer.enhance(1.8) # Saturation boost
    enhancer = ImageEnhance.Brightness(img)
    img = enhancer.enhance(1.4) # Brightness boost
    
    # Final composite
    final = Image.alpha_composite(bloom, img)
    
    # Square it up
    fw, fh = final.size
    side = max(fw, fh)
    squared = Image.new("RGBA", (side, side), (0,0,0,0))
    squared.paste(final, ((side-fw)//2, (side-fh)//2))
    
    return squared

print("Rendering Master SVG to PNG via PyMuPDF...")
doc = fitz.open(svg_path)
page = doc[0]
# Render at higher resolution for quality
pix = page.get_pixmap(alpha=True, matrix=fitz.Matrix(2, 2)) 

temp_master = "assets/icons/temp_raw.png"
pix.save(temp_master)

print("Applying Visual Enhancements (Glow/Crop/Boost)...")
raw_img = Image.open(temp_master)
img = upgrade_image(raw_img)
master_png = "assets/icons/master_1024_enhanced.png"
img.resize((1024, 1024), Image.Resampling.LANCZOS).save(master_png)
img = Image.open(master_png) # Reload for consistency

print("Generating multi-size PNGs...")
# ... [rest of the size loop and saving logic remains the same but uses the 'img' object]
ico_images = []
for size in png_sizes:
    resized = img.resize((size, size), Image.Resampling.LANCZOS)
    resized.save(f"assets/icons/logo_{size}.png")
    ico_images.append(resized)

print("Packaging Windows .ico...")
img.save("assets/techscript.ico", format="ICO", sizes=[(s, s) for s in [16, 32, 48, 64, 128, 256]])

print("Packaging macOS .icns...")
try:
    img.save("assets/techscript.icns", format="ICNS", append_images=ico_images)
except Exception as e:
    img.save("assets/techscript.icns", format="ICNS")

print("Updating VS Code Extension Icons...")
img.resize((256, 256), Image.Resampling.LANCZOS).save("vscode-extension/icons/ts_final_logo_v3.png")
img.resize((64, 64), Image.Resampling.LANCZOS).save("vscode-extension/icons/txs_final_file_logo_v3.png")

print("Updating Public Release Assets...")
img.resize((512, 512), Image.Resampling.LANCZOS).save("public-release/logo_enhanced.png")
img.save("public-release/techscript_enhanced.ico", format="ICO", sizes=[(s, s) for s in [16, 32, 48, 64, 128, 256]])

print("Icon system generation complete!")
