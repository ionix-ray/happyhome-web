import os
from PIL import Image, ImageDraw, ImageFont

BASE_PATH = "assets/images/hero_hospital_ambulance.jpg"
LOGO_PATH = "assets/images/happy-home-logo-transparent-v2.png"
FAV_PATH = "assets/images/fav-logo.png"
OUT_PATH = "assets/images/hero_hospital_ambulance.jpg"
VERIFIED_PATH = "/Users/samirparhi-dev/.gemini/antigravity-ide/brain/9a8976b0-cec9-4893-b523-4282c37b1a3f/verified_hero_hospital_ambulance.jpg"

base = Image.open(BASE_PATH).convert("RGBA")
logo = Image.open(LOGO_PATH).convert("RGBA")
fav = Image.open(FAV_PATH).convert("RGBA")

draw = ImageDraw.Draw(base)

# Load clean system fonts
try:
    font_bold_16 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 16)
    font_bold_13 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 13)
    font_bold_11 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 11)
    font_bold_10 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 10)
    font_bold_9 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 9)
    font_bold_8 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 8)
    font_bold_7 = ImageFont.truetype("/System/Library/Fonts/Supplemental/Arial Bold.ttf", 7)
except Exception:
    font_bold_16 = font_bold_13 = font_bold_11 = font_bold_10 = font_bold_9 = font_bold_8 = font_bold_7 = ImageFont.load_default()

# ──────────────────────────────────────────────────────────────────────────
# 1. HOSPITAL BUILDING ENTRANCE SIGNBOARD (x: 570..865, y: 478..635)
# ──────────────────────────────────────────────────────────────────────────
sign_bg = (244, 246, 249, 255)
# Fill building signage panel cleanly over previous text and gold crest
draw.rectangle([570, 478, 865, 635], fill=sign_bg)

# Subtle metallic border
draw.rectangle([570, 478, 865, 635], outline=(215, 222, 230, 255), width=1)
# Top brand accent stripe (Teal & Blue)
draw.line([(570, 479), (715, 479)], fill=(45, 143, 123, 255), width=3)
draw.line([(715, 479), (865, 479)], fill=(19, 76, 179, 255), width=3)

# Official Happy Home Hospital Logo
sign_logo_w = 236
sign_logo_h = int(logo.height * (sign_logo_w / logo.width))
sign_logo = logo.resize((sign_logo_w, sign_logo_h), Image.Resampling.LANCZOS)
logo_x = 570 + (295 - sign_logo_w) // 2
logo_y = 486
base.paste(sign_logo, (logo_x, logo_y), sign_logo)

# Official Subtitle & Accreditations
draw.text((588, 598), "BHUBANESWAR • 24/7 EMERGENCY & CRITICAL CARE", fill=(19, 76, 179, 255), font=font_bold_10)
draw.text((605, 614), "ACCREDITED INPATIENT NEURO REHABILITATION", fill=(45, 143, 123, 255), font=font_bold_9)

# ──────────────────────────────────────────────────────────────────────────
# 2. AMBULANCE ROOF LIGHTBOX (x: 852..938, y: 603..626)
# ──────────────────────────────────────────────────────────────────────────
draw.rectangle([852, 603, 938, 626], fill=(255, 255, 255, 255), outline=(200, 210, 220, 255))
draw.text((857, 607), "AMBULANCE", fill=(220, 38, 38, 255), font=font_bold_11)

# ──────────────────────────────────────────────────────────────────────────
# 3. AMBULANCE WINDSHIELD VISOR (x: 828..938, y: 637..659)
# ──────────────────────────────────────────────────────────────────────────
draw.rectangle([828, 637, 938, 659], fill=(220, 38, 38, 255), outline=(185, 28, 28, 255))
draw.text((833, 642), "HAPPY HOME HOSPITAL", fill=(255, 255, 255, 255), font=font_bold_8)

# ──────────────────────────────────────────────────────────────────────────
# 4. AMBULANCE FRONT BONNET (x: 755..875, y: 700..728)
# ──────────────────────────────────────────────────────────────────────────
draw.rectangle([755, 700, 875, 728], fill=(255, 255, 255, 255), outline=(220, 230, 240, 255))
bonnet_fav = fav.resize((20, 20), Image.Resampling.LANCZOS)
base.paste(bonnet_fav, (760, 704), bonnet_fav)
draw.text((785, 705), "ECNALUBMA", fill=(220, 38, 38, 255), font=font_bold_13)

# ──────────────────────────────────────────────────────────────────────────
# 5. AMBULANCE SIDE BODY PANEL (x: 970..1055, y: 642..728)
# ──────────────────────────────────────────────────────────────────────────
# Clean white side panel covering previous typos
draw.rectangle([970, 642, 1055, 728], fill=(255, 255, 255, 255), outline=(220, 230, 240, 255))

# Side official logo
side_logo_w = 78
side_logo_h = int(logo.height * (side_logo_w / logo.width))
side_logo = logo.resize((side_logo_w, side_logo_h), Image.Resampling.LANCZOS)
base.paste(side_logo, (973, 646), side_logo)

# Side emergency text
draw.text((974, 688), "24/7 EMERGENCY", fill=(220, 38, 38, 255), font=font_bold_8)
draw.text((974, 702), "ADVANCED ICU", fill=(2, 132, 199, 255), font=font_bold_8)
draw.text((974, 715), "AMBULANCE", fill=(15, 23, 42, 255), font=font_bold_8)

# Save final images
final_rgb = base.convert("RGB")
final_rgb.save(OUT_PATH, quality=96)
final_rgb.save(VERIFIED_PATH, quality=96)
print("Successfully generated brand-aware hero to OUT_PATH and VERIFIED_PATH")
