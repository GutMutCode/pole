# Pole 하이브리드 전략: Python + Rust

> Phase 5 최적화된 언어 선택 전략

## 개요

Pole 프로젝트는 **Python과 Rust의 하이브리드 아키텍처**를 채택합니다. 각 언어를 최적의 용도에 맞게 사용하여 개발 속도와 실행 성능을 동시에 달성합니다.

## 원칙

**"올바른 도구를 올바른 곳에 (Right tool for the right job)"**

- **Python**: 빠른 개발, 사용자 인터페이스, 외부 API 연동
- **Rust**: 성능 critical, CPU 집약적, 메모리 안전성 필수

---

## 컴포넌트별 언어 선택

### ✅ Python 유지 (404줄 + 522줄 = 926줄)

#### 1. **LLM Transformer** (404줄)
- `src/pole/transformer/llm_transformer.py` (297줄)
- `src/pole/transformer/llm_client.py` (107줄)

**이유:**
- ❌ Rust로 전환해도 성능 이득 없음 (네트워크 I/O가 병목)
- ✅ Python SDK가 훨씬 간편 (anthropic, openai)
- ✅ API 변경 빈번 → Python이 유연

#### 2. **CLI** (265줄)
- `src/pole/cli/main.py`

**이유:**
- ❌ 사용자 경험에 큰 영향 없음
- ✅ Python이 스크립팅에 더 적합
- ✅ 빠른 수정 및 배포

#### 3. **Spec Parser** (256줄)
- `src/pole/parser/spec_parser.py`

**이유:**
- ⚠️ 성능 이득 있지만 우선순위 낮음
- ✅ LLM 변환이 훨씬 느림 (파싱은 병목 아님)
- ⚠️ Phase 6+ 이후 재검토

#### 4. **기타** (1줄)
- `src/pole/validator/spec_validator.py` (166줄)
- `src/pole/verifier/example_tester.py` (139줄)
- `src/pole/transformer/ir_postprocessor.py` (125줄)

**총 Python 유지:** ~1,356줄

---

### ⭐ Rust 전환 (1,255줄 → Rust)

#### 1. **IR Parser** (498줄) - **M0 최우선**
- `src/pole/runtime/ir_parser.py` → `compiler/src/ir_parser.rs`

**이유:**
- ✅ **10-100배 성능 향상** (텍스트 파싱은 CPU-bound)
- ✅ **메모리 효율** (Python 객체 오버헤드 제거)
- ✅ **Phase 5-10 모두 사용** (컴파일러, 인터프리터, 검증기)
- ✅ **외부 의존성 없음** (순수 파싱 로직)

**구현:**
- nom parser combinator 사용
- PyO3로 Python 바인딩
- 성능 목표: 5ms → <0.5ms

**ROI:** ⭐⭐⭐⭐⭐ (최고)

---

#### 2. **Type Checker** (379줄) - **M0 강력 추천**
- `src/pole/verifier/type_checker.py` → `compiler/src/type_checker.rs`

**이유:**
- ✅ **타입 안전성** (Rust 타입 시스템으로 타입 체커 자체를 검증)
- ✅ **5-20배 성능** (대규모 코드베이스에서 효과)
- ✅ **Phase 6+ 필수** (컴파일 타임 타입 체킹)
- ✅ **Rust의 강점** (enum, pattern matching, ADT)

**구현:**
- Rust enum으로 타입 표현
- pattern matching으로 타입 추론
- 컴파일러가 모든 케이스 체크 강제

**ROI:** ⭐⭐⭐⭐⭐ (최고)

---

#### 3. **Contract Verifier** (145줄) - **M1 추천**
- `src/pole/verifier/contract_verifier.py` → `compiler/src/contract_verifier.rs`

**이유:**
- ✅ **Zero-cost abstractions** (컴파일 타임 최적화)
- ✅ **Phase 6+ 필수** (컴파일 타임 계약 검증)
- ✅ **Type Checker와 통합** (Rust로 함께 구현)

**구현:**
- requires/ensures 검증
- 런타임 오버헤드 최소화

**ROI:** ⭐⭐⭐⭐

---

#### 4. **Interpreter** (233줄) - **M3 선택사항**
- `src/pole/runtime/interpreter.py` → `compiler/src/interpreter.rs`

**이유:**
- ✅ **100배+ 성능** (~0.06ms → <0.001ms)
- ⚠️ **Phase 5 이후 사용도 감소** (네이티브 컴파일 사용)
- ✅ **빠른 테스트 실행** (개발 중 유용)

**판단:**
- Phase 5 이전: Python 유지
- Phase 5 이후: 선택적 Rust 전환

**ROI:** ⭐⭐⭐

---

## 타임라인

### Phase 5 M0 (3개월, 2025-10 ~ 2026-01)

**Week 1-6: Rust 기초**
- The Rust Book
- Rust by Example
- 소유권, 빌림, 생명주기

**Week 7-10: IR Parser Rust 구현** ⭐
- nom parser combinator 학습
- `compiler/src/ir_parser.rs` 구현
- PyO3 바인딩
- 성능 벤치마크

**Week 11-14: Type Checker Rust 구현** ⭐
- `compiler/src/type_checker.rs` 구현
- Rust enum + pattern matching
- 타입 추론 알고리즘
- 성능 벤치마크

**Week 15-18: LLVM 준비 & 통합 검증**
- llvm-sys 학습
- PyO3 통합 완성
- 전체 시스템 테스트

---

### Phase 5 M1-M4 (9개월, 2026-01 ~ 2026-09)

**M1 (3개월):**
- LLVM 백엔드 구현
- Contract Verifier Rust 전환 (선택)

**M2-M3 (4개월):**
- 제어 흐름, 재귀 함수 컴파일
- Interpreter Rust 전환 검토 (선택)

**M4 (2개월):**
- 전체 통합 및 최적화

---

## 성능 목표

| 컴포넌트 | Python (현재) | Rust (목표) | 향상 배수 |
|---------|-------------|------------|----------|
| IR Parser | ~5ms | <0.5ms | 10배+ |
| Type Checker | ~10ms | <2ms | 5배+ |
| Contract Verifier | ~5ms | <0.5ms | 10배+ |
| Interpreter | 0.06ms | <0.001ms | 60배+ |
| Compiler (전체) | N/A | <0.001ms | 100배+ |

---

## 기술 스택

### Rust
- **언어**: Rust 1.75+ (2024 Edition)
- **파서**: nom 7.0+
- **LLVM**: llvm-sys 17.0+ / inkwell 0.4+
- **Python 연동**: PyO3 0.20+
- **빌드**: Cargo + maturin
- **벤치마크**: criterion

### Python
- **언어**: Python 3.11+
- **LLM API**: anthropic, openai
- **CLI**: 표준 argparse
- **테스트**: pytest

---

## ROI 분석

| 컴포넌트 | 줄 수 | 전환 시간 | 성능 향상 | Phase 5+ 중요도 | ROI |
|---------|------|---------|----------|---------------|-----|
| **IR Parser** | 498 | 2-3주 | 10-100배 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Type Checker** | 379 | 3-4주 | 5-20배 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Contract Verifier** | 145 | 1-2주 | 5-10배 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Interpreter** | 233 | 2-3주 | 100배+ | ⭐⭐⭐ | ⭐⭐⭐ |
| LLM Transformer | 404 | 4-6주 | 없음 | ⭐⭐⭐⭐⭐ | ❌ |
| CLI | 265 | 2-3주 | 미미 | ⭐⭐⭐ | ❌ |
| Spec Parser | 256 | 2-3주 | 10배 | ⭐⭐ | ⚠️ |

---

## 리스크 관리

### Python 버전 백업
- Python 구현 코드 보존 (git branch)
- feature flag로 Python/Rust 선택 가능
- 롤백 가능성 유지

### 단계적 전환
1. Rust 구현 완성
2. 병렬 테스트 (Python vs Rust)
3. 성능 검증
4. 점진적 교체 (feature flag)
5. Python 코드 보존 (백업)

### 테스트 전략
- 기존 Python 테스트 100% 통과
- Rust 통합 테스트 추가
- 성능 회귀 테스트 (criterion)
- 메모리 누수 테스트 (valgrind)

---

## 결론

**하이브리드 전략의 장점:**
- ✅ 개발 속도 유지 (Python으로 빠른 프로토타이핑)
- ✅ 실행 성능 확보 (Rust로 critical path 최적화)
- ✅ 생태계 활용 (Python LLM SDK + Rust LLVM)
- ✅ 리스크 분산 (Python 백업 유지)

**Phase 5 성공 기준:**
- IR Parser, Type Checker Rust 전환 완료
- 10배+ 성능 향상 달성
- 모든 기존 테스트 통과
- Python-Rust 통합 안정화

**장기 비전:**
- Phase 6+: 추가 컴포넌트 Rust 전환 (선택적)
- Phase 7-10: 게임 표준 라이브러리 Rust 구현
- 최종: Rust 70% + Python 30% (추정)
