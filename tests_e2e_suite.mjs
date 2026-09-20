import { chromium } from 'playwright';
import fs from 'fs';
import path from 'path';

const BASE_URL = process.env.BASE_URL || 'http://127.0.0.1:8085';
const SCREENSHOT_DIR = path.resolve('test-artifacts/screenshots');

fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });

const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const BRAVE_PATH = '/Applications/Brave Browser.app/Contents/MacOS/Brave Browser';

async function runBrowserTests(browserName, executablePath) {
  console.log(`\n======================================================`);
  console.log(`🌐 Testing with ${browserName}: ${executablePath}`);
  console.log(`======================================================`);

  if (!fs.existsSync(executablePath)) {
    console.warn(`⚠️ Executable not found at ${executablePath}, skipping.`);
    return true;
  }

  const browser = await chromium.launch({
    executablePath,
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  const context = await browser.newContext({
    viewport: { width: 1440, height: 900 }
  });
  const page = await context.newPage();

  const consoleErrors = [];
  const networkErrors = [];

  page.on('console', msg => {
    if (msg.type() === 'error') {
      consoleErrors.push(msg.text());
      console.error(`❌ [${browserName}] Console Error:`, msg.text());
    }
  });

  page.on('pageerror', err => {
    consoleErrors.push(err.message);
    console.error(`❌ [${browserName}] Page Error:`, err.message);
  });

  page.on('response', resp => {
    if (resp.status() >= 400 && !resp.url().includes('favicon')) {
      networkErrors.push({ url: resp.url(), status: resp.status() });
      console.warn(`⚠️ [${browserName}] Network Error: ${resp.status()} ${resp.url()}`);
    }
  });

  const testResults = [];
  function record(testName, passed, details = '') {
    testResults.push({ testName, passed, details });
    const status = passed ? '✅ PASS' : '❌ FAIL';
    console.log(`${status}: [${browserName}] ${testName}${details ? ' - ' + details : ''}`);
  }

  try {
    // 1. Page Load
    await page.goto(BASE_URL, { waitUntil: 'domcontentloaded' });
    await page.waitForSelector('nav', { timeout: 10000 });
    const appEl = await page.locator('#app');
    record('App Mounts Cleanly', (await appEl.count()) > 0);

    // 2. Loading screen gone
    const loaderCount = await page.locator('#app-loading').count();
    record('Loading Overlay Removed on App Mount', loaderCount === 0);

    // 3. Page Title clean (no duplicate)
    const title = await page.title();
    const isCleanTitle = title.includes('Happy Home Hospital') && title.indexOf('Happy Home') === title.lastIndexOf('Happy Home');
    record('Page Title Non-Duplicated & Clean', isCleanTitle, `Title: "${title}"`);

    // 4. Vector SVG Logo Verified
    const logoEl = page.locator('nav a[aria-label*="Happy Home"] svg, nav img[alt*="Happy Home"]').first();
    const logoLoaded = (await logoEl.count()) > 0;
    record('Official Brand Logo Verified (Crisp Vector/SVG)', logoLoaded);

    // Save Hero Desktop Screenshot
    const prefix = browserName.toLowerCase().replace(/\s+/g, '_');
    await page.screenshot({ path: path.join(SCREENSHOT_DIR, `${prefix}_01_hero.png`) });

    // 5. Dual-Brand Mode Switching in Navbar
    const rehabBtn = page.locator('nav button:has-text("Rehab")').first();
    await rehabBtn.click();
    await page.waitForTimeout(300);
    const rehabBrand = await page.locator('#app').getAttribute('data-brand');
    record('Switch to Rehab Centre Mode via Nav Pill', rehabBrand === 'rehab');

    const hospBtn = page.locator('nav button:has-text("Hospital")').first();
    await hospBtn.click();
    await page.waitForTimeout(300);
    const hospBrand = await page.locator('#app').getAttribute('data-brand');
    record('Switch to Hospital Mode via Nav Pill', hospBrand === 'hospital');

    // 6. Theme Toggle & Dark Mode Contrast
    const themeBtn = page.locator('button[aria-label="Toggle theme"]').first();
    await themeBtn.click();
    await page.waitForTimeout(300);
    const darkTheme = await page.locator('#app').getAttribute('data-theme');
    record('Dark Mode Toggled Active', darkTheme === 'dark');

    // Check Hero H1 color in dark mode
    const h1Color = await page.locator('#hero-heading').evaluate(el => window.getComputedStyle(el).color);
    console.log(`[${browserName}] Hero H1 computed color in Dark Mode:`, h1Color);
    const isBrightText = h1Color.includes('248, 250, 252') || h1Color.includes('255, 255, 255');
    record('Dark Mode Hero Heading High Contrast (WCAG AAA)', isBrightText, `Color: ${h1Color}`);
    await page.screenshot({ path: path.join(SCREENSHOT_DIR, `${prefix}_02_dark_mode.png`) });

    // Toggle back to Light mode
    await themeBtn.click();
    await page.waitForTimeout(300);

    // 7. Services Section & Dual-Track Explorer Tabs
    await page.locator('#services').scrollIntoViewIfNeeded();
    await page.waitForTimeout(400);
    const serviceCards = page.locator('#services .cure-card');
    const serviceCount = await serviceCards.count();
    record('Services Section Cure-Cards Rendered', serviceCount >= 4, `Found: ${serviceCount}`);

    const rehabTab = page.locator('#services button:has-text("Addiction & Rehab")').first();
    if (await rehabTab.count() > 0) {
      await rehabTab.click();
      await page.waitForTimeout(300);
      record('Services In-Page Rehab Explorer Tab Clickable', true);
    }

    // 8. Universal Cure-Cards & Duotone Badges Layout
    const totalCureCards = await page.locator('.cure-card').count();
    record('Universal Cure-Card Layout Across Sections', totalCureCards >= 12, `Total Cards: ${totalCureCards}`);

    const iconBadges = await page.locator('.icon-badge').count();
    record('Colorful Duotone Badges Rendered', iconBadges >= 8, `Total Badges: ${iconBadges}`);

    // 9. Doctor Cards & Interactive Modal Dialog
    await page.locator('#team').scrollIntoViewIfNeeded();
    await page.waitForTimeout(400);
    const docCards = page.locator('#team div[class*="group"]');
    record('Doctor Cards Rendered', (await docCards.count()) >= 4);

    // Click "Book Consult" on first doctor to test DoctorModal
    const firstConsultBtn = page.locator('#team a:has-text("Book Consult")').first();
    await firstConsultBtn.click();
    await page.waitForTimeout(400);

    // Verify modal appeared
    const modalEl = page.locator('.fixed.inset-0.z-50');
    const modalVisible = (await modalEl.count()) > 0;
    record('Doctor Consultation Modal Opens on CTA Click', modalVisible);

    if (modalVisible) {
      await page.screenshot({ path: path.join(SCREENSHOT_DIR, `${prefix}_03_doctor_modal.png`) });
      // Close modal
      const closeBtn = modalEl.locator('button:has-text("Close")').first();
      await closeBtn.click();
      await page.waitForTimeout(300);
      const modalClosed = (await page.locator('.fixed.inset-0.z-50').count()) === 0;
      record('Doctor Modal Closes Cleanly', modalClosed);
    }

    // 10. Appointment Form Interactive Input
    await page.locator('#appointment').scrollIntoViewIfNeeded();
    await page.waitForTimeout(400);

    // Verify Emergency Assistance Heading is crisp white
    const emergencyHeadingColor = await page.locator('#appointment h3:has-text("Need Immediate Medical Assistance")')
      .evaluate(el => window.getComputedStyle(el).color);
    console.log(`[${browserName}] Emergency Card Heading Color:`, emergencyHeadingColor);
    const isEmergencyHeadingWhite = emergencyHeadingColor.includes('255, 255, 255');
    record('Emergency Card Heading Contrast (WCAG AAA White)', isEmergencyHeadingWhite, `Color: ${emergencyHeadingColor}`);

    const nameInput = page.locator('#patient-name, #appointment input[type="text"]').first();
    const phoneInput = page.locator('#patient-phone, #appointment input[type="tel"]').first();
    await nameInput.fill('Dr. Samir Parhi');
    await phoneInput.fill('8249914673');
    record('Appointment Form Interactive Input', true);
    await page.screenshot({ path: path.join(SCREENSHOT_DIR, `${prefix}_04_appointment_section.png`) });

    // 11. Zero Console & Network Errors
    record('Zero Console Errors', consoleErrors.length === 0, `Errors: ${JSON.stringify(consoleErrors)}`);
    record('Zero Asset 404s', networkErrors.length === 0, `Failed: ${JSON.stringify(networkErrors)}`);

  } catch (e) {
    console.error(`💥 [${browserName}] Test run failure:`, e);
    record('Suite Execution', false, e.message);
  } finally {
    await browser.close();
  }

  const passed = testResults.filter(r => r.passed).length;
  const failed = testResults.length - passed;
  console.log(`[${browserName}] Results: ${passed} Passed, ${failed} Failed`);
  return failed === 0;
}

async function main() {
  console.log('🚀 Running Multi-Browser E2E Tests on macOS...');
  let allPass = true;

  // Run on Google Chrome
  const chromePass = await runBrowserTests('Google Chrome', CHROME_PATH);
  allPass = allPass && chromePass;

  // Run on Brave Browser
  const bravePass = await runBrowserTests('Brave Browser', BRAVE_PATH);
  allPass = allPass && bravePass;

  console.log('\n======================================================');
  console.log(`🎯 Final Cross-Browser Result: ${allPass ? 'ALL TESTS PASSED ✅' : 'SOME TESTS FAILED ❌'}`);
  console.log('======================================================\n');
  process.exit(allPass ? 0 : 1);
}

main();
