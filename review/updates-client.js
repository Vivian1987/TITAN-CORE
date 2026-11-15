// BEGIN UPDATES CLIENT
var updateChecker=TitanUpdates.createChecker(function(url,options){return fetch(url,options);},{onchange:function(){renderUpdateStatus();}});
var updateAutoInterval=30*60*1000;
function updateVersion(){return state&&typeof state.version==='string'?state.version:null;}
function updateLabel(s){return s.phase==='available'?'Update available':s.phase==='checking'?'Checking updates…':s.phase==='current'?'No newer update':s.phase==='error'?'Check unavailable':'Updates';}
function renderUpdateStatus(){
  var s=updateChecker.snapshot();setText($('update-label'),updateLabel(s));$('update-dot').hidden=s.phase!=='available';
  $('update-check').classList.toggle('has-update',s.phase==='available');$('update-check').setAttribute('title',t(updateLabel(s)));
  if(!$('utility-dialog').hidden&&$('utility-dialog').dataset.kind==='updates')renderUpdateBody();
}
function maybeCheckUpdates(online){
  var current=updateVersion(),s=updateChecker.snapshot();renderUpdateStatus();
  if(online&&TitanUpdates.version(current)&&!s.loading&&(current!==s.current||!s.lastAttempt||Date.now()-s.lastAttempt>=updateAutoInterval))updateChecker.check(current);
}
function openUpdates(){
  if(modalOpen())closeModal();utilityOpener=document.activeElement;$('utility-dialog').dataset.kind='updates';$('utility-dialog').hidden=false;$('utility-body')._updateSig=null;
  setText($('utility-title'),'TITAN CORE updates');renderUpdateBody();syncInert();$('utility-close').focus();
  var s=updateChecker.snapshot();if(!s.lastAttempt||s.current!==updateVersion())updateChecker.check(updateVersion());
}
function renderUpdateBody(){
  var s=updateChecker.snapshot(),body=$('utility-body'),signature=JSON.stringify([s.phase,s.current,s.checkedAt,s.error,s.release&&s.release.tag,updateVersion(),locale]);if(body._updateSig===signature)return;body._updateSig=signature;var hadFocus=body.contains(document.activeElement);body.textContent='';setText($('utility-title'),'TITAN CORE updates');
  body.appendChild(el('p','utility-lead',t('Current version')+': '+(updateVersion()||'—')));
  body.appendChild(el('h3',null,t(updateLabel(s))));
  var message=s.phase==='available'?'A newer published ELF is available.':s.phase==='current'?'No newer published ELF was found.':s.phase==='empty'?'No published ELF is available yet.':s.phase==='error'?(s.error==='rate-limit'?'GitHub request limit reached. Try again later.':'Could not check GitHub. Check your connection and try again.'):'Checks published releases on the official TITAN CORE repository.';
  body.appendChild(el('p',null,t(message)));
  if(s.release&&s.phase==='available'){
    body.appendChild(el('p','mono',s.release.tag));if(s.release.prerelease)body.appendChild(el('p','update-beta',t('Pre-release')));
    var links=el('div','btns'),download=el('a','button primary',t('Download ELF'));download.href=s.release.url;download.target='_blank';download.rel='noopener noreferrer';links.appendChild(download);
    var notes=el('a','button',t('Release notes'));notes.href=s.release.page;notes.target='_blank';notes.rel='noopener noreferrer';links.appendChild(notes);body.appendChild(links);
    body.appendChild(el('p','language-note',t('Download the ELF, close games, then load it with your usual ELF loader.')));
  }
  if(s.checkedAt)body.appendChild(el('p','language-note',t('Last checked')+': '+new Date(s.checkedAt).toLocaleString()));
  var check=el('button',null,t(s.loading?'Checking updates…':'Check again'));check.type='button';check.disabled=s.loading||!TitanUpdates.version(updateVersion());check.onclick=function(){updateChecker.check(updateVersion());};body.appendChild(check);
  if(hadFocus)$('utility-close').focus();
  if(!TitanUpdates.version(updateVersion()))body.appendChild(el('p',null,t('Connect to the console to identify the current version.')));
}
$('update-check').onclick=openUpdates;
// END UPDATES CLIENT
