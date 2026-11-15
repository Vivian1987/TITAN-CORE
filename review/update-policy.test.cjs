const {test}=require('node:test');const assert=require('node:assert/strict');const u=require('./update-policy.js');
function release(tag,overrides={}){return{tag_name:tag,draft:false,prerelease:tag.includes('-'),published_at:'2026-01-01T00:00:00Z',body:'Notes',assets:[{name:'TITAN-CORE.elf',size:100,state:'uploaded',browser_download_url:`https://github.com/devchmod/TITAN-CORE/releases/download/${tag}/TITAN-CORE.elf`}],...overrides};}
const response=(data,status=200)=>({ok:status===200,status,json:async()=>data});
test('semantic versions compare numerical components and prerelease identifiers',()=>{
 assert.equal(u.compare('0.5.10','0.5.9'),1);assert.equal(u.compare('v0.5.8-beta.10','0.5.8-beta.2'),1);assert.equal(u.compare('0.5.8','0.5.8-beta.99'),1);
 assert.equal(u.compare('0.5.8-beta.1+build','v0.5.8-beta.1'),0);assert.equal(u.compare('1.0.0-1','1.0.0-alpha'),-1);
});
test('malformed versions cannot claim an update',()=>{for(const x of ['preview','0.5','01.0.0','1.2.3-beta.01','1.2.3<script>','1.2.3/other'])assert.equal(u.version(x),null);});
test('published releases are ordered by version, including prereleases',()=>{assert.equal(u.select([release('v0.5.9-beta.1'),release('v0.5.8'),release('v0.5.10-beta.1')]).tag,'v0.5.10-beta.1');});
test('drafts, unpublished entries and pending assets are excluded',()=>{
 const r=release('v9.0.0');r.assets[0].state='new';assert.equal(u.select([release('v10.0.0',{draft:true}),release('v8.0.0',{published_at:null}),r]),null);
});
test('a release without an ELF cannot announce an installable update',()=>{assert.equal(u.select([release('v9.0.0',{assets:[{name:'source.zip'}]})]),null);});
test('download URLs are restricted to the official repository, tag and filename',()=>{
 for(const url of ['javascript:alert(1)','https://github.com.evil.test/devchmod/TITAN-CORE/releases/download/v9.0.0/TITAN-CORE.elf','https://github.com/other/TITAN-CORE/releases/download/v9.0.0/TITAN-CORE.elf','https://github.com/devchmod/TITAN-CORE/releases/download/v8.0.0/TITAN-CORE.elf','https://github.com/devchmod/TITAN-CORE/releases/download/v9.0.0/TITAN-CORE.elf?redirect=evil']){
  const r=release('v9.0.0');r.assets[0].browser_download_url=url;assert.equal(u.select([r]),null);
 }
});
test('checker only requests release metadata and exposes the download for a newer version',async()=>{
 const calls=[];const c=u.createChecker(async(url,opts)=>{calls.push([url,opts]);return response([release('v0.5.9-beta.1')]);});
 const s=await c.check('0.5.8-beta.1');assert.equal(s.phase,'available');assert.equal(calls.length,1);assert.equal(calls[0][0],u.API);assert.equal(calls[0][1].method,'GET');assert.equal(calls[0][1].credentials,'omit');
});
test('older and identical releases do not advertise an update',async()=>{
 const c=u.createChecker(async()=>response([release('v0.5.8-beta.1'),release('v0.5.7')]));assert.equal((await c.check('0.5.8-beta.1')).phase,'current');assert.equal((await c.check('0.5.10')).phase,'current');
});
test('no published payload is distinct from a successful current-version check',async()=>{const c=u.createChecker(async()=>response([]));assert.equal((await c.check('0.5.8')).phase,'empty');});
test('network failure and rate limiting never report no updates',async()=>{
 let c=u.createChecker(async()=>{throw Error('offline');});assert.equal((await c.check('0.5.8')).phase,'error');
 c=u.createChecker(async()=>response([],403));let s=await c.check('0.5.8');assert.equal(s.phase,'error');assert.equal(s.error,'rate-limit');assert.equal(s.release,null);
});
test('a bounded timeout leaves the checker retryable',async()=>{let c=u.createChecker(()=>new Promise(()=>{}),{timeoutMs:10});let s=await c.check('0.5.8');assert.equal(s.phase,'error');assert.equal(s.loading,false);});
test('repeated concurrent checks use a single metadata request',async()=>{let finish,count=0;const c=u.createChecker(()=>{count++;return new Promise(r=>finish=r);});let a=c.check('0.5.8'),b=c.check('0.5.8');finish(response([]));await Promise.all([a,b]);assert.equal(count,1);});
test('a result from an old console version cannot overwrite a newer check',async()=>{
 let finish;const c=u.createChecker(()=>!finish?new Promise(r=>finish=r):Promise.resolve(response([release('v0.5.9')])));
 let first=c.check('0.5.8');await c.check('0.6.0');finish(response([release('v9.0.0')]));await first;assert.equal(c.snapshot().current,'0.6.0');assert.equal(c.snapshot().phase,'current');
});
test('an unknown console version does not trigger a request or keep a pending result',async()=>{
 let finish,count=0;const c=u.createChecker(()=>{count++;return new Promise(r=>finish=r);});await c.check('preview');assert.equal(count,0);
 let p=c.check('0.5.8');await c.check(null);finish(response([release('v9.0.0')]));await p;assert.equal(c.snapshot().phase,'waiting');assert.equal(c.snapshot().release,null);
});
