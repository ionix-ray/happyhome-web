import { chromium } from 'playwright';
import path from 'path';
import fs from 'fs';

const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
// Build-time only: resolve repo root from this file's location (tools/ -> web root).
const ROOT = path.resolve(import.meta.dirname, '..');
const IMG = (n) => path.resolve(ROOT, 'assets/images/' + n);

// Canonical high-res logo (user decision: happy-home-logo-transparent.png, 1536x704)
const logoB64 = fs.readFileSync(IMG('happy-home-logo-transparent.png')).toString('base64');
// Photo bases: current hospital hero photo (hospital+ambulance scene), rehab card photo
const hospPhotoB64 = fs.readFileSync(IMG('hero_hospital_ambulance.jpg')).toString('base64');
const rehabPhotoB64 = fs.readFileSync(IMG('card_rehab.jpg')).toString('base64');
const docB64 = fs.readFileSync(IMG('dr_vijay_sahu.jpg')).toString('base64');

const jobs = [
  {
    tpl: 'compose_premium_hero_hospital.html',
    out: IMG('hero_hospital_ambulance.jpg'),
    inject: { __PHOTO_B64__: hospPhotoB64, __LOGO_B64__: logoB64 },
  },
  {
    tpl: 'compose_premium_hero_rehab.html',
    out: IMG('hero_banner.jpg'),
    inject: { __PHOTO_B64__: rehabPhotoB64, __LOGO_B64__: logoB64, __DOC_B64__: docB64 },
  },
];

async function main() {
  const browser = await chromium.launch({
    executablePath: CHROME_PATH,
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox', '--force-device-scale-factor=1'],
  });
  for (const job of jobs) {
    let html = fs.readFileSync(path.resolve(ROOT, job.tpl), 'utf8');
    for (const [k, v] of Object.entries(job.inject)) html = html.split(k).join(v);
    const page = await browser.newPage({ viewport: { width: 1920, height: 640 } });
    await page.setContent(html, { waitUntil: 'load' });
    await page.waitForTimeout(1200);
    await page.screenshot({ path: job.out, type: 'jpeg', quality: 92 });
    console.log('saved', job.out);
    await page.close();
  }
  await browser.close();
}

main().catch((err) => { console.error(err); process.exit(1); });
