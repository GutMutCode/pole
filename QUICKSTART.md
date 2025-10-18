# Pole 빠른 시작 가이드

> 사용자 입장에서 Pole을 처음 사용하는 방법

## 1. 프로젝트 디렉토리로 이동

```bash
cd /home/gmc/Devs/pole
```

## 2. 환경 활성화

**NixOS/direnv 사용자:**
```bash
# direnv가 자동으로 환경을 로드합니다
# 처음 한 번만 허용 필요:
direnv allow

# 이후부터는 디렉토리 진입 시 자동 활성화됨
```

**또는 nix-shell 직접 사용:**
```bash
nix-shell
# ✓ Pole environment loaded 메시지가 나타남
```

## 3. 기본 사용법 테스트

### 3-1. 예제 파일 확인
```bash
# 명세 언어 파일 보기 (사람이 작성)
cat examples/01-factorial.pole

# IR 파일 보기 (LLM이 생성)
cat examples/01-factorial.pole-ir
```

### 3-2. 명세 파일 검증
```bash
pole check examples/01-factorial.pole
# ⚠ Validation passed with warnings (정상)
```

### 3-3. IR 함수 실행
```bash
# factorial(5) 실행
pole run examples/01-factorial.pole-ir factorial 5
# 출력: Result: 120

# factorial(10) 실행
pole run examples/01-factorial.pole-ir factorial 10
# 출력: Result: 3628800
```

### 3-4. 테스트 실행
```bash
pole test examples/01-factorial.pole-ir
# 출력: Total: 3, Passed: 3 (100.0%)
```

## 4. 다른 예제 시도

### Fibonacci
```bash
cat examples/02-fibonacci.pole
pole run examples/02-fibonacci.pole-ir fib 10
```

### User Validation
```bash
cat examples/03-user-validation.pole
pole check examples/03-user-validation.pole
```

## 5. 전체 명령어 목록

```bash
pole --help
```

또는:

- `pole check <file>` - 명세 파일 검증
- `pole build <file>` - 명세 → IR 변환 (LLM API 필요)
- `pole run <ir-file> <function> [args...]` - IR 함수 실행
- `pole test <ir-file>` - IR 테스트 실행

## 6. 문제 해결

### pole 명령어를 찾을 수 없다면?

**direnv 사용 중:**
```bash
direnv allow
# 또는
cd /home/gmc/Devs/pole  # 디렉토리 재진입
```

**nix-shell 사용 중:**
```bash
nix-shell  # shell에 진입했는지 확인
```

### Python 버전 확인
```bash
python --version  # Python 3.11.14 이상
```

### PYTHONPATH 확인
```bash
echo $PYTHONPATH  # /home/gmc/Devs/pole/src 포함되어야 함
```

## 7. 다음 단계

- 📖 [README.md](README.md) - 프로젝트 전체 개요
- 🏗️ [ARCHITECTURE.md](ARCHITECTURE.md) - 시스템 구조
- 📝 [specs/syntax-v0.md](specs/syntax-v0.md) - 명세 언어 문법
- 🗺️ [ROADMAP.md](ROADMAP.md) - 개발 로드맵

---

**즐거운 Pole 사용 되세요!** 🚀
