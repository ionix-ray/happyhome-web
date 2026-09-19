import { chromium } from 'playwright';

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  page.on('console', msg => console.log('BROWSER CONSOLE:', msg.text()));
  page.on('pageerror', err => console.log('BROWSER ERROR:', err.message));

  console.log("Navigating to http://localhost:59808 ...");
  await page.goto('http://localhost:59808', { waitUntil: 'networkidle' });

  // Wait for the app to mount. The nav element is rendered by Dioxus.
  try {
    await page.waitForSelector('nav', { timeout: 10000 });
    console.log('App loaded successfully and mounted Dioxus components!');
    const content = await page.content();
    if (content.includes('Happy Home Hospital')) {
      console.log('Content verified.');
    }
  } catch (e) {
    console.log('Timeout waiting for nav:', e.message);
  }

  await browser.close();
})();
