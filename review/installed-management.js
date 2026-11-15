// BEGIN INSTALLED MANAGEMENT
var managementOpen=false,managementBusy=false,managementMessage='';
function renderInstalledManagement(g){
  var installed=!!(g&&g.installed_pkg),panel=$('installed-management');panel.hidden=!installed||!managementOpen;
  if(installed){$('m-delete').setAttribute('aria-controls','installed-management');$('m-delete').setAttribute('aria-expanded',String(managementOpen));}
  else{$('m-delete').removeAttribute('aria-controls');$('m-delete').removeAttribute('aria-expanded');}
  if(!installed)return;
  setText($('installed-engine-note'),g.native_installed?'Package kept in its installed location. Native cache is managed by TITAN CORE.':'Waiting for TITAN CORE to prepare this installed package.');
  setText($('installed-manage-message'),managementMessage,true);
  $('installed-refresh').disabled=managementBusy||!state;
}
function toggleInstalledManagement(g){
  managementOpen=!managementOpen;managementMessage='';renderInstalledManagement(g);
  if(managementOpen)$('installed-management').scrollIntoView({block:'nearest'});
}
$('installed-refresh').onclick=async function(){
  var g=openTid&&findGame(openTid);if(!g||!g.installed_pkg||managementBusy)return;
  var id=gameId(g);managementBusy=true;managementMessage='';renderInstalledManagement(g);
  try{
    var r=await fetch('/api/rescan',{method:'POST'});if(!r.ok)throw Error((await r.text())||'HTTP '+r.status);
    if(openTid===id){managementMessage=t(state&&state.paused?'Scan queued. It will resume after the game exits.':'Library scan requested.');schedule(300);}
  }catch(e){if(openTid===id)managementMessage=e.message||t('Connection error.');}
  finally{managementBusy=false;var current=openTid&&findGame(openTid);if(current)renderInstalledManagement(current);}
};
$('installed-diagnostics').onclick=function(){closeModal();setView('diagnostics');};
// END INSTALLED MANAGEMENT
