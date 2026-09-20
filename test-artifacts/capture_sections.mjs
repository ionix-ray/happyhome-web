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

  // Desktop 1440
  const contextDesktop = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await contextDesktop.newPage();
  await page.goto(BASE_URL, { waitUntil: 'networkidle' });
  await page.waitForSelector('nav', { timeout: 10000 });

  // 0. Hero with Single-line TopBar, Bio-canvas, and dynamic counters
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '01_hero_desktop_v2.png') });

  // 0.5 About section with Dr. Vijay Sahu's branded embroidered coat
  await page.locator('#about').scrollIntoViewIfNeeded();
  await page.waitForTimeout(600);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '03_about_doctor_branding.png') });

  // 1. Services section with upper AI image half + lower text half + wave divider
  await page.locator('#services').scrollIntoViewIfNeeded();
  await page.waitForTimeout(600);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '05_services_split_cards.png') });

  // 2. Why us section
  await page.locator('#why-us').scrollIntoViewIfNeeded();
  await page.waitForTimeout(600);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '06_why_us_split_cards.png') });

  // 3. Departments section
  await page.locator('#departments').scrollIntoViewIfNeeded();
  await page.waitForTimeout(600);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '07_departments_split_cards.png') });

  // 4. Dark Mode Hero & Wave Transitions
  const themeBtn = page.locator('button[aria-label="Toggle theme"]').first();
  await themeBtn.click();
  await page.waitForTimeout(400);
  await page.locator('#home').scrollIntoViewIfNeeded();
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '02_dark_mode_hero_bio.png') });
  await themeBtn.click();
  await page.waitForTimeout(400);


  // Mobile 375
  const contextMobile = await browser.newContext({ viewport: { width: 375, height: 812 } });
  const pageMobile = await contextMobile.newPage();
  await pageMobile.goto(BASE_URL, { waitUntil: 'networkidle' });
  await pageMobile.waitForSelector('nav', { timeout: 10000 });

  // Mobile Hero with ticker
  await pageMobile.screenshot({ path: path.join(SCREENSHOT_DIR, '09_mobile_hero_ticker.png') });

  // Open mobile drawer
  const burger = pageMobile.locator('nav button[aria-label="Toggle mobile navigation"]');
  await burger.click();
  await pageMobile.waitForTimeout(500);
  await pageMobile.screenshot({ path: path.join(SCREENSHOT_DIR, '10_mobile_drawer_open.png') });

  await browser.close();
  console.log('✅ Captured multi-viewport screenshots!');
}

capture();
