use crate::manifest::schema::ChangeManifest;

const HTML_HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Structix Report</title>
<style>
:root {
  --color-added:    #22c55e;
  --color-removed:  #ef4444;
  --color-modified: #eab308;
  --color-moved:    #3b82f6;
  --color-renamed:  #a855f7;
  --color-bg:       #0f172a;
  --color-surface:  #1e293b;
  --color-border:   #334155;
  --color-text:     #e2e8f0;
  --color-muted:    #94a3b8;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: system-ui, -apple-system, sans-serif; background: var(--color-bg); color: var(--color-text); height: 100vh; display: flex; flex-direction: column; overflow: hidden; }

/* Header */
#header-bar { display: flex; align-items: center; gap: 1rem; padding: 0.6rem 1.25rem; background: var(--color-surface); border-bottom: 1px solid var(--color-border); flex-shrink: 0; flex-wrap: wrap; }
#repo-title { font-weight: 700; font-size: 1rem; letter-spacing: -0.01em; }
#ref-range { font-family: monospace; font-size: 0.85rem; color: var(--color-muted); }
#pill-row { display: flex; gap: 0.4rem; flex-wrap: wrap; margin-left: auto; }
.pill { padding: 2px 10px; border-radius: 12px; font-size: 0.72rem; font-weight: 700; }
.pill-added    { background: rgba(34,197,94,0.15);   color: var(--color-added); }
.pill-removed  { background: rgba(239,68,68,0.15);   color: var(--color-removed); }
.pill-modified { background: rgba(234,179,8,0.15);   color: var(--color-modified); }
.pill-moved    { background: rgba(59,130,246,0.15);  color: var(--color-moved); }
.pill-renamed  { background: rgba(168,85,247,0.15);  color: var(--color-renamed); }

/* Layout */
#layout { display: grid; grid-template-columns: 220px 1fr 260px; flex: 1; min-height: 0; }
#sidebar { overflow-y: auto; border-right: 1px solid var(--color-border); padding: 0.6rem; }
#main-panel { overflow-y: auto; padding: 1rem; }
#right-panel { overflow-y: auto; border-left: 1px solid var(--color-border); padding: 0.75rem; display: flex; flex-direction: column; }
#right-panel h3 { font-size: 0.78rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--color-muted); margin-bottom: 0.5rem; flex-shrink: 0; }
#flow-svg { flex: 1; min-height: 0; width: 100%; }

/* Footer */
#stats-bar { height: 32px; display: flex; align-items: center; padding: 0 1.25rem; border-top: 1px solid var(--color-border); background: var(--color-surface); font-size: 0.78rem; color: var(--color-muted); flex-shrink: 0; }

/* Sidebar */
#sidebar-all { display: flex; justify-content: space-between; align-items: center; padding: 0.3rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem; color: var(--color-muted); margin-bottom: 0.25rem; }
#sidebar-all:hover, #sidebar-all.active { background: var(--color-border); color: var(--color-text); }
.file-entry { display: flex; justify-content: space-between; align-items: center; padding: 0.28rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.78rem; }
.file-entry:hover { background: var(--color-border); }
.file-entry.active { background: var(--color-border); color: var(--color-text); }
.file-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; min-width: 0; color: var(--color-muted); }
.file-entry.active .file-name { color: var(--color-text); }
.file-badge { font-size: 0.68rem; background: rgba(255,255,255,0.07); padding: 1px 6px; border-radius: 10px; flex-shrink: 0; margin-left: 4px; }

/* Group headers */
.file-group-header { font-size: 0.75rem; color: var(--color-muted); margin: 1rem 0 0.4rem; padding-bottom: 0.3rem; border-bottom: 1px solid var(--color-border); font-family: monospace; word-break: break-all; }
.file-group-header:first-child { margin-top: 0; }

/* Entity cards */
.entity-card { border-left: 3px solid transparent; border-radius: 4px; background: var(--color-surface); margin-bottom: 0.4rem; }
.entity-card[data-kind="added"]    { border-left-color: var(--color-added); }
.entity-card[data-kind="removed"]  { border-left-color: var(--color-removed); }
.entity-card[data-kind="modified"] { border-left-color: var(--color-modified); }
.entity-card[data-kind="moved"]    { border-left-color: var(--color-moved); }
.entity-card[data-kind="renamed"]  { border-left-color: var(--color-renamed); }
.card-header { display: flex; align-items: center; gap: 0.45rem; padding: 0.5rem 0.75rem; cursor: pointer; user-select: none; }
.card-header:hover { background: rgba(255,255,255,0.03); border-radius: 4px; }
.entity-badge { font-size: 0.6rem; font-weight: 700; background: rgba(255,255,255,0.08); padding: 2px 5px; border-radius: 3px; text-transform: uppercase; flex-shrink: 0; letter-spacing: 0.03em; }
.entity-name { font-weight: 600; font-size: 0.88rem; }
.entity-kind-tag { font-size: 0.72rem; color: var(--color-muted); }
.change-kind-tag { margin-left: auto; font-size: 0.7rem; color: var(--color-muted); flex-shrink: 0; }
.card-detail { display: none; padding: 0 0.75rem 0.65rem; font-size: 0.82rem; }
.entity-card.expanded .card-detail { display: block; }
.detail-divider { height: 1px; background: var(--color-border); margin-bottom: 0.5rem; }

/* Signature diff */
.sig-row { font-family: monospace; padding: 0.2rem 0.5rem; border-radius: 3px; margin: 0.2rem 0; font-size: 0.8rem; word-break: break-all; }
.sig-before { background: rgba(239,68,68,0.12); color: #fca5a5; }
.sig-after  { background: rgba(34,197,94,0.12);  color: #86efac; }

/* Path move row */
.path-row { font-family: monospace; font-size: 0.78rem; margin-bottom: 0.4rem; word-break: break-all; }
.path-old { color: #fca5a5; }
.path-new { color: #86efac; }
.path-arrow { color: var(--color-muted); margin: 0 0.3rem; }

/* Source blocks */
.source-label { font-size: 0.7rem; color: var(--color-muted); margin: 0.5rem 0 0.2rem; text-transform: uppercase; letter-spacing: 0.05em; }
.source-block { background: #020617; border-radius: 4px; padding: 0.5rem 0.6rem; font-family: monospace; font-size: 0.76rem; white-space: pre-wrap; overflow-x: auto; max-height: 260px; overflow-y: auto; border: 1px solid var(--color-border); }

/* Empty state */
.empty-state { color: var(--color-muted); font-size: 0.85rem; padding: 2rem; text-align: center; }
</style>
</head>
<body>
<div id="header-bar">
  <span id="repo-title">Structix Report</span>
  <span id="ref-range"></span>
  <div id="pill-row"></div>
</div>
<div id="layout">
  <nav id="sidebar">
    <div id="sidebar-all" class="active">
      <span>All files</span>
      <span id="sidebar-all-count" class="file-badge"></span>
    </div>
    <div id="file-tree"></div>
  </nav>
  <main id="main-panel"></main>
  <aside id="right-panel">
    <h3>Change Flow</h3>
    <svg id="flow-svg"></svg>
  </aside>
</div>
<footer id="stats-bar"><span id="stats-summary"></span></footer>"#;

const HTML_SCRIPT: &str = r##"<script>
(function () {
  var raw = document.getElementById('structix-data').textContent;
  var manifest = JSON.parse(raw);

  renderHeader(manifest);
  renderSidebar(manifest);
  renderMainPanel(manifest, null);
  renderFlowDiagram(manifest);
  renderStatsBar(manifest);

  // ── Header ────────────────────────────────────────────────────────────────
  function renderHeader(m) {
    document.getElementById('ref-range').textContent = m.from_ref + ' \u2192 ' + m.to_ref;
    document.title = 'Structix: ' + m.from_ref + ' \u2192 ' + m.to_ref;
    var counts = countByKind(m.changes);
    var pillRow = document.getElementById('pill-row');
    ['added','removed','modified','moved','renamed'].forEach(function(k) {
      if (!counts[k]) return;
      var span = document.createElement('span');
      span.className = 'pill pill-' + k;
      span.textContent = capitalize(k) + ' ' + counts[k];
      pillRow.appendChild(span);
    });
  }

  // ── Sidebar ───────────────────────────────────────────────────────────────
  function renderSidebar(m) {
    var allEl = document.getElementById('sidebar-all');
    document.getElementById('sidebar-all-count').textContent = m.changes.length;
    allEl.addEventListener('click', function() {
      setActiveFile(null);
      renderMainPanel(m, null);
    });

    var fileCounts = {};
    m.changes.forEach(function(c) {
      [c.file_path, c.old_file_path].forEach(function(f) {
        if (!f) return;
        fileCounts[f] = (fileCounts[f] || 0) + 1;
      });
    });

    var tree = document.getElementById('file-tree');
    Object.keys(fileCounts).sort().forEach(function(file) {
      var div = document.createElement('div');
      div.className = 'file-entry';
      div.dataset.file = file;
      div.innerHTML =
        '<span class="file-name" title="' + escHtml(file) + '">' + escHtml(shortPath(file)) + '</span>' +
        '<span class="file-badge">' + fileCounts[file] + '</span>';
      div.addEventListener('click', function() {
        setActiveFile(file);
        renderMainPanel(m, file);
      });
      tree.appendChild(div);
    });
  }

  function setActiveFile(file) {
    var allEl = document.getElementById('sidebar-all');
    allEl.classList.toggle('active', file === null);
    document.querySelectorAll('#file-tree .file-entry').forEach(function(el) {
      el.classList.toggle('active', el.dataset.file === file);
    });
  }

  // ── Main panel ────────────────────────────────────────────────────────────
  function renderMainPanel(m, fileFilter) {
    var panel = document.getElementById('main-panel');
    panel.innerHTML = '';

    var byFile = groupByFile(m.changes, fileFilter);
    var files = Object.keys(byFile).sort();

    if (files.length === 0) {
      panel.innerHTML = '<div class="empty-state">No changes match the current filter.</div>';
      return;
    }

    files.forEach(function(file) {
      var hdr = document.createElement('div');
      hdr.className = 'file-group-header';
      hdr.textContent = file;
      panel.appendChild(hdr);

      byFile[file].forEach(function(change) {
        panel.appendChild(buildEntityCard(change));
      });
    });
  }

  function groupByFile(changes, fileFilter) {
    var out = {};
    changes.forEach(function(c) {
      if (fileFilter && c.file_path !== fileFilter && c.old_file_path !== fileFilter) return;
      if (!out[c.file_path]) out[c.file_path] = [];
      out[c.file_path].push(c);
    });
    return out;
  }

  // ── Entity card ───────────────────────────────────────────────────────────
  function buildEntityCard(change) {
    var card = document.createElement('div');
    card.className = 'entity-card';
    card.dataset.kind = change.kind;

    var hdr = document.createElement('div');
    hdr.className = 'card-header';
    hdr.innerHTML =
      '<span class="entity-badge">' + kindIcon(change.entity_kind) + '</span>' +
      '<span class="entity-name">' + escHtml(change.name) + '</span>' +
      '<span class="entity-kind-tag">' + escHtml(change.entity_kind) + '</span>' +
      '<span class="change-kind-tag">' + change.kind + '</span>';

    var detail = document.createElement('div');
    detail.className = 'card-detail';
    detail.innerHTML = '<div class="detail-divider"></div>' + buildDetail(change);

    hdr.addEventListener('click', function() {
      card.classList.toggle('expanded');
    });

    card.appendChild(hdr);
    card.appendChild(detail);
    return card;
  }

  function buildDetail(c) {
    var html = '';
    if (c.kind === 'modified') {
      var sigChanged = c.before_signature && c.after_signature && c.before_signature !== c.after_signature;
      if (sigChanged) {
        html += '<div class="sig-row sig-before">\u2212 ' + escHtml(c.before_signature) + '</div>';
        html += '<div class="sig-row sig-after">+ ' + escHtml(c.after_signature) + '</div>';
      }
      if (c.before_source) {
        html += '<div class="source-label">Before</div><div class="source-block">' + escHtml(c.before_source) + '</div>';
      }
      if (c.after_source) {
        html += '<div class="source-label">After</div><div class="source-block">' + escHtml(c.after_source) + '</div>';
      }
    } else if (c.kind === 'moved') {
      html += '<div class="path-row"><span class="path-old">' + escHtml(c.old_file_path || '') + '</span>' +
              '<span class="path-arrow">\u2192</span>' +
              '<span class="path-new">' + escHtml(c.file_path) + '</span></div>';
      if (c.after_source) {
        html += '<div class="source-block">' + escHtml(c.after_source) + '</div>';
      }
    } else if (c.kind === 'renamed') {
      html += '<div class="path-row"><span class="path-old">' + escHtml(c.old_name || '') + '</span>' +
              '<span class="path-arrow">\u2192</span>' +
              '<span class="path-new">' + escHtml(c.name) + '</span></div>';
      var sigChanged = c.before_signature && c.after_signature && c.before_signature !== c.after_signature;
      if (sigChanged) {
        html += '<div class="sig-row sig-before">\u2212 ' + escHtml(c.before_signature) + '</div>';
        html += '<div class="sig-row sig-after">+ ' + escHtml(c.after_signature) + '</div>';
      }
      if (c.after_source) {
        html += '<div class="source-block">' + escHtml(c.after_source) + '</div>';
      }
    } else {
      var src = c.after_source || c.before_source;
      if (src) html += '<div class="source-block">' + escHtml(src) + '</div>';
    }
    return html || '<span style="color:var(--color-muted);font-size:0.8rem">No detail available.</span>';
  }

  // ── Flow diagram ──────────────────────────────────────────────────────────
  function renderFlowDiagram(m) {
    var svg = document.getElementById('flow-svg');
    var flows = m.changes.map(function(c) {
      return {
        kind: c.kind,
        name: c.name,
        source: c.old_file_path || c.file_path,
        target: c.file_path
      };
    }).filter(function(flow) {
      return flow.source || flow.target;
    });

    if (flows.length === 0) {
      svg.innerHTML = '<text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" ' +
        'fill="#475569" font-family="system-ui,sans-serif" font-size="12">No file relationships to visualize</text>';
      return;
    }

    var W = svg.clientWidth || 240;
    var BAR_H = 22;
    var GAP = 8;
    var BAR_W = Math.min(100, Math.floor((W - 40) / 2));

    var srcFiles = unique(flows.map(function(c) { return c.source; }));
    var dstFiles = unique(flows.map(function(c) { return c.target; }));

    var srcPos = {};
    var dstPos = {};
    srcFiles.forEach(function(f, i) { srcPos[f] = 40 + i * (BAR_H + GAP) + BAR_H / 2; });
    dstFiles.forEach(function(f, i) { dstPos[f] = 40 + i * (BAR_H + GAP) + BAR_H / 2; });

    var totalH = Math.max(
      srcFiles.length * (BAR_H + GAP) + 50,
      dstFiles.length * (BAR_H + GAP) + 50
    );
    svg.setAttribute('viewBox', '0 0 ' + W + ' ' + totalH);
    svg.setAttribute('preserveAspectRatio', 'xMidYMin meet');

    var svgNS = 'http://www.w3.org/2000/svg';
    function el(tag, attrs, title) {
      var e = document.createElementNS(svgNS, tag);
      Object.keys(attrs).forEach(function(k) { e.setAttribute(k, attrs[k]); });
      if (title) {
        var t = document.createElementNS(svgNS, 'title');
        t.textContent = title;
        e.appendChild(t);
      }
      return e;
    }

    function strokeForKind(kind) {
      return {
        added: '#22c55e',
        removed: '#ef4444',
        modified: '#eab308',
        moved: '#3b82f6',
        renamed: '#a855f7'
      }[kind] || '#94a3b8';
    }

    var srcX = 4;
    var dstX = W - BAR_W - 4;

    srcFiles.forEach(function(f) {
      var cy = srcPos[f];
      svg.appendChild(el('rect', { x: srcX, y: cy - BAR_H/2, width: BAR_W, height: BAR_H, rx: 3,
        fill: '#1e293b', stroke: '#334155', 'stroke-width': 1 }));
      var t = el('text', { x: srcX + 5, y: cy + 4,
        fill: '#94a3b8', 'font-size': 10, 'font-family': 'system-ui,sans-serif' });
      t.textContent = shortPath(f);
      svg.appendChild(t);
    });

    dstFiles.forEach(function(f) {
      var cy = dstPos[f];
      svg.appendChild(el('rect', { x: dstX, y: cy - BAR_H/2, width: BAR_W, height: BAR_H, rx: 3,
        fill: '#1e293b', stroke: '#334155', 'stroke-width': 1 }));
      var t = el('text', { x: dstX + 5, y: cy + 4,
        fill: '#94a3b8', 'font-size': 10, 'font-family': 'system-ui,sans-serif' });
      t.textContent = shortPath(f);
      svg.appendChild(t);
    });

    flows.forEach(function(flow) {
      var y1 = srcPos[flow.source];
      var y2 = dstPos[flow.target];
      var x1 = srcX + BAR_W;
      var x2 = dstX;
      var cx = (x1 + x2) / 2;
      var path = el('path', {
        d: 'M' + x1 + ',' + y1 + ' C' + cx + ',' + y1 + ' ' + cx + ',' + y2 + ' ' + x2 + ',' + y2,
        fill: 'none', stroke: strokeForKind(flow.kind), 'stroke-width': flow.kind === 'modified' ? 2 : 1.5, opacity: 0.72
      }, flow.kind + ': ' + flow.name + ' (' + shortPath(flow.source) + ' → ' + shortPath(flow.target) + ')');
      path.addEventListener('mouseenter', function() {
        path.setAttribute('stroke-width', 3);
        path.setAttribute('opacity', 1);
      });
      path.addEventListener('mouseleave', function() {
        path.setAttribute('stroke-width', flow.kind === 'modified' ? 2 : 1.5);
        path.setAttribute('opacity', 0.72);
      });
      svg.appendChild(path);
    });
  }

  // ── Stats bar ─────────────────────────────────────────────────────────────
  function renderStatsBar(m) {
    var total = m.changes.length;
    var files = m.files_affected.length;
    var parts = [total + ' change' + (total !== 1 ? 's' : ''), files + ' file' + (files !== 1 ? 's' : '') + ' affected'];
    if (m.summary) parts.push(m.summary);
    document.getElementById('stats-summary').textContent = parts.join(' \u00b7 ');
  }

  // ── Utilities ─────────────────────────────────────────────────────────────
  function countByKind(changes) {
    var counts = {};
    changes.forEach(function(c) { counts[c.kind] = (counts[c.kind] || 0) + 1; });
    return counts;
  }
  function capitalize(s) { return s.charAt(0).toUpperCase() + s.slice(1); }
  function shortPath(p) {
    if (!p) return '';
    var parts = p.replace(/\\/g, '/').split('/');
    return parts.length <= 2 ? p : parts.slice(-2).join('/');
  }
  function escHtml(s) {
    if (!s) return '';
    return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')
            .replace(/"/g,'&quot;').replace(/'/g,'&#39;');
  }
  function kindIcon(k) {
    var icons = { 'function':'fn','method':'fn','class':'cls','interface':'ifc',
                  'enum':'enum','property':'prop','constructor':'ctor' };
    return icons[k] || k.slice(0,3);
  }
  function unique(arr) {
    var seen = {};
    return arr.filter(function(x) { if (!x || seen[x]) return false; seen[x] = true; return true; });
  }
})();
</script>"##;

const HTML_FOOT: &str = r#"</body>
</html>"#;

pub fn build_html_report(manifest: &ChangeManifest) -> String {
    let json = serde_json::to_string(manifest).unwrap_or_else(|_| "{}".to_string());
    let safe_json = json.replace("</script>", r"<\/script>");
    format!(
        "{}\n<script id=\"structix-data\" type=\"application/json\">{}</script>\n{}\n{}",
        HTML_HEAD, safe_json, HTML_SCRIPT, HTML_FOOT
    )
}
