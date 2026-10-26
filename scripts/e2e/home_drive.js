// Tela inicial e salvamento da ficha no WebKit (Safari): criar ficha, salvar bolinha,
// salvar ao voltar, pastas (criar, renomear, abrir, subpasta, mover, compartilhar, excluir).
// Uso: node home_drive.js <url-base> <pasta-de-saída> [iphone|desktop]
const path = require('path');
const pw = require('playwright');

const BASE = process.argv[2];
const OUT = process.argv[3];
const PROFILE = process.argv[4] || 'desktop';
const run = Date.now().toString(36);
const pass = 'Teste' + run + '9';
const results = [];

async function register(ctx, username) {
  const res = await ctx.request.post(`${BASE}/api/form_register`, { form: { username, password: pass, confirm_password: pass }, maxRedirects: 0 });
  const loc = res.headers()['location'] || '';
  if (loc.includes('error')) throw new Error('registro: ' + decodeURIComponent(loc));
}

async function step(page, label, fn) {
  const errs = [];
  // Recarregar a página cancela o envio de log em andamento; o WebKit relata isso como erro
  const onErr = e => { if (!/record_client_log/.test(e.message)) errs.push('pageerror: ' + e.message); };
  const onConsole = m => { if (m.type() === 'error' && !/favicon|Failed to load resource/.test(m.text())) errs.push('console: ' + m.text()); };
  page.on('pageerror', onErr);
  page.on('console', onConsole);
  let ok = true, note = '';
  try { note = (await fn()) || ''; } catch (e) { ok = false; note = 'ERRO: ' + e.message.split('\n')[0]; }
  page.off('pageerror', onErr);
  page.off('console', onConsole);
  if (errs.length) ok = false;
  await page.screenshot({ path: path.join(OUT, `drive_${PROFILE}_${String(results.length + 1).padStart(2, '0')}.png`) }).catch(() => {});
  results.push(ok);
  console.log(`${ok ? 'OK  ' : 'FAIL'} ${label.padEnd(48)} ${note}${errs.length ? '\n      ' + errs.slice(0, 4).join('\n      ') : ''}`);
}

const tap = async (loc) => PROFILE === 'iphone' ? loc.tap() : loc.click();
const saveRequest = (page) => page.waitForResponse(r => /\/api\/(update|patch)_sheet/.test(r.url()), { timeout: 15000 });
const folderCard = (page, name) => page.locator('.drive-folder-card', { hasText: name });

(async () => {
  const browser = await pw.webkit.launch();
  const device = PROFILE === 'iphone' ? pw.devices['iPhone 14'] : { viewport: { width: 1366, height: 900 } };
  const ctx = await browser.newContext(device);
  await register(ctx, 'drive_' + run);
  const p = await ctx.newPage();
  let sheetUrl = '';

  await step(p, 'sem login não cria nem importa ficha', async () => {
    const anon = await browser.newContext();
    const attempts = {
      create_sheet: { name: 'Anonima', sheet_type: 'mage' },
      import_sheet: { data: JSON.stringify({ id: 'x', name: 'Importada' }) },
    };
    const problems = [];
    const notes = [];
    for (const [fn, form] of Object.entries(attempts)) {
      const res = await anon.request.post(`${BASE}/api/${fn}`, { form, headers: { Accept: 'application/json' } });
      const text = (await res.text()).slice(0, 120);
      if (res.ok()) problems.push(`${fn} aceito: ${text}`);
      // create_sheet decodifica sem dificuldade: a recusa tem de ser por falta de login
      else if (fn === 'create_sheet' && !/login/i.test(text)) problems.push(`${fn} recusado por outro motivo: ${text}`);
      else notes.push(`${fn} ${res.status()}`);
    }
    await anon.close();
    if (problems.length) throw new Error(problems.join(' | '));
    return notes.join(', ');
  });

  await step(p, 'início: drive vazio', async () => {
    await p.goto(BASE + '/', { waitUntil: 'networkidle' });
    await p.waitForSelector('.drive-container', { timeout: 20000 });
  });

  await step(p, 'criar ficha pelo formulário', async () => {
    // No celular o formulário começa recolhido
    if (PROFILE === 'iphone') await tap(p.locator('.mobile-create-toggle-btn'));
    await p.locator('.name-input').fill('Maga do Drive');
    await tap(p.locator('.create-btn'));
    await p.waitForURL(/\/sheet\//, { timeout: 20000 });
    await p.waitForSelector('.page-main .dot', { timeout: 30000 });
    sheetUrl = p.url();
  });

  await step(p, 'bolinha salva sozinha e sobrevive ao recarregar', async () => {
    const before = await p.locator('.page-main .dot.filled').count();
    const saved = saveRequest(p);
    await tap(p.locator('.page-main .dot').nth(2));
    const res = await saved;
    if (!res.ok()) throw new Error('salvamento HTTP ' + res.status());
    await p.reload({ waitUntil: 'networkidle' });
    await p.waitForSelector('.page-main .dot', { timeout: 30000 });
    const after = await p.locator('.page-main .dot.filled').count();
    if (after === before) throw new Error(`bolinhas marcadas: ${before} antes, ${after} depois`);
    return `${before} → ${after} bolinhas marcadas`;
  });

  await step(p, '"Início" salva a edição pendente antes de sair', async () => {
    const before = await p.locator('.page-main .dot.filled').count();
    const saved = saveRequest(p);
    await tap(p.locator('.page-main .dot').nth(7));
    // Volta antes do salvamento automático (1,2 s)
    await tap(p.locator('.back-link').first());
    const res = await saved;
    if (!res.ok()) throw new Error('salvamento HTTP ' + res.status());
    await p.waitForURL(u => new URL(u).pathname === '/', { timeout: 15000 });
    await p.goto(sheetUrl, { waitUntil: 'networkidle' });
    await p.waitForSelector('.page-main .dot', { timeout: 30000 });
    const after = await p.locator('.page-main .dot.filled').count();
    if (after === before) throw new Error('a edição feita antes de sair se perdeu');
    await p.goto(BASE + '/', { waitUntil: 'networkidle' });
    await p.waitForSelector('text=Maga do Drive', { timeout: 20000 });
  });

  await step(p, 'criar pasta', async () => {
    await tap(p.locator('.drive-primary-btn'));
    await p.locator('.folder-input').fill('Pasta E2E');
    await tap(p.locator('.folder-icon-preset-btn').nth(1));
    await tap(p.locator('.folder-modal-card button[type="submit"]'));
    await folderCard(p, 'Pasta E2E').waitFor({ timeout: 15000 });
  });

  await step(p, 'renomear pasta (janela já vem preenchida)', async () => {
    await tap(folderCard(p, 'Pasta E2E').locator('.drive-folder-menu-btn').nth(2));
    const input = p.locator('.folder-input');
    const current = await input.inputValue();
    if (current !== 'Pasta E2E') throw new Error('nome na janela: ' + current);
    await input.fill('Pasta Renomeada');
    await tap(p.locator('.folder-modal-card button[type="submit"]'));
    await folderCard(p, 'Pasta Renomeada').waitFor({ timeout: 15000 });
  });

  await step(p, 'mover ficha para a pasta pelo botão', async () => {
    await tap(p.locator('.character-card', { hasText: 'Maga do Drive' }).locator('.card-move-floating-btn'));
    await tap(p.locator('.move-folder-opt-btn', { hasText: 'Pasta Renomeada' }));
    await p.waitForFunction(() => [...document.querySelectorAll('.drive-folder-count')].some(e => e.textContent.trim().startsWith('1 ')), null, { timeout: 15000 });
    return (await folderCard(p, 'Pasta Renomeada').locator('.drive-folder-count').innerText()).trim();
  });

  await step(p, 'abrir pasta mostra a ficha e a trilha', async () => {
    await tap(folderCard(p, 'Pasta Renomeada').locator('.drive-folder-info'));
    await p.waitForSelector('.drive-breadcrumb-item.active:has-text("Pasta Renomeada")', { timeout: 15000 });
    await p.locator('.character-card', { hasText: 'Maga do Drive' }).waitFor({ timeout: 15000 });
  });

  await step(p, 'criar subpasta dentro da pasta', async () => {
    await tap(p.locator('.drive-primary-btn'));
    await p.locator('.folder-input').fill('Sub E2E');
    await tap(p.locator('.folder-modal-card button[type="submit"]'));
    await folderCard(p, 'Sub E2E').waitFor({ timeout: 15000 });
  });

  await step(p, 'mover pasta não oferece ela mesma nem a subpasta', async () => {
    await tap(p.locator('.drive-breadcrumb-item.clickable').first());
    await folderCard(p, 'Pasta Renomeada').waitFor({ timeout: 15000 });
    await tap(folderCard(p, 'Pasta Renomeada').locator('.drive-folder-menu-btn').nth(0));
    const options = await p.locator('.move-folder-opt-btn').allInnerTexts();
    await tap(p.locator('.folder-modal-card .btn-cancel'));
    if (options.some(o => /Pasta Renomeada|Sub E2E/.test(o))) throw new Error('destinos: ' + options.join(' | '));
    return options.length + ' destino(s)';
  });

  await step(p, 'compartilhar pasta abre a lista de acessos', async () => {
    await tap(folderCard(p, 'Pasta Renomeada').locator('.drive-folder-menu-btn').nth(1));
    await p.waitForSelector('.share-modal-card .share-entry-item', { timeout: 15000 });
    await tap(p.locator('.share-close-x-btn'));
    await p.waitForSelector('.share-modal-card', { state: 'detached', timeout: 5000 });
  });

  await step(p, 'excluir pasta', async () => {
    await tap(folderCard(p, 'Pasta Renomeada').locator('.drive-folder-menu-btn').nth(3));
    await tap(p.locator('.modal-card .btn-danger'));
    await folderCard(p, 'Pasta Renomeada').waitFor({ state: 'detached', timeout: 15000 });
    await p.locator('.character-card', { hasText: 'Maga do Drive' }).waitFor({ timeout: 15000 });
  });

  await step(p, 'tocar no card abre a ficha', async () => {
    await tap(p.locator('.character-card', { hasText: 'Maga do Drive' }).locator('.card-name'));
    await p.waitForURL(/\/sheet\//, { timeout: 15000 });
  });

  const sheetWidth = () => p.evaluate(() => document.querySelector('.sheet-container').getBoundingClientRect().width);
  if (PROFILE === 'iphone') {
    await step(p, 'modo Larga não aparece no celular', async () => {
      await p.waitForSelector('.sheet-container', { timeout: 30000 });
      await tap(p.locator('.actions-menu-trigger-btn'));
      await p.waitForSelector('.actions-menu-backdrop.open', { timeout: 5000 });
      const visible = await p.locator('.layout-toggle-btn').isVisible();
      await tap(p.locator('.actions-menu-panel .panel-close-btn'));
      if (visible) throw new Error('opção de largura visível no celular');
    });
  } else {
    await step(p, 'modo Larga alarga a ficha e fica salvo', async () => {
      await p.setViewportSize({ width: 1600, height: 1000 });
      // A largura fica no menu "⋯", seção Exibição
      await p.waitForSelector('.actions-menu-trigger-btn', { timeout: 30000 });
      const a4 = await sheetWidth();
      await p.locator('.actions-menu-trigger-btn').click();
      await p.locator('.layout-toggle-btn').click();
      await p.locator('.actions-menu-panel .panel-close-btn').click();
      await p.waitForFunction(() => document.querySelector('.sheet-container').getBoundingClientRect().width > 1100, null, { timeout: 5000 });
      const wide = await sheetWidth();
      await p.reload({ waitUntil: 'networkidle' });
      await p.waitForFunction(() => document.querySelector('.sheet-container').getBoundingClientRect().width > 1100, null, { timeout: 15000 });
      return `${Math.round(a4)}px → ${Math.round(wide)}px, mantido após recarregar`;
    });

    await step(p, 'impressão continua em A4 no modo Larga', async () => {
      // O CSS de impressão usa a largura da página; o que importa é a regra larga não valer
      const pageWidth = () => p.evaluate(() => getComputedStyle(document.querySelector('.sheet-page-container')).getPropertyValue('--page-width').trim());
      const onScreen = await pageWidth();
      await p.emulateMedia({ media: 'print' });
      const printed = await pageWidth();
      await p.emulateMedia({ media: 'screen' });
      if (printed !== '210mm') throw new Error(`--page-width na impressão: ${printed}`);
      // Volta para A4 para não afetar outras execuções
      await p.locator('.actions-menu-trigger-btn').click();
      await p.locator('.layout-toggle-btn').click();
      await p.locator('.actions-menu-panel .panel-close-btn').click();
      return `tela: ${onScreen} · impressão: ${printed}`;
    });
  }

  await browser.close();
  const failed = results.filter(ok => !ok).length;
  console.log(`\n${results.length - failed}/${results.length} passos OK (${PROFILE})`);
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
