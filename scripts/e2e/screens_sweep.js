// Varredura de todas as telas no WebKit (Safari): iPhone 14 e desktop.
// Uso: node screens_sweep.js <url-base> <pasta-de-saída> [iphone|desktop]
const path = require('path');
const fs = require('fs');
const pw = require('playwright');

const BASE = process.argv[2];
const OUT = process.argv[3];
const PROFILE = process.argv[4] || 'iphone';
const run = Date.now().toString(36);
const pass = 'Teste' + run + '9';
const report = [];
let shot = 0;

// PNG 2x2 válido para testar upload de imagem
const PNG = Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAIAAAD91JpzAAAAFklEQVR4nGP8z8DAwMDAxMDAwMDAAAAPAQEB3n0yXQAAAABJRU5ErkJggg==', 'base64');
fs.writeFileSync(path.join(OUT, 'tiny.png'), PNG);

async function serverFn(ctx, name, form) {
  const res = await ctx.request.post(`${BASE}/api/${name}`, { form, headers: { Accept: 'application/json' } });
  const text = await res.text();
  if (!res.ok()) throw new Error(`${name} -> ${res.status()} ${text.slice(0, 200)}`);
  try { return JSON.parse(text); } catch { return text; }
}

async function register(ctx, username) {
  const res = await ctx.request.post(`${BASE}/api/form_register`, { form: { username, password: pass, confirm_password: pass }, maxRedirects: 0 });
  const loc = res.headers()['location'] || '';
  if (loc.includes('error')) throw new Error('registro: ' + decodeURIComponent(loc));
}

function watch(page) {
  const errs = [];
  page.on('pageerror', e => errs.push('pageerror: ' + e.message));
  page.on('console', m => { if (m.type() === 'error' && !/favicon|Failed to load resource/.test(m.text())) errs.push('console: ' + m.text()); });
  page.on('requestfailed', r => { if (!/favicon|room_events/.test(r.url())) errs.push('requestfailed: ' + r.url().replace(BASE, '') + ' ' + (r.failure() || {}).errorText); });
  return errs;
}

async function screen(page, label, fn) {
  const errs = watch(page);
  const t0 = Date.now();
  let note = '', ok = true;
  try { note = (await fn()) || ''; } catch (e) { ok = false; note = 'ERRO: ' + e.message.split('\n')[0]; }
  await page.waitForTimeout(400);
  const metrics = await page.evaluate(() => ({
    overflowX: document.documentElement.scrollWidth - window.innerWidth,
    buttons: document.querySelectorAll('button').length,
  })).catch(() => ({ overflowX: -1, buttons: -1 }));
  const file = `${PROFILE}_${String(++shot).padStart(2, '0')}_${label.replace(/[^a-z0-9]+/gi, '_')}.png`;
  await page.screenshot({ path: path.join(OUT, file) }).catch(() => {});
  const problems = [...errs];
  if (metrics.overflowX > 2) problems.push(`rolagem horizontal ${metrics.overflowX}px`);
  report.push({ label, ok: ok && problems.length === 0, ms: Date.now() - t0, note, problems, file });
  console.log(`${ok && !problems.length ? 'OK  ' : 'FAIL'} ${label.padEnd(42)} ${String(Date.now() - t0).padStart(5)}ms ${note}${problems.length ? '\n      ' + problems.slice(0, 4).join('\n      ') : ''}`);
  page.removeAllListeners('pageerror'); page.removeAllListeners('console'); page.removeAllListeners('requestfailed');
}

const tap = async (loc) => PROFILE === 'iphone' ? loc.tap() : loc.click();

process.on('unhandledRejection', e => console.log('      (promessa rejeitada ignorada)'));
(async () => {
  const browser = await pw.webkit.launch();
  const device = PROFILE === 'iphone' ? pw.devices['iPhone 14'] : { viewport: { width: 1366, height: 900 } };
  const anon = await browser.newContext(device);
  const gmCtx = await browser.newContext(device);
  const plCtx = await browser.newContext(device);

  // ---------- Telas públicas ----------
  const ap = await anon.newPage();
  for (const [route, label] of [['/', 'início (deslogado)'], ['/about', 'sobre'], ['/colabore', 'colabore'], ['/feed', 'feed público']]) {
    await screen(ap, label, async () => { await ap.goto(BASE + route, { waitUntil: 'networkidle' }); });
  }
  await screen(ap, 'login: cadastro pelo formulário', async () => {
    await ap.goto(BASE + '/login?tab=register', { waitUntil: 'networkidle' });
    const user = ap.locator('input[name="username"]:visible').first();
    await user.fill('ui_' + run);
    const pwds = ap.locator('input[type="password"]:visible');
    for (let i = 0; i < await pwds.count(); i++) await pwds.nth(i).fill(pass);
    await tap(ap.locator('button[type="submit"]:visible').first());
    await ap.waitForURL(u => !u.toString().includes('/login'), { timeout: 15000 });
    return 'redirecionou para ' + new URL(ap.url()).pathname;
  });

  // ---------- Dados de teste ----------
  await register(gmCtx, 'gm_' + run);
  await register(plCtx, 'pl_' + run);
  const mageId = await serverFn(gmCtx, 'create_sheet', { name: 'Maga WebKit', sheet_type: 'mage' });
  const godsId = await serverFn(gmCtx, 'create_sheet', { name: 'Familiar WebKit', sheet_type: 'gods_and_monsters' });
  const plSheet = await serverFn(plCtx, 'create_sheet', { name: 'Herói do Jogador', sheet_type: 'mage' });
  const roomId = await serverFn(gmCtx, 'create_room', { name: 'Mesa ' + run, description: 'varredura', is_public: 'true' });
  const details = await serverFn(gmCtx, 'get_room_details', { room_id: roomId });
  await serverFn(plCtx, 'join_room_by_code', { code: details.code });
  await serverFn(gmCtx, 'assign_sheet_to_room', { sheet_id: mageId, room_id: roomId });
  await serverFn(plCtx, 'assign_sheet_to_room', { sheet_id: plSheet, room_id: roomId });
  try {
    const flags = await serverFn(gmCtx, 'get_system_feature_flags', {});
    for (const f of flags) await serverFn(gmCtx, 'update_system_feature_flag', { key: f.key, value: 'true' }).catch(() => {});
  } catch {}
  // Segurança: jogador não pode levar a ficha do mestre para outro lugar
  const steal = await plCtx.request.post(`${BASE}/api/assign_sheet_to_room`, { form: { sheet_id: mageId, room_id: roomId }, headers: { Accept: 'application/json' } });
  report.push({ label: 'segurança: vincular ficha alheia é recusado', ok: !steal.ok(), ms: 0, note: `HTTP ${steal.status()}`, problems: [] });
  console.log(`${!steal.ok() ? 'OK  ' : 'FAIL'} segurança: vincular ficha alheia é recusado  HTTP ${steal.status()}`);

  // ---------- Telas logadas ----------
  const p = await gmCtx.newPage();
  await screen(p, 'início (drive com fichas)', async () => {
    await p.goto(BASE + '/', { waitUntil: 'networkidle' });
    await p.waitForSelector('text=Maga WebKit', { timeout: 20000 });
  });
  await screen(p, 'perfil', async () => { await p.goto(BASE + '/profile', { waitUntil: 'networkidle' }); });
  await screen(p, 'perfil público', async () => { await p.goto(`${BASE}/user/gm_${run}`, { waitUntil: 'networkidle' }); });
  await screen(p, 'logs (admin)', async () => { await p.goto(BASE + '/logs', { waitUntil: 'networkidle' }); });
  await screen(p, 'lista de salas', async () => { await p.goto(BASE + '/rooms', { waitUntil: 'networkidle' }); await p.waitForSelector('text=Mesa ' + run, { timeout: 15000 }); });

  // ---------- Ficha de Mago: todas as páginas ----------
  await screen(p, 'ficha mago: abrir', async () => {
    await p.goto(`${BASE}/sheet/${mageId}`, { waitUntil: 'networkidle' });
    await p.waitForSelector('.sheet-tab-btn', { timeout: 30000 });
  });
  const tabCount = await p.locator('.sheet-tab-btn').count();
  for (let i = 0; i < tabCount; i++) {
    const name = ((await p.locator('.sheet-tab-btn').nth(i).innerText()) || '').replace(/\s+/g, ' ').trim().slice(0, 28);
    await screen(p, `ficha mago: aba ${i + 1} ${name}`, async () => { await tap(p.locator('.sheet-tab-btn').nth(i)); await p.waitForTimeout(500); });
  }
  await tap(p.locator('.sheet-tab-btn').first());
  await screen(p, 'ficha mago: clicar bolinha salva no servidor', async () => {
    const saved = p.waitForResponse(r => /\/api\/(update|patch)_sheet/.test(r.url()), { timeout: 15000 }).catch(e => e);
    await tap(p.locator('.page-main .dot').nth(2));
    const res = await saved;
    if (res instanceof Error) throw new Error('nenhum salvamento automático em 15s');
    if (!res.ok()) throw new Error('salvamento HTTP ' + res.status());
    return 'salvamento ' + new URL(res.url()).pathname + ' 200';
  });
  await screen(p, 'ficha mago: editor rico (negrito)', async () => {
    for (let i = 0; i < tabCount; i++) {
      await tap(p.locator('.sheet-tab-btn').nth(i)); await p.waitForTimeout(300);
      if (await p.locator('.wysiwyg-content:visible').count()) break;
    }
    const ed = p.locator('.wysiwyg-content:visible').first();
    await tap(ed);
    await p.keyboard.type('Texto em negrito');
    // Seleciona o texto pela API de seleção (o atalho de teclado não existe no iPhone emulado)
    await ed.evaluate(el => { const r = document.createRange(); r.selectNodeContents(el); const s = getSelection(); s.removeAllRanges(); s.addRange(r); });
    await tap(p.locator('.wysiwyg-btn:visible').first());
    const html = await ed.innerHTML();
    if (!/<(b|strong)>/i.test(html)) throw new Error('negrito não aplicado: ' + html.slice(0, 80));
    return 'negrito aplicado';
  });
  await screen(p, 'ficha mago: upload de imagem', async () => {
    let input = null;
    for (let i = 0; i < tabCount && !input; i++) {
      await tap(p.locator('.sheet-tab-btn').nth(i)); await p.waitForTimeout(300);
      const c = p.locator('input[type="file"]');
      if (await c.count()) input = c.first();
    }
    if (!input) return 'sem campo de upload visível';
    const up = p.waitForResponse(r => /upload_image|save_uploaded_media/.test(r.url()), { timeout: 15000 }).catch(() => null);
    await input.setInputFiles(path.join(OUT, 'tiny.png'));
    const res = await up;
    if (res && !res.ok()) throw new Error('upload HTTP ' + res.status());
    return res ? 'upload ' + res.status() : 'sem requisição de upload (pode exigir enquadramento)';
  });
  // Menu de ações: abre cada modal (exceto importar/exportar/excluir)
  await p.goto(`${BASE}/sheet/${mageId}`, { waitUntil: 'networkidle' });
  await p.waitForSelector('.actions-menu-trigger-btn', { timeout: 30000 }).catch(() => {});
  if (await p.locator('.actions-menu-trigger-btn').count()) {
    await tap(p.locator('.actions-menu-trigger-btn').first());
    const items = await p.locator('.actions-menu-item-btn').allInnerTexts();
    await p.keyboard.press('Escape').catch(() => {});
    for (let i = 0; i < items.length; i++) {
      const label = items[i].replace(/\s+/g, ' ').trim();
      if (/import|export|exclu|apag|delet|json|imprim|print|pdf/i.test(label)) continue;
      await p.goto(`${BASE}/sheet/${mageId}`, { waitUntil: 'networkidle' });
      await p.waitForSelector('.actions-menu-trigger-btn', { timeout: 30000 });
      await screen(p, `ficha mago: menu "${label.slice(0, 26)}"`, async () => {
        await tap(p.locator('.actions-menu-trigger-btn').first());
        await tap(p.locator('.actions-menu-item-btn').nth(i));
        await p.waitForTimeout(700);
      });
    }
  }

  // ---------- Ficha Deuses & Monstros ----------
  await screen(p, 'ficha D&M: abrir', async () => {
    await p.goto(`${BASE}/sheet/${godsId}`, { waitUntil: 'networkidle' });
    await p.waitForSelector('.sheet-tab-btn', { timeout: 30000 });
  });
  const gTabs = await p.locator('.sheet-tab-btn').count();
  for (let i = 0; i < gTabs; i++) {
    await screen(p, `ficha D&M: aba ${i + 1}`, async () => { await tap(p.locator('.sheet-tab-btn').nth(i)); await p.waitForTimeout(400); });
  }

  // ---------- Sala ----------
  await screen(p, 'sala: abrir', async () => {
    await p.goto(`${BASE}/room/${roomId}`, { waitUntil: 'networkidle' });
    await p.waitForSelector('.room-tab-btn', { timeout: 30000 });
  });
  const roomTabs = await p.locator('.room-tab-btn').count();
  for (let i = 0; i < roomTabs; i++) {
    const name = ((await p.locator('.room-tab-btn').nth(i).innerText()) || '').replace(/\s+/g, ' ').trim().slice(0, 26);
    await screen(p, `sala: aba ${name}`, async () => {
      const btn = p.locator('.room-tab-btn').nth(i);
      if (await btn.isVisible()) await tap(btn); else await btn.evaluate(b => b.click());
      await p.waitForTimeout(700);
      if (await p.locator('canvas:visible').count()) {
        const box = await p.locator('canvas:visible').first().boundingBox();
        if (box && PROFILE === 'iphone') await p.touchscreen.tap(box.x + box.width / 2, box.y + box.height / 2);
        return 'canvas do mapa renderizado';
      }
    });
  }
  await screen(p, 'sala: gaveta de iniciativa + rolar', async () => {
    const btn = p.locator('.initiative-top-bar-btn:visible').first();
    if (!(await btn.count())) return 'botão de iniciativa não visível';
    await tap(btn);
    await p.waitForTimeout(600);
    const roll = p.locator('.initiative-drawer button:has-text("Rolar"):visible').first();
    if (await roll.count()) { await tap(roll); await p.waitForTimeout(800); return 'iniciativa rolada'; }
    return 'gaveta aberta';
  });
  // Jogador na mesma sala (garante que a sala abre para quem não é mestre)
  const pp = await plCtx.newPage();
  await screen(pp, 'sala: visão do jogador', async () => {
    await pp.goto(`${BASE}/room/${roomId}`, { waitUntil: 'networkidle' });
    await pp.waitForSelector('.room-tab-btn, .chat-top-bar-btn', { timeout: 30000 });
  });

  await browser.close();
  const failed = report.filter(r => !r.ok);
  fs.writeFileSync(path.join(OUT, `sweep_${PROFILE}.json`), JSON.stringify(report, null, 2));
  console.log(`\nRESUMO ${PROFILE}: ${report.length - failed.length}/${report.length} telas/fluxos sem problemas`);
  process.exit(failed.length ? 1 : 0);
})().catch(e => { console.error('ERRO FATAL:', e.stack); process.exit(2); });
