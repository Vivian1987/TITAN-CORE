"""Synthetic UI fixture only; never connects to a PS5 or downloads a payload."""
from http.server import HTTPServer
from pathlib import Path
from urllib.parse import urlparse
import importlib.util,json
spec=importlib.util.spec_from_file_location('fixture',Path(__file__).with_name('monitor-preview.py'));base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)
base.state['version']='1.0.0'
job={'phase':'idle'}
requests=[]
class Handler(base.Handler):
 def do_GET(self):
  route=urlparse(self.path).path
  if route=='/':
   body=(base.ROOT/'src/ui.html').read_text().replace('__VERSION__','1.0.0 SYNTHETIC').replace(' manifest="titan.appcache"','').replace('https://api.github.com/repos/devchmod/TITAN-CORE/releases?per_page=100','/fixture/releases')
   self.send_response(200);self.send_header('Content-Type','text/html');self.end_headers();self.wfile.write(body.encode());return
  if route=='/api/installer/destinations':
   data=[{'path':'/data','name':'Internal','free':30000000000,'external':False,'folder':'/user/app/<TITLE_ID>/app.pkg'},{'path':'/mnt/usb0','name':'USB 1','free':300000000000,'external':True,'folder':'/mnt/usb0/TITAN-CORE/Games'},{'path':'/mnt/usb1','name':'USB 2','free':500000000000,'external':True,'folder':'/mnt/usb1/TITAN-CORE/Games'}]
  elif route=='/api/installer/status':data=job
  elif route=='/fixture/releases':data=[]
  elif route=='/fixture/requests':data=requests
  else:return super().do_GET()
  self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(json.dumps(data).encode())
 def do_POST(self):
  if self.path in ['/api/installer/start','/api/settings']:
   body=self.rfile.read(int(self.headers.get('Content-Length',0))).decode();requests.append({'path':self.path,'body':body});(base.ROOT/'review/external-preview-requests.json').write_text(json.dumps(requests,indent=2))
   if self.path=='/api/installer/start':
    v=json.loads(body);job.update(phase='complete',path=('/user/app/PPSA12345/app.pkg' if v['destination']=='/data' else v['destination']+'/TITAN-CORE/Games/PPSA12345/PPSA12345.pkg'),destination=v['destination'],name='Synthetic package',done=200000000000,total=200000000000)
   self.send_response(200);self.end_headers();self.wfile.write(b'{}');return
  super().do_POST()
HTTPServer(('127.0.0.1',8769),Handler).serve_forever()
