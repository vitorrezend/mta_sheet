// Teste E2E do chat da sala no motor WebKit (Safari) com Playwright.
// Mestre: WebKit desktop. Jogador: WebKit emulando iPhone 14.
// Uso: node room_chat.js <url-base> <pasta-de-saída> [webkit|chromium]
const path = require('path');
const pw = require('playwright');

const { devices } = pw;
const ENGINE = process.argv[4] || 'webkit';
const BASE = process.argv[2] || 'http://127.0.0.1:3005';
const OUT = process.argv[3] || '.';
const run = Date.now().toString(36);
const pass = 'Teste' + run + '9';
const results = [];
const check = (name, ok, detail = '') => { results.push({ name, ok, detail }); console.log(`${ok ? 'PASS' : 'FAIL'} ${name}${detail ? ' — ' + detail : ''}`); };

// Conta quantas notas de som foram tocadas (OscillatorNode.start) e o estado do AudioContext
const audioProbe = () => {
  window.__osc = 0;
  window.__hasAudio = typeof (window.AudioContext || window.webkitAudioContext) === 'function';
  if (!window.OscillatorNode) return;
  const orig = OscillatorNode.prototype.start;
  OscillatorNode.prototype.start = function (...a) { window.__osc++; return orig.apply(this, a); };
};

async function serverFn(ctx, name, form) {
  const res = await ctx.request.post(`${BASE}/api/${name}`, { form, headers: { Accept: 'application/json' } });
  const text = await res.text();
  if (!res.ok()) throw new Error(`${name} -> ${res.status()} ${text}`);
  try { return JSON.parse(text); } catch { return text; }
}

async function register(ctx, username) {
  const res = await ctx.request.post(`${BASE}/api/form_register`, {
    form: { username, password: pass, confirm_password: pass }, maxRedirects: 0,
  });
  const loc = res.headers()['location'] || '';
  if (loc.includes('error')) throw new Error('registro falhou: ' + decodeURIComponent(loc));
}

function collectErrors(page, label, bucket) {
  page.on('pageerror', e => bucket.push(`[${label}] pageerror: ${e.message}`));
  page.on('console', m => { if (m.type() === 'error') bucket.push(`[${label}] console: ${m.text()}`); });
}

(async () => {
  const browser = await pw[ENGINE].launch(ENGINE === 'chromium' ? { channel: 'msedge' } : {});
  console.log('Motor:', ENGINE);
  const errors = [];
  const gmCtx = await browser.newContext({ viewport: { width: 1280, height: 860 } });
  const plCtx = await browser.newContext({ ...devices['iPhone 14'] });
  for (const c of [gmCtx, plCtx]) await c.addInitScript(audioProbe);

  await register(gmCtx, 'gm_' + run);
  await register(plCtx, 'pl_' + run);
  const roomId = await serverFn(gmCtx, 'create_room', { name: 'Sala WebKit ' + run, description: 'teste', is_public: 'false' });
  const details = await serverFn(gmCtx, 'get_room_details', { room_id: roomId });
  await serverFn(plCtx, 'join_room_by_code', { code: details.code });
  check('setup: sala criada e jogador entrou', !!roomId);

  const gm = await gmCtx.newPage();
  const pl = await plCtx.newPage();
  collectErrors(gm, 'mestre', errors);
  collectErrors(pl, 'jogador-iphone', errors);

  await gm.goto(`${BASE}/room/${roomId}`);
  await pl.goto(`${BASE}/room/${roomId}`);
  await gm.waitForSelector('.room-tab-btn', { timeout: 60000 });
  await pl.waitForSelector('.chat-top-bar-btn, .room-tab-btn', { timeout: 60000 });

  // Mestre fica na aba de personagens e faz um clique qualquer (gesto que destrava o áudio)
  await gm.locator('.room-tab-btn').first().click();

  // Jogador abre o chat pelo toque
  const plChatBtn = pl.locator('.chat-top-bar-btn:visible, .room-tab-btn:has-text("Chat"):visible').first();
  await plChatBtn.tap();
  await pl.waitForSelector('.chat-text-input', { timeout: 20000 });
  const fontSize = await pl.$eval('.chat-text-input', el => getComputedStyle(el).fontSize);
  check('iPhone: campo do chat com fonte >= 16px (sem zoom ao focar)', parseFloat(fontSize) >= 16, fontSize);

  await pl.locator('.chat-text-input').tap();
  await pl.locator('.chat-text-input').fill('Olá do iPhone ' + run);
  await pl.locator('.chat-send-btn').tap();
  await pl.waitForSelector(`.bubble-text-content:has-text("Olá do iPhone ${run}")`, { timeout: 10000 });
  check('jogador vê a própria mensagem imediatamente', true);

  // Mestre recebe em tempo real, sem recarregar, com som e contador de não lidas
  const badge = gm.locator('.chat-unread-badge').first();
  await badge.waitFor({ timeout: 10000 }).catch(() => {});
  check('mestre: contador de não lidas aparece fora da aba do chat', await badge.isVisible(), await badge.textContent().catch(() => ''));
  const hasAudio = await gm.evaluate(() => window.__hasAudio);
  const gmOsc = await gm.evaluate(() => window.__osc);
  if (hasAudio) check('mestre: som de nova mensagem tocou', gmOsc > 0, `notas=${gmOsc}`);
  else console.log('N/A  som (este build do motor não tem Web Audio)');

  await gm.locator('.room-tab-btn:has-text("Chat")').click();
  await gm.waitForSelector(`.bubble-text-content:has-text("Olá do iPhone ${run}")`, { timeout: 10000 });
  check('mestre: mensagem aparece no histórico', true);
  check('mestre: contador zera ao abrir o chat', !(await gm.locator('.chat-unread-badge').first().isVisible()));
  const time = await gm.locator('.bubble-time').first().textContent();
  check('horário local exibido (formato HH:MM)', /^\d{2}:\d{2}$/.test((time || '').trim()), time);

  // Mestre rola dados pelo rolador rápido; jogador (iPhone) recebe o cartão e o som
  const plOscBefore = await pl.evaluate(() => window.__osc);
  await gm.locator('.chat-quick-roll-toggle-btn').click();
  await gm.locator('.quick-label-input').fill('Percepção + Prontidão');
  await gm.locator('.quick-roll-submit-btn').click();
  await pl.waitForSelector('.roll-result-card:has-text("Percepção + Prontidão")', { timeout: 10000 });
  check('iPhone: rolagem do mestre chega em tempo real', true);
  const dice = await pl.locator('.roll-result-card').last().locator('.die-box').count();
  check('iPhone: cartão mostra 5 dados', dice === 5, `dados=${dice}`);
  await pl.waitForTimeout(500);
  const plOsc = await pl.evaluate(() => window.__osc);
  if (hasAudio) check('jogador: som de dados tocou após o toque inicial', plOsc > plOscBefore, `notas ${plOscBefore} -> ${plOsc}`);

  // Comando /r pelo campo de texto
  await pl.locator('.chat-text-input').fill('/r 3d10 dif 7 esp Esquiva');
  await pl.locator('.chat-send-btn').tap();
  await gm.waitForSelector('.roll-result-card:has-text("Esquiva")', { timeout: 10000 });
  check('comando /r vira rolagem para todos', true);

  // Não pode falar como personagem de outro jogador (validação no servidor)
  const spoof = await plCtx.request.post(`${BASE}/api/send_room_chat_message`, {
    form: { room_id: roomId, content: 'falsificado', sheet_id: 'ficha-inexistente' }, headers: { Accept: 'application/json' } });
  check('servidor recusa personagem que não é do jogador', !spoof.ok(), `HTTP ${spoof.status()}`);

  await pl.screenshot({ path: path.join(OUT, `${ENGINE}_iphone_chat.png`), fullPage: false });
  await gm.screenshot({ path: path.join(OUT, `${ENGINE}_desktop_chat.png`), fullPage: false });

  const relevant = errors.filter(e => !/favicon|404/.test(e));
  check('sem erros de JavaScript no WebKit', relevant.length === 0, relevant.slice(0, 5).join(' | '));

  await browser.close();
  const failed = results.filter(r => !r.ok).length;
  console.log(`\nRESUMO: ${results.length - failed}/${results.length} verificações passaram`);
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error('ERRO FATAL:', e.message); process.exit(2); });
