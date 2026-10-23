// E2E do mapa tático montável no WebKit: mestre (desktop) monta, jogador (iPhone) vê
// em tempo real e move só o próprio token.
// Uso: node room_map.js <url-base> <pasta-de-saída>
const path = require('path');
const pw = require('playwright');

const BASE = process.argv[2];
const OUT = process.argv[3] || '.';
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
const getMap = (ctx, roomId) => serverFn(ctx, 'get_room_map', { room_id: roomId });

// Centro de uma célula em pixels do tabuleiro (mesmas fórmulas de rules/map_grid.rs)
function cellCenter(shape, cs, col, row) {
  const r = cs / 2;
  if (shape === 'square') return [col * cs + r, row * cs + r];
  const s3 = Math.sqrt(3);
  if (shape === 'hex_pointy') { const w = s3 * r; return [col * w + (row % 2 ? w / 2 : 0) + w / 2, row * 1.5 * r + r]; }
  const h = s3 * r; return [col * 1.5 * r + r, row * h + (col % 2 ? h / 2 : 0) + h / 2];
}
// Converte posição do tabuleiro para coordenada de tela, considerando o zoom atual
async function toScreen(page, bx, by) {
  return page.evaluate(([bx, by]) => {
    const board = document.querySelector('.grid-board');
    board.scrollIntoView({ block: 'center', inline: 'center' });
    const rect = board.getBoundingClientRect();
    const zoom = rect.width / parseFloat(board.style.width);
    return [rect.left + bx * zoom, rect.top + by * zoom];
  }, [bx, by]);
}
async function dragOnBoard(page, from, to, steps = 8) {
  const [x1, y1] = await toScreen(page, ...from);
  const [x2, y2] = await toScreen(page, ...to);
  await page.mouse.move(x1, y1);
  await page.mouse.down();
  await page.mouse.move(x2, y2, { steps });
  await page.mouse.up();
}
// Arraste com tremor vertical (como uma mão de verdade passando perto dos cantos)
async function shakyDrag(page, from, to, wobble = 14, steps = 24) {
  const [x1, y1] = await toScreen(page, ...from);
  const [x2, y2] = await toScreen(page, ...to);
  await page.mouse.move(x1, y1);
  await page.mouse.down();
  for (let i = 1; i <= steps; i++) {
    const t = i / steps;
    await page.mouse.move(x1 + (x2 - x1) * t, y1 + (y2 - y1) * t + (i % 2 ? wobble : -wobble) * (i < steps ? 1 : 0));
  }
  await page.mouse.up();
}
async function tapOnBoard(page, bx, by) {
  const [x, y] = await toScreen(page, bx, by);
  await page.mouse.click(x, y);
}
// Cor do pixel do canvas do mapa num ponto do tabuleiro
async function canvasColor(page, bx, by) {
  return page.evaluate(([bx, by]) => {
    const canvas = document.querySelector('.grid-canvas');
    const ratio = canvas.width / parseFloat(document.querySelector('.grid-board').style.width);
    const d = canvas.getContext('2d').getImageData(Math.round(bx * ratio), Math.round(by * ratio), 1, 1).data;
    return [d[0], d[1], d[2]];
  }, [bx, by]);
}
const near = (c, hex) => {
  const n = parseInt(hex.slice(1), 16);
  const t = [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  return c.every((v, i) => Math.abs(v - t[i]) < 30);
};

(async () => {
  const browser = await pw.webkit.launch();
  const gmCtx = await browser.newContext({ viewport: { width: 1366, height: 900 } });
  const plCtx = await browser.newContext({ ...pw.devices['iPhone 14'] });
  await register(gmCtx, 'gm_' + run);
  await register(plCtx, 'pl_' + run);
  const roomId = await serverFn(gmCtx, 'create_room', { name: 'Mapa ' + run, description: 'e2e', is_public: 'false' });
  const details = await serverFn(gmCtx, 'get_room_details', { room_id: roomId });
  await serverFn(plCtx, 'join_room_by_code', { code: details.code });
  const plSheet = await serverFn(plCtx, 'create_sheet', { name: 'Heroina ' + run, sheet_type: 'mage' });
  await serverFn(plCtx, 'assign_sheet_to_room', { sheet_id: plSheet, room_id: roomId });

  const errors = [];
  const gm = await gmCtx.newPage();
  const pl = await plCtx.newPage();
  for (const [p, label] of [[gm, 'mestre'], [pl, 'jogador']]) {
    p.on('pageerror', e => errors.push(`[${label}] ${e.message}`));
    p.on('console', m => { if (m.type() === 'error' && !/favicon|Failed to load resource/.test(m.text())) errors.push(`[${label}] ${m.text()}`); });
  }
  const openMap = async (p) => {
    await p.goto(`${BASE}/room/${roomId}`, { waitUntil: 'networkidle' });
    const tab = p.locator('.room-tab-btn:has-text("Mapa")').first();
    await tab.waitFor({ timeout: 20000 });
    if (await tab.isVisible()) await tab.click(); else await tab.evaluate(b => b.click());
    await p.waitForSelector('.grid-board', { timeout: 20000 });
    await p.waitForFunction(() => !document.querySelector('.grid-loading'), null, { timeout: 15000 });
  };

  await step('abrir mapa', async () => {
    await openMap(gm);
    await openMap(pl);
    check('aba Mapa liberada para mestre e jogador', true);
    const plTools = await pl.locator('.grid-tool-btn').allInnerTexts();
    check('jogador vê só Mover e Régua', plTools.length === 2, plTools.join(', ').replace(/\s+/g, ' '));
  });

  const cs = 50;
  await step('personagens', async () => {
    await gm.locator('.grid-action-btn:has-text("Personagens")').click();
    await pl.waitForSelector('.grid-token', { timeout: 10000 });
    check('token do personagem aparece para o jogador em tempo real', true);
  });

  await step('terreno', async () => {
    await gm.locator('.grid-tool-btn:has-text("Terreno")').click();
    await gm.locator('.grid-swatch:has-text("Água funda")').click();
    await dragOnBoard(gm, cellCenter('square', cs, 3, 3), cellCenter('square', cs, 7, 3));
    await gm.waitForTimeout(1500);
    const map = (await getMap(gmCtx, roomId)).map;
    const water = map.terrain.filter(t => t.kind === 'deep_water').length;
    check('pincelada vira UMA gravação com várias células', water >= 5, `${water} células de água funda`);
    const color = await canvasColor(pl, ...cellCenter('square', cs, 5, 3));
    check('jogador vê o terreno no canvas (pixel)', near(color, '#245d8f'), `rgb(${color})`);
  });

  await step('paredes', async () => {
    await gm.locator('.grid-tool-btn:has-text("Paredes")').click();
    await gm.locator('.grid-swatch:has-text("Parede")').first().click();
    // Linha horizontal no y = 2 células, do x = 1 ao x = 8, com o mouse tremendo
    await shakyDrag(gm, [1 * cs + 4, 2 * cs + 6], [8 * cs - 3, 2 * cs - 5]);
    await gm.waitForTimeout(1200);
    let map = (await getMap(plCtx, roomId)).map;
    const walls = map.segments.filter(w => w.kind === 'wall');
    const w = walls[0] || {};
    const straight = walls.length === 1 && w.a.y === 2 && w.b.y === 2 && Math.abs(Math.abs(w.b.x - w.a.x) - 7) < 0.01;
    check('arraste tremido vira UMA parede reta', straight, JSON.stringify(map.segments));

    // Porta de uma célula e abrir/fechar no modo Mover
    await gm.locator('.grid-swatch:has-text("Porta fechada")').click();
    await dragOnBoard(gm, [4 * cs, 6 * cs], [5 * cs, 6 * cs]);
    await gm.waitForTimeout(1000);
    await gm.locator('.grid-tool-btn:has-text("Mover")').click();
    await tapOnBoard(gm, 4.5 * cs, 6 * cs);
    await gm.waitForTimeout(1200);
    map = (await getMap(plCtx, roomId)).map;
    check('porta desenhada e aberta no modo Mover', map.segments.some(s => s.kind === 'door_open'), JSON.stringify(map.segments.map(s => s.kind)));

    // Borracha passando por cima da parede
    await gm.locator('.grid-tool-btn:has-text("Paredes")').click();
    await gm.locator('.grid-swatch:has-text("Apagar")').click();
    await dragOnBoard(gm, [3 * cs, 2 * cs - 15], [3 * cs, 2 * cs + 15]);
    await gm.waitForTimeout(1000);
    map = (await getMap(plCtx, roomId)).map;
    check('borracha apaga a parede inteira', !map.segments.some(s => s.kind === 'wall'), JSON.stringify(map.segments.map(s => s.kind)));
    // Uma parede para continuar existindo no resto do teste
    await gm.locator('.grid-swatch:has-text("Parede")').first().click();
    await dragOnBoard(gm, [1 * cs, 10 * cs], [6 * cs, 10 * cs]);
    await gm.waitForTimeout(800);
  });

  await step('objetos', async () => {
    await gm.locator('.grid-tool-btn:has-text("Objetos")').click();
    await gm.locator('.grid-swatch:has-text("Baú")').click();
    await tapOnBoard(gm, ...cellCenter('square', cs, 9, 9));
    await gm.waitForTimeout(1200);
    const map = (await getMap(plCtx, roomId)).map;
    const chest = map.props.find(p => p.kind === 'chest');
    check('objeto colocado na célula tocada', chest && chest.cell.col === 9 && chest.cell.row === 9, JSON.stringify(chest));
  });

  await step('jogador move o próprio token', async () => {
    const map = (await getMap(plCtx, roomId)).map;
    const mine = map.tokens.find(t => t.sheet_id === plSheet);
    await pl.locator('.grid-tool-btn:has-text("Mover")').click();
    await dragOnBoard(pl, cellCenter('square', cs, mine.grid_col, mine.grid_row), cellCenter('square', cs, 12, 12), 12);
    await pl.waitForTimeout(1500);
    const after = (await getMap(gmCtx, roomId)).map.tokens.find(t => t.sheet_id === plSheet);
    check('arrastar move o token no servidor', after.grid_col === 12 && after.grid_row === 12, `(${after.grid_col}, ${after.grid_row})`);
    const gmLeft = await gm.locator('.grid-token').first().evaluate(e => parseFloat(e.style.left));
    check('mestre vê a nova posição', Math.abs(gmLeft - (12 * cs + cs / 2 - 21.25)) < 3, `left=${gmLeft}`);
  });

  await step('NPC e permissões', async () => {
    await gm.locator('.grid-action-btn:has-text("NPC")').click();
    await gm.locator('.grid-modal-content input[type="text"]').fill('Agente');
    await gm.locator('.grid-modal-content .btn-primary').click();
    await gm.waitForTimeout(1200);
    let map = (await getMap(gmCtx, roomId)).map;
    const npc = map.tokens.find(t => t.is_npc);
    await pl.waitForFunction(() => document.querySelectorAll('.grid-token').length >= 2, null, { timeout: 8000 });
    await dragOnBoard(pl, cellCenter('square', cs, npc.grid_col, npc.grid_row), cellCenter('square', cs, 1, 1), 10);
    await pl.waitForTimeout(1500);
    map = (await getMap(gmCtx, roomId)).map;
    const still = map.tokens.find(t => t.id === npc.id);
    check('jogador NÃO consegue mover o NPC', still.grid_col === npc.grid_col && still.grid_row === npc.grid_row);

    // Mestre oculta o NPC: some da tela e do servidor para o jogador
    await gm.locator('.grid-tool-btn:has-text("Mover")').click();
    await tapOnBoard(gm, ...cellCenter('square', cs, npc.grid_col, npc.grid_row));
    await gm.locator('.grid-selection-actions button:has-text("Ocultar")').click();
    await pl.waitForFunction(() => document.querySelectorAll('.grid-token').length === 1, null, { timeout: 8000 });
    const plMap = (await getMap(plCtx, roomId)).map;
    check('token oculto não chega ao jogador', !plMap.tokens.some(t => t.id === npc.id));
  });

  await step('grade hexagonal', async () => {
    await gm.locator('.grid-action-btn:has-text("Grade")').click();
    await gm.locator('.toggle-choice-btn:has-text("pontas")').click();
    await gm.locator('.grid-modal-content .btn-primary').click();
    await pl.waitForTimeout(1500);
    await gm.locator('.grid-tool-btn:has-text("Terreno")').click();
    await gm.locator('.grid-swatch:has-text("Lava")').click();
    await tapOnBoard(gm, ...cellCenter('hex_pointy', cs, 5, 7));
    await gm.waitForTimeout(1200);
    const map = (await getMap(plCtx, roomId)).map;
    check('mapa vira hexagonal para o jogador', map.grid_shape === 'hex_pointy');
    check('toque no hexágono pinta a célula certa', map.terrain.some(t => t.kind === 'lava' && t.cell.col === 5 && t.cell.row === 7));
    const color = await canvasColor(pl, ...cellCenter('hex_pointy', cs, 5, 7));
    check('jogador vê o hexágono pintado (pixel)', near(color, '#d9480f'), `rgb(${color})`);
    check('paredes continuam depois de trocar para hexagonal', map.segments.length >= 1);

    // Linha reta no hexágono passando pelo centro das células
    await gm.locator('.grid-tool-btn:has-text("Paredes")').click();
    await gm.locator('.grid-swatch:has-text("Parede")').first().click();
    const [ax, ay] = cellCenter('hex_pointy', cs, 2, 12);
    const [bx, by] = cellCenter('hex_pointy', cs, 7, 12);
    await shakyDrag(gm, [ax + 3, ay - 2], [bx - 2, by + 3], 6);
    await gm.waitForTimeout(1200);
    const hexMap = (await getMap(plCtx, roomId)).map;
    const line = hexMap.segments[hexMap.segments.length - 1];
    const ok = line && Math.abs(line.a.x * cs - ax) < 0.5 && Math.abs(line.a.y * cs - ay) < 0.5 && Math.abs(line.b.x * cs - bx) < 0.5 && Math.abs(line.b.y * cs - by) < 0.5;
    check('no hexágono a linha reta encaixa nos centros', ok, JSON.stringify(line));
  });

  await gm.screenshot({ path: path.join(OUT, 'mapa_mestre.png') });
  await pl.screenshot({ path: path.join(OUT, 'mapa_iphone.png') });
  check('sem erros de JavaScript no WebKit', errors.length === 0, errors.slice(0, 4).join(' | '));

  await browser.close();
  const failed = results.filter(r => !r.ok).length;
  console.log(`\nRESUMO: ${results.length - failed}/${results.length} verificações passaram`);
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error('ERRO FATAL:', e.stack); process.exit(2); });
