"""Windows-only synthetic benchmark; all outputs stay under the checkout."""
import ctypes, hashlib, json, os, struct, subprocess, time, zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / 'artifacts/dev-02-performance'
CORPUS.mkdir(exist_ok=True)
EXE = ROOT / 'src-tauri/target/release/examples/inspect_image.exe'

def chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))

def png(path, width, height, depth):
    row = b'\0' + bytes([80, 160, 240]) * (width * (depth // 8))
    codec = zlib.compressobj()
    data = b''.join(codec.compress(row) for _ in range(height)) + codec.flush()
    path.write_bytes(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width,height,depth,2,0,0,0)) + chunk(b'IDAT',data) + chunk(b'IEND',b''))

class Counters(ctypes.Structure):
    _fields_ = [('cb',ctypes.c_ulong),('faults',ctypes.c_ulong)] + [(n,ctypes.c_size_t) for n in ['peak','working','peakPaged','paged','peakNonpaged','nonpaged','pagefile','peakPagefile']]

memory = ctypes.windll.psapi.GetProcessMemoryInfo
memory.argtypes = [ctypes.c_void_p,ctypes.POINTER(Counters),ctypes.c_ulong]
results=[]
for width,height,depth in [(1024,1024,8),(4000,4000,8),(4096,4096,16),(8000,5000,8),(8001,5000,8)]:
    path=CORPUS/f'{width}x{height}-{depth}.png';png(path,width,height,depth)
    before=hashlib.sha256(path.read_bytes()).hexdigest()
    for repeat in range(2):
        started=time.perf_counter()
        proc=subprocess.Popen([str(EXE),str(path)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,creationflags=subprocess.CREATE_NO_WINDOW)
        out,err=proc.communicate(timeout=90)
        info=Counters();info.cb=ctypes.sizeof(info)
        ok=memory(ctypes.c_void_p(int(proc._handle)),ctypes.byref(info),ctypes.sizeof(info))
        report=json.loads(out)
        results.append(dict(width=width,height=height,depth=depth,file_bytes=path.stat().st_size,repeat=repeat,wall_ms=round((time.perf_counter()-started)*1000),peak_working_set_bytes=info.peak if ok else None,returncode=proc.returncode,report=report))
        assert hashlib.sha256(path.read_bytes()).hexdigest()==before
    print(f'Measured {width}x{height}/{depth}',flush=True)
(ROOT/'artifacts/dev-02-performance.json').write_text(json.dumps(results,indent=2),encoding='utf-8')
print(json.dumps([dict(width=r['width'],depth=r['depth'],repeat=r['repeat'],peak=r['peak_working_set_bytes'],ms=r['wall_ms'],code=r['report'].get('code')) for r in results]))

def section(data): return struct.pack('>I',len(data))+data
layer_results=[]
for count in [1,128,1024,1025]:
    name=b'Layer';pascal=bytes([len(name)])+name+bytes((4-(len(name)+1)%4)%4)
    extra=bytes(8)+pascal
    record=struct.pack('>iiiiHhI',0,0,1,1,1,0,3)+b'8BIMnorm'+bytes([255,0,0,0])+section(extra)
    info=struct.pack('>h',count)+record*count+b'\0\0\x80'*count
    info+=bytes(len(info)%2)
    layers=section(section(info)+bytes(4))
    header=b'8BPS'+struct.pack('>H',1)+bytes(6)+struct.pack('>HIIHH',3,1,1,8,3)+bytes(8)
    file=CORPUS/f'layers-{count}.psd';file.write_bytes(header+layers+b'\0\0\xff\x80\0')
    before=hashlib.sha256(file.read_bytes()).hexdigest();started=time.perf_counter()
    proc=subprocess.Popen([str(EXE),str(file)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,creationflags=subprocess.CREATE_NO_WINDOW)
    out,err=proc.communicate(timeout=90);info=Counters();info.cb=ctypes.sizeof(info)
    ok=memory(ctypes.c_void_p(int(proc._handle)),ctypes.byref(info),ctypes.sizeof(info))
    report=json.loads(out);layer_results.append(dict(layers=count,file_bytes=file.stat().st_size,wall_ms=round((time.perf_counter()-started)*1000),peak_working_set_bytes=info.peak if ok else None,returncode=proc.returncode,parsed_layers=len(report.get('layers',[])),code=report.get('code')))
    assert hashlib.sha256(file.read_bytes()).hexdigest()==before
(ROOT/'artifacts/dev-02-layer-performance.json').write_text(json.dumps(layer_results,indent=2),encoding='utf-8')
print(json.dumps(layer_results))
