import { chromium } from 'playwright';
import path from 'path';

const BASE_URL = process.env.BASE_URL || 'http://127.0.0.1:8089';
const SCREENSHOT_DIR = path.resolve('test-artifacts/screenshots');
const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

async function capture() {
  const browser = await chromium.launch({
    executablePath: CHROME_PATH,
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  await page.goto(BASE_URL, { waitUntil: 'networkidle' });
  await page.waitForSelector('nav', { timeout: 10000 });

  // 1. Services section with dual-track explorer
  await page.locator('#services').scrollIntoViewIfNeeded();
  await page.waitForTimeout(500);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '05_services_dual_track.png') });

  // 2. Why us section with cure-cards & badges
  await page.locator('#why-us').scrollIntoViewIfNeeded();
  await page.waitForTimeout(500);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '06_why_us_cards.png') });

  // 3. Departments section
  await page.locator('#departments').scrollIntoViewIfNeeded();
  await page.waitForTimeout(500);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '07_departments_cards.png') });

  // 4. Contact section
  await page.locator('#contact').scrollIntoViewIfNeeded();
  await page.waitForTimeout(500);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '08_contact_section.png') });

  await browser.close();
  console.log('✅ Captured detailed section screenshots!');
}

capture();
