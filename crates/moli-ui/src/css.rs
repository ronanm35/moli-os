/// Feuille de style complète embarquée dans le binaire.
/// Charte « Le Relevé » + thème Maison. Aucun fichier externe.
pub const STYLES: &str = r#"
:root {
  /* Palette jour */
  --bg:       #edf0f5;
  --surface:  #ffffff;
  --s2:       #f5f7fa;
  --s3:       #e9edf3;
  --ink:      #171c28;
  --ink2:     #4a5265;
  --ink3:     #6b7386;
  --line:     #e1e5ec;
  --warm:     #f0a53a;
  --warm-ink: #8a4b06;
  --warm-bg:  #fff2dc;
  --warm-glow:rgba(240,165,58,.38);
  --cool:     #4b74f2;
  --cool-bg:  #e8eefe;
  --good:     #2c9a88;
  --good-bg:  #e1f3ef;
  --alert:    #df5a40;
  --alert-bg: #fde8e3;
  --shadow:   0 1px 2px rgb(23 32 56/.05),0 10px 30px -12px rgb(23 32 56/.16);
  --lift:     0 1px 2px rgb(23 32 56/.06),0 16px 40px -14px rgb(23 32 56/.24);
  --r:        20px;
  --ease:     cubic-bezier(.22,1,.36,1);
  color-scheme: light;
}

@media (prefers-color-scheme:dark) {
  :root {
    --bg:      #0d1016;
    --surface: #161a22;
    --s2:      #1b2029;
    --s3:      #232935;
    --ink:     #eceff5;
    --ink2:    #b3bac8;
    --ink3:    #788195;
    --line:    #252b37;
    --warm:     #f5ae4c;
    --warm-ink: #ffd49a;
    --warm-bg:  #33270f;
    --warm-glow:rgba(245,174,76,.30);
    --cool:    #7e9dff;
    --cool-bg: #1a2238;
    --good:    #4fc0ad;
    --good-bg: #142b28;
    --alert:   #ff7b62;
    --alert-bg:#3a1c16;
    --shadow:  0 1px 2px rgb(0 0 0/.30),0 12px 30px -14px rgb(0 0 0/.60);
    --lift:    0 1px 2px rgb(0 0 0/.35),0 18px 40px -14px rgb(0 0 0/.70);
    color-scheme: dark;
  }
}

*,*::before,*::after{box-sizing:border-box}

html,body{
  margin:0;
  min-height:100dvh;
  background:var(--bg);
  color:var(--ink);
  font-family:'Figtree Variable','Figtree',ui-sans-serif,system-ui,-apple-system,'Segoe UI',Roboto,sans-serif;
  font-size:16px;
  line-height:1.35;
  -webkit-font-smoothing:antialiased;
  -webkit-tap-highlight-color:transparent;
}

h1,h2,h3,p{margin:0}
button,select,input{font:inherit;color:inherit;cursor:pointer}
button:disabled{cursor:default;opacity:.5}
a{color:inherit;text-decoration:none}
:focus-visible{outline:3px solid var(--cool);outline-offset:3px;border-radius:10px}
.num{font-variant-numeric:tabular-nums}

/* ── Layout ────────────────────────────────────────────────── */
#shell{display:grid;grid-template-rows:56px 1fr;min-height:100dvh}

/* Header */
#header{
  display:flex;align-items:center;gap:16px;
  padding:0 20px;
  background:var(--surface);
  border-bottom:1px solid var(--line);
  position:sticky;top:0;z-index:40;
}
#header .logo{
  font-size:17px;font-weight:800;letter-spacing:-.02em;
  color:var(--ink);display:flex;align-items:center;gap:8px;
}
#header .logo svg{width:24px;height:24px;fill:var(--warm)}
#header nav{display:flex;gap:2px;flex:1;overflow-x:auto;scrollbar-width:none}
#header nav a{
  display:inline-flex;align-items:center;gap:6px;
  padding:8px 14px;border-radius:10px;
  font-size:14px;font-weight:600;color:var(--ink3);
  white-space:nowrap;transition:background .15s,color .15s;
}
#header nav a:hover{background:var(--s2);color:var(--ink2)}
#header nav a.on{background:var(--warm-bg);color:var(--warm-ink)}
#header .pill{
  margin-left:auto;flex:none;
  display:flex;align-items:center;gap:6px;
  padding:6px 12px;border-radius:99px;
  font-size:13px;font-weight:650;
  background:var(--s2);color:var(--ink3);
}

/* Main content */
#content{padding:24px 20px;max-width:1280px;margin:0 auto;width:100%}

/* ── Cards ─────────────────────────────────────────────────── */
.card{
  background:var(--surface);
  border-radius:var(--r);
  box-shadow:var(--shadow);
  padding:20px;
}
.card-head{
  display:flex;align-items:center;justify-content:space-between;gap:12px;
  margin-bottom:16px;
}
.card-head h2{
  font-size:15px;font-weight:650;letter-spacing:.01em;
  color:var(--ink2);display:flex;align-items:center;gap:8px;
}
.more{
  border:0;background:var(--s2);border-radius:99px;
  width:36px;height:36px;display:grid;place-items:center;
  color:var(--ink2);transition:background .2s;flex:none;
}
.more:hover{background:var(--s3)}

/* ── Chips ─────────────────────────────────────────────────── */
.chip{
  display:inline-flex;align-items:center;gap:6px;
  padding:9px 14px;border-radius:99px;font-size:14px;font-weight:600;
  border:0;cursor:pointer;
}
.chip.alert{background:var(--alert-bg);color:var(--alert)}
.chip.warm {background:var(--warm-bg); color:var(--warm-ink)}
.chip.cool {background:var(--cool-bg); color:var(--cool)}
.chip.good {background:var(--good-bg); color:var(--good)}
.chip.muted{background:var(--s2);      color:var(--ink3)}

/* ── Mode buttons ───────────────────────────────────────────── */
.modes{display:flex;gap:8px;overflow-x:auto;scrollbar-width:none;flex-wrap:nowrap}
.modes button{
  flex:none;display:inline-flex;align-items:center;gap:6px;
  border:0;border-radius:99px;padding:8px 14px;
  font-size:14px;font-weight:650;
  background:var(--s2);color:var(--ink3);
  transition:background .2s,color .2s;
}
.modes button.on{background:var(--warm-bg);color:var(--warm-ink)}
.modes .sep{flex:none;width:1px;margin:6px 4px;background:var(--line)}

/* ── Light tiles ────────────────────────────────────────────── */
.light-tile{
  display:flex;flex-direction:column;gap:8px;
  padding:14px;border-radius:16px;
  background:var(--s2);border:0;text-align:left;
  transition:background .2s,box-shadow .2s;
  cursor:pointer;
}
.light-tile.on{background:var(--warm-bg);box-shadow:0 0 0 1px var(--warm-glow)}
.light-tile .icon{
  width:36px;height:36px;border-radius:10px;
  background:var(--s3);display:grid;place-items:center;
  transition:background .2s;
}
.light-tile.on .icon{background:var(--warm);color:#fff}
.light-tile .name{font-size:13px;font-weight:650;color:var(--ink2)}
.light-tile.on .name{color:var(--warm-ink)}

/* ── Sensor tiles ───────────────────────────────────────────── */
.sensor{
  background:var(--s2);border-radius:14px;
  padding:14px 16px;display:flex;flex-direction:column;gap:4px;
}
.sensor small{font-size:11px;font-weight:650;color:var(--ink3);text-transform:uppercase;letter-spacing:.06em}
.sensor b{font-size:20px;font-weight:700}

/* ── Notices ────────────────────────────────────────────────── */
.notices{display:flex;flex-wrap:wrap;gap:10px;margin-bottom:4px}

/* ── Grid ───────────────────────────────────────────────────── */
.grid{
  display:grid;
  grid-template-columns:repeat(12,minmax(0,1fr));
  gap:20px;align-items:start;
}
.col8{grid-column:span 8}
.col7{grid-column:span 7}
.col6{grid-column:span 6}
.col5{grid-column:span 5}
.col4{grid-column:span 4}
.col3{grid-column:span 3}
.col12{grid-column:span 12}

/* ── Tiles grid ─────────────────────────────────────────────── */
.tiles{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:12px}
.clims{display:grid;grid-template-columns:1fr 1fr;gap:12px}

/* ── Climate card ───────────────────────────────────────────── */
.climate-card{
  display:flex;align-items:center;justify-content:space-between;gap:12px;
  padding:14px 16px;background:var(--s2);border-radius:16px;
}
.climate-card .temp{font-size:26px;font-weight:800}
.climate-card .set{font-size:13px;color:var(--ink3)}
.climate-card .ctrl{display:flex;gap:6px}
.climate-card .ctrl button{
  width:32px;height:32px;border-radius:50%;border:0;
  background:var(--surface);font-size:18px;
  display:grid;place-items:center;box-shadow:var(--shadow);
}

/* ── Fancy TV remote card ───────────────────────────────────── */
.remote-card{
  display:grid;grid-template-columns:1fr auto;gap:14px;
  min-height:168px;align-content:space-between;
  transition:transform .2s var(--ease);
}
.remote-card:hover{transform:translateY(-2px)}
.tv-obj{grid-column:1/-1;display:grid;justify-items:center;gap:0}
.screen{
  width:96px;height:56px;border-radius:8px;
  background:#171c28;box-shadow:inset 0 0 0 3px #2a3142;
  transition:background .4s;
}
.screen.on{
  background:linear-gradient(135deg,#5fb4f0,#8f8cf5 55%,#f0a53a);
  box-shadow:inset 0 0 0 3px #2a3142,0 8px 26px -6px #8f8cf5aa;
}
.stand{width:30px;height:6px;border-radius:0 0 6px 6px;background:#2a3142}
.remote-text{display:grid}
.remote-text strong{font-size:18px}
.remote-text span{font-size:13px;color:var(--ink3)}
.go-btn{
  align-self:end;width:36px;height:36px;border-radius:50%;
  background:var(--s2);display:grid;place-items:center;color:var(--ink2);
}

/* ── Outside card ───────────────────────────────────────────── */
.outside-card{transition:transform .2s var(--ease)}
.outside-card:hover{transform:translateY(-2px)}
.facts{display:grid;grid-template-columns:repeat(4,1fr);gap:8px}
.facts small{display:block;font-size:11px;color:var(--ink3);font-weight:650}
.facts b{font-size:17px;font-weight:650}

/* ── Sky hero ───────────────────────────────────────────────── */
.sky-hero{
  border-radius:var(--r);overflow:hidden;
  background:linear-gradient(160deg,#1a2a4a,#2d4f8a);
  padding:24px;color:#fff;min-height:140px;
  display:flex;flex-direction:column;justify-content:space-between;
}
.sky-hero .temp-main{font-size:52px;font-weight:800;line-height:1}
.sky-hero .sky-row{display:flex;align-items:flex-end;justify-content:space-between;gap:16px}
.sky-hero .sky-desc{font-size:15px;opacity:.75}
.sky-hero .sky-detail{display:flex;gap:20px}
.sky-hero .sky-detail span{font-size:13px;opacity:.8}

/* ── Muted text ─────────────────────────────────────────────── */
.muted{color:var(--ink3)}
.small{font-size:13px}

/* ── Section spacing ────────────────────────────────────────── */
.accueil{display:grid;gap:22px}

/* ── Live indicator ─────────────────────────────────────────── */
#live-dot{
  width:8px;height:8px;border-radius:50%;
  background:var(--good);flex:none;
  transition:background .3s;
}
#live-dot.dead{background:var(--alert)}
#live-dot.wait{background:var(--warm)}

/* ── Responsive ─────────────────────────────────────────────── */
@media(max-width:1180px){
  .col8,.col7{grid-column:span 12}
  .col5,.col4{grid-column:span 6}
  .col3{grid-column:span 6}
}
@media(max-width:760px){
  .grid>*{grid-column:span 12!important}
  .tiles{grid-template-columns:1fr 1fr}
  .clims{grid-template-columns:1fr}
  .facts{grid-template-columns:repeat(2,1fr)}
  #header nav a span{display:none}
}
"#;
