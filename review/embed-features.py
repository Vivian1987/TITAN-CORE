from pathlib import Path
root=Path(__file__).resolve().parents[1]
p=root/'src/ui.html';s=p.read_text();a=s.index('// BEGIN TITAN FEATURES');b=s.index('// END TITAN FEATURES',a)+len('// END TITAN FEATURES')
files=['update-policy.js','updates-client.js','installed-management.js']
p.write_text(s[:a]+'// BEGIN TITAN FEATURES\n'+'\n'.join((root/'review'/n).read_text().strip() for n in files)+'\n// END TITAN FEATURES'+s[b:])
