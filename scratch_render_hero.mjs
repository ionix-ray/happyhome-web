import { chromium } from 'playwright';
import path from 'path';

const CHROME_PATH = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const HTML_PATH = 'file://' + path.resolve('compose_hero.html');
const OUT_PATH = '/Users/samirparhi-dev/.gemini/antigravity-ide/brain/9a8976b0-cec9-4893-b523-4282c37b1a3f/test_rendered_hero.jpg';

async function main() {
  const browser = await chromium.launch({
    executablePath: CHROME_PATH,
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  const page = await browser.newPage({ viewport: { width: 1200, height: 896 }, deviceScaleFactor: 1 });
  await page.goto(HTML_PATH, { waitUntil: 'load' });
  await page.waitForTimeout(1000);

  await page.screenshot({
    path: OUT_PATH,
    type: 'jpeg',
    quality: 95
  });

  console.log('Rendered test_rendered_hero.jpg successfully');
  await browser.close();
}

main().catch(console.error);
