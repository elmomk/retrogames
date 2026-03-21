#!/bin/bash
set -e
cd "$(dirname "$0")/.."

BASE_URL="${1:-http://localhost:8090}"
GAMES="micro space shadow arena dragon mariolike nova cyber neon"

mkdir -p /tmp/retrogames-test

echo "==> Testing against $BASE_URL"
echo ""

NODE_PATH=/home/mo/.npm/_npx/e41f203b7505f1fb/node_modules node -e "
const { chromium } = require('playwright');
(async () => {
  const browser = await chromium.launch();
  const baseUrl = '$BASE_URL';
  const games = '$GAMES'.split(' ');
  const results = [];

  for (const target of ['launcher', ...games]) {
    const url = target === 'launcher' ? baseUrl + '/' : baseUrl + '/' + target + '/index.html';
    const pg = await browser.newPage();
    const errs = [];
    pg.on('pageerror', e => errs.push(e.message));
    try {
      await pg.goto(url, { timeout: 10000, waitUntil: 'domcontentloaded' });
      await pg.waitForTimeout(1500);
      await pg.screenshot({ path: '/tmp/retrogames-test/' + target + '.png' });
      if (target === 'launcher') {
        const cards = await pg.locator('.game-card').count();
        results.push([target, 'OK', cards + ' cards', errs]);
      } else {
        const canvases = await pg.locator('canvas').count();
        results.push([target, 'OK', canvases + ' canvas', errs]);
      }
    } catch(e) {
      results.push([target, 'FAIL', e.message.split('\n')[0].substring(0,40), errs]);
    }
    await pg.close();
  }

  console.log('PAGE'.padEnd(14) + 'STATUS'.padEnd(8) + 'DETAIL'.padEnd(16) + 'JS ERRORS');
  console.log('-'.repeat(60));
  for (const [name, status, detail, errs] of results) {
    console.log(name.padEnd(14) + status.padEnd(8) + String(detail).substring(0,14).padEnd(16) + (errs.length ? errs[0].substring(0,30) : 'none'));
  }
  await browser.close();
})();
"

echo ""
echo "Screenshots saved to /tmp/retrogames-test/"
