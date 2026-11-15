// BEGIN UPDATE POLICY
(function (root) {
  'use strict';
  var API = 'https://api.github.com/repos/devchmod/TITAN-CORE/releases?per_page=100';
  function version(text) {
    if (typeof text !== 'string' || text.length > 128) return null;
    var m = /^v?(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/.exec(text);
    if (!m || [m[1],m[2],m[3]].some(function (n) { return n.length > 9; })) return null;
    var pre = m[4] ? m[4].split('.') : [];
    if (pre.some(function (x) { return /^[0-9]+$/.test(x) && x.length > 1 && x[0] === '0'; })) return null;
    return { core: [Number(m[1]),Number(m[2]),Number(m[3])], pre: pre, text: text.replace(/^v/, '') };
  }
  function compare(a, b) {
    a = typeof a === 'string' ? version(a) : a; b = typeof b === 'string' ? version(b) : b;
    if (!a || !b) throw Error('Invalid version');
    for (var i=0;i<3;i++) if (a.core[i] !== b.core[i]) return a.core[i] > b.core[i] ? 1 : -1;
    if (!a.pre.length || !b.pre.length) return a.pre.length ? -1 : b.pre.length ? 1 : 0;
    for (var j=0;j<Math.max(a.pre.length,b.pre.length);j++) {
      var x=a.pre[j], y=b.pre[j]; if (x===y) continue; if (x===undefined) return -1; if (y===undefined) return 1;
      var xn=/^[0-9]+$/.test(x), yn=/^[0-9]+$/.test(y);
      if (xn && yn && x.length !== y.length) return x.length > y.length ? 1 : -1;
      if (xn !== yn) return xn ? -1 : 1; return x > y ? 1 : -1;
    }
    return 0;
  }
  function assetURL(url, tag, name) {
    try {
      var u=new URL(url), parts=u.pathname.split('/').map(decodeURIComponent);
      return u.protocol==='https:' && u.hostname==='github.com' && !u.port && !u.username && !u.password && !u.search && !u.hash &&
        parts.length===7 && parts[1]==='devchmod' && parts[2]==='TITAN-CORE' && parts[3]==='releases' && parts[4]==='download' && parts[5]===tag && parts[6]===name;
    } catch (e) { return false; }
  }
  function select(releases) {
    if (!Array.isArray(releases)) throw Error('Invalid release response');
    var best=null;
    releases.slice(0,100).forEach(function (r) {
      if (!r || r.draft !== false || !r.published_at || !Number.isFinite(Date.parse(r.published_at))) return;
      var v=version(r.tag_name); if (!v || !Array.isArray(r.assets)) return;
      var a=r.assets.find(function (x) { return x && typeof x.name==='string' && /^TITAN-CORE(?:-[0-9A-Za-z.+-]+)?\.elf$/i.test(x.name) && x.state==='uploaded' && Number.isFinite(x.size) && x.size>0 && assetURL(x.browser_download_url,r.tag_name,x.name); });
      if (!a || (best && compare(v,best.version)<=0)) return;
      best={version:v,tag:r.tag_name,url:a.browser_download_url,name:a.name,size:a.size,
        page:'https://github.com/devchmod/TITAN-CORE/releases/tag/'+encodeURIComponent(r.tag_name),
        notes:typeof r.body==='string'?r.body.slice(0,4000):'',prerelease:r.prerelease===true||v.pre.length>0};
    });
    return best;
  }
  function createChecker(fetcher, options) {
    options=options||{};var pending=null,seq=0,controller=null;
    var s={phase:'idle',current:null,release:null,error:null,checkedAt:null,lastAttempt:0,loading:false};
    function snapshot(){return Object.assign({},s);}
    function emit(){if(options.onchange)options.onchange(snapshot());}
    function check(current) {
      if (pending && s.current===current) return pending;
      var v=version(current); if (!v) {seq++;if(controller)controller.abort();pending=null;s.phase='waiting';s.current=current;s.error=null;s.release=null;s.loading=false;emit();return Promise.resolve(snapshot());}
      var ticket=++seq;if(controller)controller.abort();controller=typeof AbortController==='function'?new AbortController():null;
      var ownController=controller,timer;
      s={phase:'checking',current:current,release:null,error:null,checkedAt:null,lastAttempt:Date.now(),loading:true};emit();
      pending=(async function () {
        try {
          var timeout=new Promise(function(_,reject){timer=setTimeout(function(){if(ownController)ownController.abort();reject(Error('timeout'));},options.timeoutMs||8000);});
          var request=(async function(){
            var r=await fetcher(API,{method:'GET',mode:'cors',credentials:'omit',cache:'no-store',redirect:'error',referrerPolicy:'no-referrer',headers:{Accept:'application/vnd.github+json'},signal:ownController?ownController.signal:undefined});
            if(!r.ok)throw Error(r.status===403||r.status===429?'rate-limit':'HTTP '+r.status);
            return select(await r.json());
          })();
          var chosen=await Promise.race([request,timeout]);
          if(ticket!==seq)return snapshot();
          s.release=chosen;s.phase=!chosen?'empty':compare(chosen.version,v)>0?'available':'current';s.checkedAt=Date.now();
        } catch(e) {if(ticket===seq){s.phase='error';s.error=e.message||'network';}}
        finally {clearTimeout(timer);if(ticket===seq){s.loading=false;pending=null;emit();}}
        return snapshot();
      })();
      return pending;
    }
    return {check:check,snapshot:snapshot};
  }
  var api={API:API,version:version,compare:compare,select:select,assetURL:assetURL,createChecker:createChecker};
  if(typeof module==='object'&&module.exports)module.exports=api;else root.TitanUpdates=api;
})(typeof window!=='undefined'?window:this);
// END UPDATE POLICY
