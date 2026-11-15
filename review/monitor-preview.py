"""Loopback synthetic fixture; no PS5 or payload execution."""
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
state={'version':'preview','busy':False,'paused':False,'running':[],'games':[{'title_id':'DEMO00001','title_name':'Preview · Folder game','kind':'folder','packing':'plain','status':'ready','version':'1.00','path':'/preview/game','size':5000000000,'cache_size':1000000000,'cache_built':1,'drive':'/data'},{'title_id':'DEMO00002','title_name':'Preview · Package game','kind':'pkg','packing':None,'status':'error','error':'Preview error for interface testing','path':'/preview/demo.pkg','size':3000000000,'cache_size':0,'drive':'/mnt/usb0'}], 'drives':[{'mount':'/data','label':'Internal SSD','total':1000000000000,'free':420000000000},{'mount':'/mnt/usb0','label':'USB drive','total':500000000000,'free':300000000000}],'cache':{'size':1000000000,'free':420000000000,'path':'/data/titan-core/caches','ampr_present':True},'log':['[PREVIEW] Synthetic data for UI testing only.']}
settings={'scanpath':['/data/homebrew'],'auto_import_installed':True,'inject_ampr':True,'game_fakelib':'emulators','backports':True,'scan_interval_seconds':15,'low_space_gb':5,'notify':True,'log':True,'debug':False}
state['telemetry']={'cpu_system_percent':32.5,'cpu_payload_percent':2.5,'cpu_system_supported':True,'cpu_payload_supported':True,'logical_cpus':16,'ram_total_bytes':17179869184,'ram_used_bytes':8589934592,'payload_peak_ram_bytes':83886080,'payload_uptime_seconds':3661}
state['engine']='hybrid'
state['services']={'mode':'combined','phase':'ready','native_ready':True,'kstuff_ready':True,'shadowmount_ready':True,'api_port':10101,'firmware':'13.60','detail':'SYNTHETIC: TITAN + kstuff + ShadowMount ready'}
state['games'].append({'source_id':'exfat-fixture','title_id':'','title_name':'Fixture EXFAT','kind':'image','image_type':'exfatfs','metadata_source':'filename','path':'/mnt/usb0/Fixture.EXFAT','size':4294967296,'status':'engine_required','drive':'/mnt/usb0'})
state['games'].append({'source_id':'image-fixture','title_id':'PPSA54321','title_name':'Fixture Mounted Image','kind':'image','image_type':'exfatfs','engine':'shadowmountplus','path':'/mnt/usb0/Mounted.exfat','size':4294967296,'status':'ready','drive':'/mnt/usb0','metadata_source':'engine'})
state['games'].append({'source_id':'installed-fixture','title_id':'PPSA22715','title_name':'Fixture Installed Game','kind':'pkg','installed_pkg':False,'native_installed':False,'installed':True,'version':'01.005.000','engine':'native','path':'/data/homebrew/titan-core/PPSA22715/PPSA22715.pkg','size':6000000000,'status':'ready','drive':'/data'})
state['games'].append({'source_id':'pending-installed','title_id':'PPSA14300','title_name':'Fixture Pending Installed','kind':'pkg','installed_pkg':True,'installed':True,'engine':'shadowmountplus','path':'/user/app/PPSA14300/app.pkg','size':1000000000,'status':'preparation_pending','drive':'/data'})
class Handler(BaseHTTPRequestHandler):
 def do_GET(self):
  route=self.path.split('?')[0]
  if route=='/api/state': body=json.dumps(state).encode(); mime='application/json'
  elif route=='/api/installer/status': body=json.dumps({'phase':'queued','done':0,'total':0}).encode(); mime='application/json'
  elif route=='/api/klog': body=json.dumps({'content':'[SYNTHETIC] kernel <fixture> log\nNo real console data.','truncated':False}).encode(); mime='application/json'
  elif route=='/api/settings': body=json.dumps(settings).encode(); mime='application/json'
  elif route.startswith(('/api/sizes/','/api/image-sizes/')): body=json.dumps({'game':86865713561,'dlc':0,'save':0,'cache':838861}).encode(); mime='application/json'
  elif route.startswith('/api/icon/'): self.send_error(404); return
  elif route=='/app-icon.png': body=(ROOT/'ps5/icon0.png').read_bytes(); mime='image/png'
  elif route=='/bg.jpg': body=(ROOT/'src/bg.jpg').read_bytes(); mime='image/jpeg'
  elif route=='/':
   body=(ROOT/'src/ui.html').read_text().replace('__VERSION__','0.5.3-beta.1 FIXTURE').replace(' manifest="titan.appcache"','')
   body=body.encode(); mime='text/html'
  else: self.send_error(404); return
  self.send_response(200); self.send_header('Content-Type',mime); self.end_headers();self.wfile.write(body)
 def do_POST(self):
  path=self.rfile.read(int(self.headers.get('Content-Length',0))).decode()
  if self.path=='/api/installer/browse':
   entries=[{'name':'USB 1','path':'/mnt/usb0','directory':True}] if not path else [{'name':'Game <test>.pkg','path':'/mnt/usb0/Game.pkg','directory':False,'size':200000000000},{'name':'Games','path':'/mnt/usb0/Games','directory':True}]
   self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(json.dumps({'path':path,'entries':entries}).encode());return
  if self.path in ['/api/installer/start','/api/installer/cancel']:
   self.send_response(200);self.end_headers();self.wfile.write(b'{}');return
  self.send_response(409);self.end_headers()
 def log_message(self,*args):pass
if __name__ == '__main__':
 HTTPServer(('127.0.0.1',8765),Handler).serve_forever()
