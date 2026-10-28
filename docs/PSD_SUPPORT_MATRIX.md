# DEV-02 PSD support matrix

Facts are scoped to bounded PSD v1 RGB/Grayscale8/16 documents with an embedded merged image. Original PSD bytes, resources, masks, effects and opaque Photoshop data remain intact in the original blob. Byte preservation does not certify editable interpretation or Photoshop rendering parity.

| Feature | Implementation | Limit / validation |
|---|---|---|
| Header/canvas/mode/depth | Read with checked section bounds | PSD v1 only, bounded dimensions |
| Layer names/order/positions/size/visible/opacity/blend/clipping | Original record metadata and byte ranges | Stored bottom-to-top; Unicode luni overrides legacy Macintosh name; non-ASCII legacy fallback is explicitly warned when producer encoding may differ |
| Groups | lsct/lsdk markers and validated nesting | 64 levels; structural records, not compositor |
| Channels/alpha | Channel IDs and original channel ranges; raw/RLE/ZIP/ZIP-prediction validation | RGB/gray8/16, per-row bounds |
| Layer mask | Presence and record/channel ranges preserved | Mask rendering/complex coordinates not certified |
| ICC | Resource1039 extracted and hashed | RGB ICC converted by LittleCMS; gray ICC currently refused |
| Composite preview | Embedded merged image decoded separately | Not a newly flattened replacement for original layers |
| Normal/special blend, opacity, clipping, adjustment/effects/smart objects | Original record preserved | No editable layer recomposition is claimed; cached composite may include author's baked appearance |
| Linked smart objects | Opaque bytes only | No filesystem/network links opened, no code executed |
| 16-bit layer extension Lr16 | Read as a separate bounded layer-info block | Synthetic Lr16 regression; Photoshop runtime still unverified |
| PSB/32-bit/CMYK/Lab/indexed PSD | Explicitly unsupported | Original external file is untouched; no success/certification |

A successful import certifies only the byte/hash and stated parsing scope. Missing/unsupported composite data produces an explicit error. A PSD containing effects can use its saved composite without claiming that changing the underlying effect/layer will render correctly. Photoshop itself is not installed or used as a licensed reference, so no Photoshop parity certification is issued.

Primary format source: [Adobe PSD specification](https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/). Independent reader/compressor for synthetic tests: [psd-tools documentation](https://psd-tools.readthedocs.io/en/latest/). Reference fixtures contain only programmatically-created shapes, no user illustration.
