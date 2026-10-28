"""Optional E-only reference generator. Product/CI tests use checked-in fixtures.

Use psd-tools==1.26.0 and attrs==25.4.0 in .tools/psd-reference;
Pillow/numpy are the existing developer Python libraries. Never parse user art here.
"""
import struct, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'.tools/psd-reference'))
from PIL import Image
from psd_tools import PSDImage
from psd_tools.api.layers import Group,PixelLayer
from psd_tools.compression import compress
from psd_tools.constants import Compression

def main():
    # Generate separately for review; never overwrite checked-in goldens automatically.
    output=ROOT/'artifacts/dev-02-regenerated'
    output.mkdir(exist_ok=True)
    doc=PSDImage.new('RGB',(8,8),color=(255,255,255))
    group=Group.new(doc,'Group')
    PixelLayer.frompil(Image.new('RGBA',(4,4),(255,0,0,128)),group,'Eye',left=1,top=2)
    hidden=PixelLayer.frompil(Image.new('RGBA',(2,2),(0,255,0,255)),doc,'Hidden')
    hidden.visible=False
    doc.save(output/'groups.psd')
    PSDImage.open(output/'groups.psd').topil(apply_icc=False).save(output/'groups-merged.png')
    for depth in [8,16]:
        raw=b''.join(bytes([v]) if depth==8 else struct.pack('>H',v*257) for v in [255,255,0,0,128,128])
        for codec in [1,2,3]:
            header=b'8BPS'+struct.pack('>H',1)+bytes(6)+struct.pack('>HIIHH',3,1,2,depth,3)+bytes(12)
            (output/f'rgb{depth}-codec{codec}.psd').write_bytes(header+struct.pack('>H',codec)+compress(raw,Compression(codec),2,3,depth))
    print('Generated only synthetic reference files under E project artifacts. Review pixels/structure before replacing goldens.')
if __name__=='__main__':main()
