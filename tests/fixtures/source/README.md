# Synthetic source corpus

All artwork is programmatically-created solid shapes/channels by this project.
No user image or downloaded character illustration is included.

- groups.psd / groups-merged.png: psd-tools1.26.0 RGB8 document,8×8 white canvas, group with red Eye layer and hidden green layer; includes original mask/channel/group records. The PNG is the independently read saved merged image, not Photoshop certification.
- rgb{8,16}-codec{1,2,3}.psd: two-pixel RGB planar data, independently encoded by psd-tools RLE/ZIP/ZIP-prediction. Expected RGBA is [255,0,128,255] twice.
- Additional PNG modes/ICC/P3, Unicode/layer flags, Lr16, alpha white matte, malformed prefixes/mutations and limits are constructed in image_tests.rs rather than opaque external fixtures.

`scripts/generate-source-fixtures.py` is an optional E-only reference regeneration tool. It writes to artifacts for review and never overwrites these golden files. Rust tests need no Python image dependency. Hashes of the committed fixture files are recorded in DEV-02 evidence.
