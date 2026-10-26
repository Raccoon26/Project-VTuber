# Compatibility matrix

조사일 2026-10-25 JST. 모든 행의 **우리 앱 exporter는 미구현**, 프로그램 내 load/표정/물리/재실행 검증은 **미실행**이다. 공식 문서에 대상 포맷이 존재하는 것과 우리 산출물 호환성은 다르다. 고급 표현은 공통 마스터에 남기고 exporter가 손실 보고를 만든다.

| 대상 | 조사된 전달 경로 | 표정·물리/제약과 손실 | 정적 / runtime 증거·다음 DEV |
|---|---|---|---|
| VTube Studio | Cubism moc3/model3.json/texture/physics bundle | Cubism 대상 SDK 버전 고정, custom input mapping 필요; cmo3 편집 정보는 runtime에 없음 | profile만 / 미실행; 11,24 |
| VBridger + VTS | 위 bundle + creator preset/mapping | 고급 입·눈 비대칭 keyforms 필요; preset 호환성을 별도 확인 | synthetic binding 검사 / 미실행; 11,24 |
| VBridger + VMC | ARKit 호환 VRM + receiver | blendshape clip 이름 일치; VBridger VMC는 머리·안구 회전/몸을 보내지 않음 | profile만 / 미실행; 24 |
| VSeeFace | VRM **0.x** 또는 VSFAvatar | 공식 문서 VRM1 불가; ARKit 및 VMC 수신 가능, custom shader/Unity 확장은 VSFAvatar 별도 | profile만 / 미실행; 23,24 |
| Warudo | VRM / Unity Mod SDK `.warudo` | VRM/사용자 표정, 앱 물리·blueprint 의존성; shader/mod 별도 | profile만 / 미실행; 23,24 |
| VNyan | VRM0/VRM1/VSFAvatar | ARKit/VMC, node graph 및 앱 물리 확장은 공통 포맷 밖 | profile만 / 미실행; 23,24 |
| Inochi Session | INP, 편집 원본 INX 계열 | Inochi parameter/physics; Cubism deformer 변환을 가정하지 않음 | profile만 / 미실행; 07–11,24 |
| PrprLive | Live2D runtime, 정확한 Cubism 버전 재확인 필요 | 개발자 업데이트의 모델 load 이슈 존재; 현재 유통·고급 표정 지원 제한 미확인 | 미확인 / 미실행; 24 |
| VRChat | Unity + Avatar SDK build/test, 계정/업로드 별도 | VRM 직접 업로드와 다름; humanoid/viseme/PhysBones/performance rank, PC/모바일 제한 | profile만 / 미실행; 23,24 |
| GLB | glTF2 mesh/skin/morph | VRM 표정 의미·springbone·앱 전용 shader 손실 | profile만 / 미실행; 23 |
| FBX/Unity 일반 | Blender export 및 대상 Unity importer | morph/축/단위/material/physics 재현 여부 개별 검증 | 설계만 / 미실행; 23 |
| Animaze/3tene/Luppet 등 | 추가 조사 대상, 추정한 포맷으로 인증하지 않음 | 앱·버전·확장 조건 조사 필요 | Planned / 미실행; 24 |

근거: [VTS API](https://github.com/DenchiSoft/VTubeStudio), [VBridger](https://store.steampowered.com/app/1898830/VBridger/), [VSeeFace 공식](https://www.vseeface.icu/), [Warudo character](https://docs.warudo.app/docs/assets/character), [VNyan 개발자](https://suvidriel.itch.io/vnyan), [Inochi2D](https://github.com/Inochi2D/inochi2d), [PrprLive 개발자 공지](https://store.steampowered.com/news/app/1279610/view/3709323430566127422), [VRChat 제작](https://creators.vrchat.com/avatars/creating-your-first-avatar/), [glTF spec](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html).

## 인증 보고서 필수 필드 (후속 구현)
profile ID/version, exporter version, source/master/export hashes, 대상 앱·plugin·OS/GPU 버전, 검증한 channel/pose/physics, 알려진 손실, 시험 동영상·로그, 검토자, 시각, pass/fail/unverified/not_applicable. 단순 파일 열기는 통과 조건의 일부일 뿐이다. 미지원 기능은 경고와 대체 경로를 보여 주고 사용자가 선택한 필수 기능이 없으면 정식 export를 차단한다.
