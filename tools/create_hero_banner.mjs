import { chromium } from 'playwright';
import path from 'path';
import fs from 'fs';

const OUT_PATH = path.resolve(import.meta.dirname, '..', 'assets/images/hero_hospital_ambulance.jpg');
const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

// Read logo and emergency photo as base64
const logoB64 = fs.readFileSync(path.resolve(import.meta.dirname, '..', 'assets/images/happy-home-logo-transparent.png')).toString('base64');
const ambB64 = fs.readFileSync('/Volumes/hex/happy-home/insta_banner/demo_banner_3_emergency.jpg').toString('base64');

const htmlContent = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Happy Home Hospital & Emergency Banner</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@500;600;700;800;900&display=swap" rel="stylesheet">
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      width: 1200px;
      height: 900px;
      overflow: hidden;
      font-family: 'Plus Jakarta Sans', system-ui, -apple-system, sans-serif;
      background: #091428;
      color: #f8fafc;
      position: relative;
    }

    .bg-layer {
      position: absolute;
      inset: 0;
      background: radial-gradient(circle at 85% 15%, rgba(2, 132, 199, 0.4), transparent 50%),
                  radial-gradient(circle at 10% 85%, rgba(45, 143, 123, 0.3), transparent 45%),
                  linear-gradient(135deg, #091428 0%, #0d213f 50%, #07101f 100%);
    }

    .grid-pattern {
      position: absolute;
      inset: 0;
      background-image: linear-gradient(rgba(255, 255, 255, 0.04) 1px, transparent 1px),
                        linear-gradient(90deg, rgba(255, 255, 255, 0.04) 1px, transparent 1px);
      background-size: 36px 36px;
    }

    .container {
      position: relative;
      z-index: 10;
      width: 100%;
      height: 100%;
      padding: 36px 44px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
    }

    /* Top Brand Bar with Official Logo */
    .top-bar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      background: #ffffff;
      padding: 14px 28px;
      border-radius: 20px;
      box-shadow: 0 12px 36px rgba(0, 0, 0, 0.35);
    }
    .logo-img {
      height: 60px;
      object-fit: contain;
    }
    .top-badges {
      display: flex;
      align-items: center;
      gap: 14px;
    }
    .badge-pill {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 8px 16px;
      border-radius: 9999px;
      font-size: 13px;
      font-weight: 800;
      letter-spacing: 0.5px;
    }
    .badge-emergency {
      background: #fef2f2;
      color: #dc2626;
      border: 1.5px solid #fecaca;
    }
    .badge-nabh {
      background: #f0fdf4;
      color: #059669;
      border: 1.5px solid #bbf7d0;
    }

    /* Main Content Grid */
    .main-grid {
      display: grid;
      grid-template-columns: 1fr 1.08fr;
      gap: 36px;
      align-items: center;
      flex: 1;
      padding: 18px 0;
    }

    .left-col {
      display: flex;
      flex-direction: column;
      gap: 18px;
    }
    .eyebrow {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      color: #38bdf8;
      font-size: 13.5px;
      font-weight: 800;
      letter-spacing: 2px;
      text-transform: uppercase;
    }
    .headline {
      font-size: 42px;
      font-weight: 900;
      line-height: 1.15;
      letter-spacing: -0.5px;
      color: #ffffff;
    }
    .headline span {
      background: linear-gradient(135deg, #38bdf8, #2dd4bf);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    .lead-text {
      font-size: 15.5px;
      line-height: 1.6;
      color: #cbd5e1;
    }

    .features-list {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    .feature-item {
      display: flex;
      align-items: center;
      gap: 12px;
      font-size: 14.5px;
      font-weight: 700;
      color: #f1f5f9;
    }
    .check-icon {
      width: 22px;
      height: 22px;
      border-radius: 50%;
      background: #059669;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      font-size: 13px;
      flex-shrink: 0;
    }

    /* Right visual card showcasing modern emergency ambulance response */
    .visual-card {
      position: relative;
      border-radius: 26px;
      overflow: hidden;
      box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
      border: 2px solid rgba(56, 189, 248, 0.35);
      height: 490px;
      background: #0f2744;
    }
    .visual-card img {
      width: 100%;
      height: 100%;
      object-fit: cover;
      object-position: center;
    }
    .card-overlay-badge {
      position: absolute;
      bottom: 20px;
      left: 20px;
      right: 20px;
      background: rgba(9, 20, 40, 0.92);
      backdrop-filter: blur(16px);
      padding: 14px 18px;
      border-radius: 16px;
      border: 1px solid rgba(255, 255, 255, 0.15);
      display: flex;
      align-items: center;
      justify-content: space-between;
    }
    .overlay-title {
      font-size: 15px;
      font-weight: 800;
      color: #ffffff;
    }
    .overlay-sub {
      font-size: 12px;
      color: #38bdf8;
      margin-top: 2px;
    }
    .ambulance-tag {
      background: #dc2626;
      color: #ffffff;
      padding: 6px 14px;
      border-radius: 9999px;
      font-size: 12px;
      font-weight: 800;
      letter-spacing: 1px;
    }

    /* Bottom Bar */
    .bottom-bar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      background: rgba(15, 23, 42, 0.85);
      border: 1px solid rgba(255, 255, 255, 0.12);
      backdrop-filter: blur(14px);
      padding: 12px 24px;
      border-radius: 16px;
    }
    .bottom-left {
      display: flex;
      align-items: center;
      gap: 20px;
      font-size: 13.5px;
      color: #94a3b8;
    }
    .bottom-left strong {
      color: #f8fafc;
    }
    .call-btn {
      background: linear-gradient(135deg, #0284c7, #0ea5e9);
      color: #ffffff;
      padding: 9px 20px;
      border-radius: 10px;
      font-weight: 800;
      font-size: 13.5px;
      display: flex;
      align-items: center;
      gap: 8px;
    }
  </style>
</head>
<body>
  <div class="bg-layer"></div>
  <div class="grid-pattern"></div>

  <div class="container">
    <!-- Top Bar with Authentic Official Happy Home Hospital Logo -->
    <div class="top-bar">
      <img src="data:image/png;base64,${logoB64}" class="logo-img" alt="Happy Home Hospital">
      <div class="top-badges">
        <div class="badge-pill badge-emergency">
          <span style="display:inline-block; width:8px; height:8px; border-radius:50%; background:#dc2626;"></span>
          24×7 EMERGENCY ON CALL
        </div>
        <div class="badge-pill badge-nabh">
          NABH QUALITY STANDARDS
        </div>
      </div>
    </div>

    <!-- Main Grid -->
    <div class="main-grid">
      <div class="left-col">
        <div class="eyebrow">
          <span style="color:#38bdf8;">✦</span> 24/7 Critical Care &amp; Neuro Center
        </div>
        <h1 class="headline">
          Advanced Hospital Care &amp; <span>Emergency Response.</span>
        </h1>
        <p class="lead-text">
          Happy Home Hospital provides 24/7 emergency trauma resuscitation, advanced neuro ICU telemetry, and rapid cardiac life support ambulances across Bhubaneswar.
        </p>
        <div class="features-list">
          <div class="feature-item">
            <div class="check-icon">✓</div>
            <span>Dedicated Cardiac &amp; Neuro ICU with Ventilators</span>
          </div>
          <div class="feature-item">
            <div class="check-icon">✓</div>
            <span>24/7 Advanced Life Support (ALS) Ambulance on Standby</span>
          </div>
          <div class="feature-item">
            <div class="check-icon">✓</div>
            <span>AIIMS &amp; SCB Trained Senior Critical Care Faculty</span>
          </div>
        </div>
      </div>

      <!-- Right Visual Card with Emergency Ambulance Photo -->
      <div class="visual-card">
        <img src="data:image/jpeg;base64,${ambB64}" alt="Happy Home Hospital Emergency Response Ambulance">
        <div class="card-overlay-badge">
          <div>
            <div class="overlay-title">Happy Home Emergency Ambulance Fleet</div>
            <div class="overlay-sub">Plot no.223, K-1 Kalinga Nagar, Bhubaneswar</div>
          </div>
          <div class="ambulance-tag">24/7 ON CALL</div>
        </div>
      </div>
    </div>

    <!-- Bottom Bar -->
    <div class="bottom-bar">
      <div class="bottom-left">
        <span>Emergency Helpline: <strong>+91 8249914673</strong></span>
        <span>•</span>
        <span>OPD Timings: <strong>Mon–Sat 9:00 AM – 8:00 PM</strong></span>
      </div>
      <div class="call-btn">
        <span>📞 Call +91 8249914673</span>
      </div>
    </div>
  </div>
</body>
</html>`;

async function main() {
  const browser = await chromium.launch({
    executablePath: CHROME_PATH,
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  const page = await browser.newPage({ viewport: { width: 1200, height: 900 } });
  await page.setContent(htmlContent, { waitUntil: 'load' });
  await page.waitForTimeout(1200);

  await page.screenshot({
    path: OUT_PATH,
    type: 'jpeg',
    quality: 96
  });

  console.log('Saved brand-aware hero banner to:', OUT_PATH);
  await browser.close();
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
