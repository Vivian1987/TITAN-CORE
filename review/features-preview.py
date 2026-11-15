"""Synthetic loopback UI fixture: never loads a PS5 payload or requests GitHub."""
from http.server import HTTPServer
from pathlib import Path
from urllib.parse import urlparse,parse_qs
import importlib.util,json
spec=importlib.util.spec_from_file_location('fixture',Path(__file__).with_name('monitor-preview.py'));base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)
base.state['version']='0.5.8-beta.1'
for g in base.state['games']:
 if g.get('source_id')=='installed-fixture':g.update(installed_pkg=True,native_installed=True,path='/user/app/PPSA22715/app.pkg',cache_size=120000000,cache_built=1700000000)
requests=[]
class Handler(base.Handler):
 def do_GET(self):
  route=urlparse(self.path).path
  if route=='/':
   mode=parse_qs(urlparse(self.path).query).get('scenario',['available'])[0]
   s=(base.ROOT/'src/ui.html').read_text().replace('__VERSION__','0.5.8-beta.1 SYNTHETIC').replace(' manifest="titan.appcache"','').replace('https://api.github.com/repos/devchmod/TITAN-CORE/releases?per_page=100','/fixture/releases?scenario='+mode)
   self.send_response(200);self.send_header('Content-Type','text/html');self.end_headers();self.wfile.write(s.encode());return
  if route=='/api/sizes/PPSA22715':
   self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(json.dumps({'game':6000000000,'cache':120000000,'dlc':0,'save':0}).encode());return
  if route=='/fixture/releases':
   mode=parse_qs(urlparse(self.path).query).get('scenario',['available'])[0]
   if mode=='error':self.send_response(503);self.end_headers();return
   tag='v0.5.9-beta.1' if mode=='available' else 'v0.5.7-beta.1'
   data=[] if mode=='empty' else [{'tag_name':tag,'draft':False,'prerelease':True,'published_at':'2026-01-01T00:00:00Z','assets':[{'name':'TITAN-CORE.elf','size':16700000,'state':'uploaded','browser_download_url':f'https://github.com/devchmod/TITAN-CORE/releases/download/{tag}/TITAN-CORE.elf'}]}]
   self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(json.dumps(data).encode());return
  if route=='/fixture/requests':
   self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(json.dumps(requests).encode());return
  super().do_GET()
 def do_POST(self):
  if self.path=='/api/rescan' or self.path.startswith('/api/rebuild/'):
   self.rfile.read(int(self.headers.get('Content-Length',0)));requests.append(self.path);self.send_response(200);self.end_headers();return
  super().do_POST()
HTTPServer(('127.0.0.1',8768),Handler).serve_forever()
