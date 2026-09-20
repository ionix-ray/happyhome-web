import os
from PIL import Image, ImageDraw, ImageFont

BASE_IMG_PATH = "assets/images/hero_hospital_ambulance.jpg"
LOGO_PATH = "assets/images/happy-home-logo-transparent-v2.png"
FAV_LOGO_PATH = "assets/images/fav-logo.png"
OUT_IMG_PATH = "/Users/samirparhi-dev/.gemini/antigravity-ide/brain/9a8976b0-cec9-4893-b523-4282c37b1a3f/brand_aware_hero.jpg"

base = Image.open(BASE_IMG_PATH).convert("RGBA")
logo = Image.open(LOGO_PATH).convert("RGBA")
fav_logo = Image.open(FAV_LOGO_PATH).convert("RGBA")

# Try finding a crisp font
try:
    font_large = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 26)
    font_med = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 16)
    font_small = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 12)
    font_sub = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 10)
except Exception:
    font_large = font_med = font_small = font_sub = ImageFont.load_default()

# ──────────────────────────────────────────────────────────────────────────
# 1. HOSPITAL ENTRANCE PORCH BEAM SIGNAGE (x: 320..820, y: 435..505)
# ──────────────────────────────────────────────────────────────────────────
# The front fascia of the porch is a clean white architectural beam.
# Let's place the official logo prominently in the center of this beam!
# Beam dimensions: width ~ 460, height ~ 65
porch_logo_h = 58
porch_logo_w = int(logo.width * (porch_logo_h / logo.height))
porch_logo = logo.resize((porch_logo_w, porch_logo_h), Image.Resampling.LANCZOS)

# Position centered on the porch fascia
porch_x = 410
porch_y = 442
base.paste(porch_logo, (porch_x, porch_y), porch_logo)

# Also add subtle 3D drop shadow underneath
draw = ImageDraw.Draw(base)

# ──────────────────────────────────────────────────────────────────────────
# 2. AMBULANCE ROOF LIGHT BOX (x: 700..815, y: 632..665)
# ──────────────────────────────────────────────────────────────────────────
# Clean white lightbox panel with bold red "AMBULANCE"
# Draw clean white rounded rect over the existing typo "ANBULANCE"
draw.rectangle([700, 632, 816, 665], fill=(255, 255, 255, 255), outline=(210, 220, 230, 255))
# Text AMBULANCE in bold red
draw.text((712, 637), "AMBULANCE", fill=(220, 38, 38, 255), font=font_med)

# ──────────────────────────────────────────────────────────────────────────
# 3. AMBULANCE WINDSHIELD VISOR (x: 660..845, y: 712..746)
# ──────────────────────────────────────────────────────────────────────────
# Clean red visor band with "HAPPY HOME HOSPITAL"
draw.rectangle([662, 712, 842, 746], fill=(220, 38, 38, 255), outline=(185, 28, 28, 255))
draw.text((674, 720), "HAPPY HOME HOSPITAL", fill=(255, 255, 255, 255), font=font_med)

# ──────────────────────────────────────────────────────────────────────────
# 4. AMBULANCE FRONT BONNET (x: 720..860, y: 775..805)
# ──────────────────────────────────────────────────────────────────────────
# White bonnet surface with clean reverse medical "AMBULANCE" and small emblem
draw.rectangle([715, 775, 870, 804], fill=(255, 255, 255, 255))
# Place small lotus emblem on bonnet
bonnet_logo = fav_logo.resize((24, 24), Image.Resampling.LANCZOS)
base.paste(bonnet_logo, (720, 777), bonnet_logo)
draw.text((752, 781), "ECNALUBMA", fill=(220, 38, 38, 255), font=font_med)

# ──────────────────────────────────────────────────────────────────────────
# 5. AMBULANCE SIDE BODY PANEL (x: 935..1095, y: 720..810)
# ──────────────────────────────────────────────────────────────────────────
# Cover the garbled side text with clean white livery panel
draw.rectangle([935, 718, 1098, 808], fill=(255, 255, 255, 255), outline=(226, 232, 240, 255))
# Place authentic Happy Home Hospital logo on side
side_logo_h = 44
side_logo_w = int(logo.width * (side_logo_h / logo.height))
side_logo = logo.resize((side_logo_w, side_logo_h), Image.Resampling.LANCZOS)
base.paste(side_logo, (940, 722), side_logo)

# Subtitle text on side panel
draw.text((942, 772), "24/7 ADVANCED LIFE SUPPORT", fill=(2, 132, 199, 255), font=font_sub)
draw.text((942, 788), "CRITICAL CARE ICU AMBULANCE", fill=(15, 23, 42, 255), font=font_sub)

# Save final JPEG
final_rgb = base.convert("RGB")
final_rgb.save(OUT_IMG_PATH, quality=96)
print("Saved brand_aware_hero.jpg successfully")
