import { chromium } from 'playwright';
import path from 'path';
import fs from 'fs';

const OUT_PATH = path.resolve('assets/images/hero_hospital_ambulance.jpg');
const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

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

    /* Background with architectural hospital atmosphere and subtle glow */
    .bg-layer {
      position: absolute;
      inset: 0;
      background: radial-gradient(circle at 80% 20%, rgba(2, 132, 199, 0.35), transparent 50%),
                  radial-gradient(circle at 10% 80%, rgba(45, 143, 123, 0.25), transparent 45%),
                  linear-gradient(135deg, #091428 0%, #0d213f 50%, #091428 100%);
    }

    /* Architectural grid lines */
    .grid-pattern {
      position: absolute;
      inset: 0;
      background-image: linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
                        linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
      background-size: 40px 40px;
    }

    .container {
      position: relative;
      z-index: 10;
      width: 100%;
      height: 100%;
      padding: 40px 50px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
    }

    /* Top Brand Bar */
    .top-bar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      background: rgba(255, 255, 255, 0.96);
      padding: 16px 32px;
      border-radius: 20px;
      box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    }
    .logo-img {
      height: 64px;
      object-fit: contain;
    }
    .top-badges {
      display: flex;
      align-items: center;
      gap: 16px;
    }
    .badge-pill {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 8px 16px;
      border-radius: 9999px;
      font-size: 13px;
      font-weight: 700;
    }
    .badge-emergency {
      background: #fee2e2;
      color: #dc2626;
      border: 1px solid #fca5a5;
    }
    .badge-nabh {
      background: #ecfdf5;
      color: #059669;
      border: 1px solid #a7f3d0;
    }

    /* Main Content Split */
    .main-grid {
      display: grid;
      grid-template-columns: 1fr 1.05fr;
      gap: 40px;
      align-items: center;
      flex: 1;
      padding: 20px 0;
    }

    .left-col {
      display: flex;
      flex-direction: column;
      gap: 20px;
    }
    .eyebrow {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      color: #38bdf8;
      font-size: 14px;
      font-weight: 800;
      letter-spacing: 2px;
      text-transform: uppercase;
    }
    .headline {
      font-size: 44px;
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
      font-size: 16px;
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
      font-size: 15px;
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
    }

    /* Right visual card showcasing modern emergency response & ambulance */
    .visual-card {
      position: relative;
      border-radius: 28px;
      overflow: hidden;
      box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.6);
      border: 2px solid rgba(56, 189, 248, 0.3);
      height: 480px;
      background: #0f2744;
    }
    .visual-card img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }
    .card-overlay-badge {
      position: absolute;
      bottom: 24px;
      left: 24px;
      right: 24px;
      background: rgba(9, 20, 40, 0.88);
      backdrop-filter: blur(16px);
      padding: 16px 20px;
      border-radius: 18px;
      border: 1px solid rgba(255, 255, 255, 0.15);
      display: flex;
      align-items: center;
      justify-content: space-between;
    }
    .overlay-title {
      font-size: 16px;
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
      background: rgba(15, 23, 42, 0.75);
      border: 1px solid rgba(255, 255, 255, 0.1);
      backdrop-filter: blur(12px);
      padding: 14px 28px;
      border-radius: 16px;
    }
    .bottom-left {
      display: flex;
      align-items: center;
      gap: 24px;
      font-size: 14px;
      color: #94a3b8;
    }
    .bottom-left strong {
      color: #f8fafc;
    }
    .bottom-cta {
      display: flex;
      align-items: center;
      gap: 12px;
    }
    .call-btn {
      background: linear-gradient(135deg, #0284c7, #0ea5e9);
      color: #ffffff;
      padding: 10px 22px;
      border-radius: 12px;
      font-weight: 800;
      font-size: 14px;
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
    <!-- Top Bar with Official Logo -->
    <div class="top-bar">
      <img src="file://${path.resolve('assets/images/happy-home-logo-transparent-v2.png')}" class="logo-img" alt="Happy Home Hospital">
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
          Happy Home Hospital provides 24/7 emergency stabilization, advanced neuro ICU telemetry, and rapid cardiac life support ambulances across Bhubaneswar.
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
        <img src="file://${path.resolve('assets/images/card_corridor.jpg')}" alt="Happy Home Hospital Facility">
        <div class="card-overlay-badge">
          <div>
            <div class="overlay-title">Happy Home Emergency Fleet</div>
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
      <div class="bottom-cta">
        <div class="call-btn">
          <span>📞 Call 8249914673</span>
        </div>
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
  await page.setContent(htmlContent, { waitUntil: 'networkidle' });
  await page.waitForTimeout(1000);

  await page.screenshot({
    path: OUT_PATH,
    type: 'jpeg',
    quality: 95
  });

  console.log('Saved banner to:', OUT_PATH);
  await browser.close();
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
