# Monetization design

핵심 기능은 무료. 앱 최하단 배너 **최대 1개**. popup/interstitial/forced video 금지. 네트워크·계약·광고 응답 실패 시 배너를 숨기고 프로젝트·GPU 작업은 계속한다. DEV-00은 계약/계정/SDK 설치를 하지 않았고 `OfflineAdvertisementAdapter`만 제공한다.

| 방식 | 데스크톱 적합성 근거 | 일본·지급 조건 | 개인정보/배포/오프라인 | 결정 |
|---|---|---|---|---|
| 직접 스폰서 정적 이미지/문구 | 당사자 계약으로 desktop 표시권·기간·지역·소재권을 명시 가능 | 일본 노출/통화/세금/지급 일정은 계약으로 확정; 현재 미계약 | 로컬 캐시+수동 링크, 추적 픽셀 금지. 외부 SDK 불필요. 만료 후 숨김 | 우선 검토, 현재 활성화 없음 |
| Pubfinity | 공식 Windows 앱용 banner/native/video SDK 소개 | 일본 inventory·publisher 자격·최소 지급액·정산 주기 공개 근거 확보 안 됨 | Tauri/Win32 bridge 지원·수집 항목·DPA·재배포·캐시 정책 확인 필요 | 후보; 파트너/출시 가능으로 표시 금지 |
| Google AdSense 웹 삽입 | 공식 정책이 일반 software application 통합 금지 | 일본 지원 여부와 무관하게 이 경로 부적합 | WebView에 광고 페이지를 감싸 정책 우회하지 않음 | 이 경로 Rejected with Reason |
| Microsoft UWP Advertising | 공식 문서에 기존 플랫폼 종료 경고 | 신규 수익 기반으로 사용 불가 | UWP SDK 문서 존재가 현행 서비스 증거가 아님 | 기각 |

근거: [Pubfinity 공식](https://pubfinity.com/), [Pubfinity privacy](https://pubfinity.com/privacy-policy/), [AdSense 정책](https://support.google.com/adsense/answer/48182?hl=en), [Microsoft 종료 안내 포함 문서](https://learn.microsoft.com/en-us/windows/uwp/monetize/install-the-microsoft-advertising-libraries).

## Adapter 경계
입력은 locale/광고 영역/동의 상태만, 프로젝트 ID/이미지/파일 경로/GPU 장치 식별자/작업 내용은 금지. DEV-00 인터페이스는 `banner(locale) → text/sponsor|null`, `dispose()`이고 기본값 null이다. 이미지·URL을 받을 실제 구현은 검증된 mime/크기, 허용 URL scheme/host, 명시 click만 시스템 브라우저 열기를 요구한다.

실제 SDK는 core WebView와 같은 origin에 주입하지 않는다. 별도 origin/process, project IPC 권한 없음, GPU inference 스케줄러 접근 없음, 2초 deadline/circuit breaker, 배너 높이 제한, offline cache expiry를 설계한다. 동적 JavaScript 창작물보다 정적 소재를 우선한다. 이 격리와 timeout은 아직 구현되지 않았으므로 실제 네트워크 adapter를 연결하지 않는다.

DEV-27 도입 전 필요한 문서: desktop/Tauri 배포 서면 허용, 일본 서비스와 정산 조건, 앱 콘텐츠 승인, 데이터 항목/보관/제3자/국외 이전/동의, SDK 라이선스·SBOM·네트워크 캡처, offline/failure 시험. 계약/계정/결제는 사용자 승인 없이 진행하지 않는다.
