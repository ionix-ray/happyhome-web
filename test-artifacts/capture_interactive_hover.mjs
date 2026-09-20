import { chromium } from 'playwright';
import path from 'path';

const BASE_URL = process.env.BASE_URL || 'http://127.0.0.1:8089';
const SCREENSHOT_DIR = path.resolve('test-artifacts/screenshots');
const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

async function testInteractiveHover() {
  const browser = await chromium.launch({
    executablePath: CHROME_PATH,
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  await page.goto(BASE_URL, { waitUntil: 'networkidle' });
  await page.waitForSelector('nav', { timeout: 10000 });

  // Move mouse near DNA helix and synaptic cluster
  await page.mouse.move(320, 320);
  await page.waitForTimeout(600);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '09_interactive_hover_stimulus.png') });

  // Click to trigger radial neuro-genesis pulse wave and local mutation burst
  await page.mouse.click(320, 320);
  await page.waitForTimeout(300);
  await page.screenshot({ path: path.join(SCREENSHOT_DIR, '10_interactive_click_pulse.png') });

  await browser.close();
  console.log('✅ Captured interactive hover and click excitation screenshots!');
}

testInteractiveHover();
