// E2E dos recursos novos no WebKit: presença, atualização em tempo real sem polling,
// conflito de edição da ficha, rolagem da ficha para a sala e toque nas bolinhas (iPhone).
// Uso: node room_features.js <url-base> <pasta-de-saída>
const path = require('path');
const pw = require('playwright');
const BASE = process.argv[2];
const OUT = process.argv[3];
const run = Date.now().toString(36);
const pass = 'Teste' + run + '9';
const results = [];
const check = (name, ok, detail = '') => { results.push({ name, ok }); console.log(`${ok ? 'PASS' : 'FAIL'} ${name}${detail ? ' — ' + detail : ''}`); };

async function serverFn(ctx, name, form) {
  const res = await ctx.request.post(`${BASE}/api/${name}`, { form, headers: { Accept: 'application/json' } });
  const text = await res.text();
  if (!res.ok()) throw new Error(`${name} -> ${res.status()} ${text.slice(0, 160)}`);
  try { return JSON.parse(text); } catch { return text; }
}
async function register(ctx, username) {
  const res = await ctx.request.post(`${BASE}/api/form_register`, { form: { username, password: pass, confirm_password: pass }, maxRedirects: 0 });
  if ((res.headers()['location'] || '').includes('error')) throw new Error('registro falhou');
}
async function step(name, fn) {
  try { await fn(); } catch (e) { check(name, false, 'ERRO: ' + e.message.split('\n')[0]); }
}

(async () => {
  const browser = await pw.webkit.launch();
  const gmCtx = await browser.newContext({ viewport: { width: 1366, height: 900 } });
  const plCtx = await browser.newContext({ ...pw.devices['iPhone 14'] });
  const gmName = 'gm_' + run, plName = 'pl_' + run;
  await register(gmCtx, gmName);
  await register(plCtx, plName);
  const roomId = await serverFn(gmCtx, 'create_room', { name: 'Mesa ' + run, description: 'e2e', is_public: 'false' });
  const details = await serverFn(gmCtx, 'get_room_details', { room_id: roomId });
  await serverFn(plCtx, 'join_room_by_code', { code: details.code });
  const plSheet = await serverFn(plCtx, 'create_sheet', { name: 'Heroina ' + run, sheet_type: 'mage' });
  await serverFn(plCtx, 'assign_sheet_to_room', { sheet_id: plSheet, room_id: roomId });

  // Mestre abre a sala e conta as recargas completas da sala (o antigo polling de 4s)
  const gm = await gmCtx.newPage();
  let detailCalls = 0;
  gm.on('request', r => { if (r.url().includes('/api/get_room_details')) detailCalls++; });
  await gm.goto(`${BASE}/room/${roomId}`, { waitUntil: 'networkidle' });
  await gm.waitForSelector('.room-tab-btn');

  // 1) Presença
  const pl = await plCtx.newPage();
  await step('presença', async () => {
    await pl.goto(`${BASE}/room/${roomId}`, { waitUntil: 'networkidle' });
    await gm.waitForFunction(n => (document.querySelector('.room-online-strip') || {}).textContent?.includes(n), plName, { timeout: 10000 });
    check('presença: mestre vê o jogador online', true, (await gm.locator('.room-online-strip').textContent()).trim());
    await pl.goto('about:blank');
    await gm.waitForFunction(n => !(document.querySelector('.room-online-strip') || {}).textContent?.includes(n), plName, { timeout: 10000 });
    check('presença: jogador sai da lista ao fechar a sala', true);
  });

  // 2) Ficha do jogador muda -> card na sala do mestre atualiza sem recarregar a sala
  await step('tempo real', async () => {
    detailCalls = 0;
    await pl.goto(`${BASE}/sheet/${plSheet}`, { waitUntil: 'networkidle' });
    await pl.waitForSelector('.info-header input');
    const nameInput = pl.locator('.info-header input').first();
    await nameInput.tap();
    await nameInput.fill('Renomeada ' + run);
    await nameInput.press('Tab');
    const t0 = Date.now();
    await gm.waitForSelector(`text=Renomeada ${run}`, { timeout: 15000 });
    check('tempo real: card do mestre atualiza ao salvar a ficha', true, `${Date.now() - t0}ms após editar (inclui 1,2s do salvamento automático)`);
    check('tempo real: sem recarregar a sala inteira', detailCalls === 0, `get_room_details chamado ${detailCalls}x`);
  });

  // 3) iPhone: tocar numa bolinha preenche e salva
  await step('toque', async () => {
    const saved = pl.waitForResponse(r => /\/api\/(update|patch)_sheet/.test(r.url()), { timeout: 15000 }).catch(e => e);
    const dot = pl.locator('.page-main .dot:visible').nth(1);
    await dot.tap();
    const cls = await dot.getAttribute('class');
    const res = await saved;
    check('iPhone: tocar na bolinha preenche', /filled/.test(cls || ''), cls);
    check('iPhone: tocar na bolinha salva', !(res instanceof Error) && res.ok(), res instanceof Error ? 'sem salvamento' : 'HTTP ' + res.status());
    if (!(res instanceof Error)) {
      const body = res.request().postData() || '';
      const fullSize = (await serverFn(plCtx, 'get_sheet', { id: plSheet }).then(d => JSON.stringify(d).length).catch(() => 0));
      check('salvamento parcial: envia só o que mudou', res.url().includes('/api/patch_sheet') && body.length < 2000,
        `${new URL(res.url()).pathname}: ${body.length} bytes (ficha inteira ~${fullSize} bytes)`);
    }
  });

  // 4) Conflito: mestre altera a ficha pela sala enquanto o jogador está com ela aberta
  await step('conflito', async () => {
    // Mestre ajusta a quintessência pela sala (mesmo caminho dos botões rápidos do card)
    await serverFn(gmCtx, 'update_room_sheet_stats', { room_id: roomId, sheet_id: plSheet, action: 'AddQuintessence' });
    const dot = pl.locator('.page-main .dot:visible').nth(3);
    await dot.tap();
    await pl.waitForSelector('.sheet-conflict-banner', { timeout: 15000 });
    check('conflito: jogador é avisado em vez de sobrescrever', true);
    await pl.screenshot({ path: path.join(OUT, 'conflito_iphone.png') });
    await pl.locator('.conflict-keep-btn').tap();
    await pl.waitForSelector('.sheet-conflict-banner', { state: 'detached', timeout: 15000 });
    check('conflito: "manter minhas alterações" grava e some o aviso', true);
  });

  // 5) Rolar da ficha para a sala
  await step('rolagem da ficha', async () => {
    await gm.locator('.room-tab-btn:has-text("Chat")').click();
    await pl.locator('.sheet-dice-fab').tap();
    await pl.waitForSelector('.sheet-dice-target:has-text("Mesa")', { timeout: 10000 });
    await pl.locator('.sheet-dice-panel select').first().selectOption('Força');
    const shown = (await pl.locator('.sheet-dice-panel select').first().evaluate(s => s.options[s.selectedIndex].text)).trim();
    const btnText = (await pl.locator('.sheet-dice-roll-btn').textContent()).trim();
    const expected = parseInt(shown.match(/\((\d+)\)/)[1], 10);
    check('rolador: botão usa o nível do atributo exibido', btnText.includes(String(expected)), `${shown} -> "${btnText}"`);
    await pl.locator('.sheet-dice-roll-btn').tap();
    await pl.waitForSelector('.sheet-dice-result .roll-result-card', { timeout: 10000 });
    const label = (await pl.locator('.sheet-dice-result .roll-action-label').textContent()).trim();
    check('rolagem da ficha: resultado aparece na ficha com o atributo certo', label.startsWith('Força'), label);
    await gm.waitForSelector('.roll-result-card:has-text("Força")', { timeout: 10000 });
    check('rolagem da ficha: mestre vê no chat da sala', true, (await gm.locator('.roll-result-card').last().locator('.roll-action-label').textContent()).trim());
    await pl.screenshot({ path: path.join(OUT, 'rolador_iphone.png') });
  });

  // 6) Diário da crônica: o mestre salva e o jogador com a aba aberta vê na hora
  await step('diário', async () => {
    await pl.goto(`${BASE}/room/${roomId}`, { waitUntil: 'networkidle' });
    const tab = pl.locator('.room-tab-btn:has-text("Diário")').first();
    if (await tab.isVisible()) await tab.tap(); else await tab.evaluate(b => b.click());
    await pl.waitForSelector('.room-chronicle-section .wysiwyg-content');
    const v1 = await serverFn(gmCtx, 'update_room_chronicle_notes', { room_id: roomId, notes: 'Sessão ' + run, expected_version: '0' });
    await pl.waitForFunction(t => document.querySelector('.room-chronicle-section .wysiwyg-content')?.textContent.includes(t), 'Sessão ' + run, { timeout: 10000 });
    check('diário: jogador recebe o texto do mestre em tempo real', true, `versão ${v1}`);
    const stale = await plCtx.request.post(`${BASE}/api/update_room_chronicle_notes`, {
      form: { room_id: roomId, notes: 'apagaria o do mestre', expected_version: '0' }, headers: { Accept: 'application/json' } });
    check('diário: gravação desatualizada é recusada', !stale.ok(), `HTTP ${stale.status()}`);
  });

  check('mestre: nenhuma recarga completa da sala durante o teste', detailCalls <= 1, `get_room_details: ${detailCalls}`);
  await browser.close();
  const failed = results.filter(r => !r.ok).length;
  console.log(`\nRESUMO: ${results.length - failed}/${results.length} verificações passaram`);
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error('ERRO FATAL:', e.message); process.exit(2); });
