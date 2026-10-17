/// Script vanilla embarqué (~500 octets) : connexion SSE + mise à jour DOM.
/// Zéro framework, zéro bibliothèque externe.
pub const SCRIPT: &str = r#"
(()=>{
  /* ── SSE live ─────────────────────────────────────────── */
  const dot=document.getElementById('live-dot');
  let es,retry=1e3;
  function connect(){
    es=new EventSource('/api/events');
    es.addEventListener('snapshot',e=>apply(JSON.parse(e.data).devices??[],'snap'));
    es.addEventListener('event',e=>{const ev=JSON.parse(e.data);applyOne(ev)});
    es.onopen=()=>{dot.className='';retry=1e3};
    es.onerror=()=>{dot.className='dead';es.close();setTimeout(connect,retry=Math.min(retry*2,3e4))};
  }
  connect();

  /* ── Appliquer un snapshot complet ────────────────────── */
  function apply(devices){
    for(const d of devices){
      for(const[k,s]of Object.entries(d.state??{})){
        setPoint(d.id+'/'+k,s.value);
      }
      /* disponibilité */
      const el=document.getElementById('dev-'+CSS.escape(d.id));
      if(el)el.classList.toggle('unavail',d.online===false);
    }
  }

  /* ── Appliquer un événement unique ────────────────────── */
  function applyOne(ev){
    if(ev.type==='State'&&ev.point&&ev.value!==undefined){
      setPoint(ev.point,ev.value);
    }
    if(ev.type==='Availability'){
      const el=document.getElementById('dev-'+CSS.escape(ev.device));
      if(el)el.classList.toggle('unavail',ev.online===false);
    }
  }

  /* ── Mettre à jour les éléments liés à un point ───────── */
  function setPoint(point,value){
    document.querySelectorAll('[data-p="'+point+'"]').forEach(el=>{
      const fmt=el.dataset.f;
      if(fmt==='bool'){
        el.classList.toggle('on',value===true||value==='on'||value==='true');
      }else if(fmt==='toggle'){
        el.setAttribute('aria-pressed',value===true?'true':'false');
        el.classList.toggle('on',value===true);
      }else{
        const unit=el.dataset.u??'';
        el.textContent=(value==null||value==='')?'—':value+unit;
      }
    });
  }

  /* ── Envoi d'une commande (boutons) ───────────────────── */
  window.mact=function(point,value,label){
    const body=JSON.stringify({point,value});
    fetch('/api/act',{method:'POST',headers:{'Content-Type':'application/json','x-moli-origin':'ui'},body})
      .catch(()=>{});
    /* retour visuel immédiat (optimiste) */
    setPoint(point,value);
    if(label){
      const t=document.getElementById('toast');
      if(t){t.textContent=label;t.classList.add('show');clearTimeout(t._t);t._t=setTimeout(()=>t.classList.remove('show'),2500)}
    }
  };

  /* ── Slider de luminosité / température couleur ───────── */
  document.addEventListener('change',e=>{
    const el=e.target;
    if(!el.dataset.p)return;
    mact(el.dataset.p,Number(el.value));
  });

  /* ── Toast ────────────────────────────────────────────── */
  const style=document.createElement('style');
  style.textContent='#toast{position:fixed;bottom:24px;left:50%;transform:translateX(-50%) translateY(120%);transition:transform .25s;background:var(--ink);color:var(--bg);padding:10px 20px;border-radius:99px;font-size:14px;font-weight:600;pointer-events:none;z-index:100}#toast.show{transform:translateX(-50%) translateY(0)}';
  document.head.appendChild(style);
  const toast=document.createElement('div');
  toast.id='toast';
  document.body.appendChild(toast);
})();
"#;
