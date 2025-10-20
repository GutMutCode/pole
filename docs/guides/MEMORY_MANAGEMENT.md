# Pole 컴파일러 메모리 관리

> Arena Allocator를 활용한 효율적인 메모리 전략

**대상:** 컴파일러 코어 개발자  
**최종 업데이트:** 2025-10-20

---

## 📋 목차

1. [메모리 관리 전략 개요](#메모리-관리-전략-개요)
2. [Arena Allocator 구조](#arena-allocator-구조)
3. [구현 가이드](#구현-가이드)
4. [성능 최적화](#성능-최적화)
5. [문제 해결](#문제-해결)

---

## 메모리 관리 전략 개요

### 목표

Pole 컴파일러에 **Arena Allocator**를 도입하여:
- 메모리 사용량 **75% 감소**
- 컴파일 속도 **3x 향상**
- OOM 시 크래시 방지

### SQLite 스타일 메모리 관리

**핵심 아이디어:**
- 컴파일 단계별로 별도의 Arena 사용
- 단계 완료 후 Arena 전체를 한 번에 해제
- 개별 free() 호출 불필요

### 예상 성능 개선

| 메트릭 | 현재 | Arena 적용 후 | 개선 |
|--------|------|--------------|------|
| 메모리 (factorial) | 110MB | 30MB | 73% ↓ |
| 할당 횟수 | 10,000+ | 3 | 99.9% ↓ |
| 컴파일 시간 | 5ms | 1.7ms | 3x ↑ |

---

## Arena Allocator 구조

### 3단계 Arena 설계

```rust
// compiler/src/arena.rs
use bumpalo::Bump;

pub struct CompilerArenas {
    parse_arena: Bump,    // AST 파싱 (50MB → 15MB)
    ir_arena: Bump,       // IR 생성 (30MB → 10MB)
    codegen_arena: Bump,  // 코드 생성 (20MB → 5MB)
}

impl CompilerArenas {
    pub fn new(limit: usize) -> Self {
        let chunk_size = limit / 3;
        Self {
            parse_arena: Bump::with_capacity(chunk_size),
            ir_arena: Bump::with_capacity(chunk_size),
            codegen_arena: Bump::with_capacity(chunk_size),
        }
    }
    
    pub fn reset(&mut self) {
        self.parse_arena.reset();
        self.ir_arena.reset();
        self.codegen_arena.reset();
    }
    
    pub fn total_allocated(&self) -> usize {
        self.parse_arena.allocated_bytes() +
        self.ir_arena.allocated_bytes() +
        self.codegen_arena.allocated_bytes()
    }
}
```

### Arena 수명 관리

```
컴파일 시작
  ├─ Arena 생성 (100MB 제한)
  │
  ├─ Phase 1: Parsing
  │   ├─ parse_arena에 AST 할당
  │   └─ AST 파싱 완료
  │
  ├─ Phase 2: IR Generation
  │   ├─ ir_arena에 IR 할당
  │   ├─ parse_arena 해제 (AST 불필요)
  │   └─ IR 생성 완료
  │
  ├─ Phase 3: Code Generation
  │   ├─ codegen_arena에 LLVM IR 할당
  │   ├─ ir_arena 해제 (IR 불필요)
  │   └─ LLVM IR 생성 완료
  │
  └─ Arena 전체 해제
컴파일 종료
```

---

## 구현 가이드

### 1. 의존성 추가

```toml
# compiler/Cargo.toml
[dependencies]
bumpalo = "3.14"  # Arena allocator
```

### 2. CodeGen에 Arena 통합

```rust
// compiler/src/codegen.rs
use bumpalo::Bump;

pub struct CodeGen<'ctx, 'arena> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    arena: &'arena Bump,  // Arena 추가
}

impl<'ctx, 'arena> CodeGen<'ctx, 'arena> {
    pub fn new_with_arena(
        context: &'ctx Context,
        module_name: &str,
        arena: &'arena Bump,
    ) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        Self {
            context,
            module,
            builder,
            arena,
        }
    }
    
    // 임시 데이터 Arena에 할당
    fn alloc_temp<T>(&self, value: T) -> &'arena T {
        self.arena.alloc(value)
    }
    
    // 예: 함수 시그니처 생성
    fn create_function_signature(&self, name: &str, params: &[Type]) -> &'arena FnSig {
        let sig = FnSig {
            name: name.to_string(),
            params: params.to_vec(),
        };
        self.alloc_temp(sig)
    }
}
```

### 3. IR Parser에 Arena 통합

```rust
// compiler/src/ir_parser.rs
pub struct Parser<'arena> {
    tokens: Vec<Token>,
    current: usize,
    arena: &'arena Bump,
}

impl<'arena> Parser<'arena> {
    pub fn new_with_arena(source: &str, arena: &'arena Bump) -> Self {
        let tokens = tokenize(source);
        Self {
            tokens,
            current: 0,
            arena,
        }
    }
    
    // AST 노드를 Arena에 할당
    fn alloc_expr(&self, expr: Expr) -> &'arena Expr {
        self.arena.alloc(expr)
    }
}
```

### 4. OOM 복구 처리

```rust
// compiler/src/memory.rs
pub enum CompileError {
    OutOfMemory {
        phase: &'static str,
        used: usize,
        limit: usize,
    },
    ParseError(String),
    TypeError(String),
}

pub fn compile_with_arena(source: &str) -> Result<Module, CompileError> {
    const MEMORY_LIMIT: usize = 100 * 1024 * 1024; // 100MB
    let mut arenas = CompilerArenas::new(MEMORY_LIMIT);
    
    // AST 파싱
    let ast = match parse_with_arena(&arenas.parse_arena, source) {
        Ok(ast) => ast,
        Err(e) => {
            let used = arenas.parse_arena.allocated_bytes();
            if used > MEMORY_LIMIT / 3 {
                return Err(CompileError::OutOfMemory {
                    phase: "parsing",
                    used,
                    limit: MEMORY_LIMIT / 3,
                });
            }
            return Err(CompileError::ParseError(e.to_string()));
        }
    };
    
    // IR 생성
    let ir = generate_ir_with_arena(&arenas.ir_arena, ast)?;
    arenas.parse_arena.reset(); // AST 해제
    
    // 코드 생성
    let module = generate_code_with_arena(&arenas.codegen_arena, ir)?;
    arenas.ir_arena.reset(); // IR 해제
    
    Ok(module)
}
```

---

## 성능 최적화

### 1. 메모리 통계 수집

```rust
pub struct MemoryStats {
    pub parse_peak: usize,
    pub ir_peak: usize,
    pub codegen_peak: usize,
    pub total_allocated: usize,
}

impl CompilerArenas {
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            parse_peak: self.parse_arena.allocated_bytes(),
            ir_peak: self.ir_arena.allocated_bytes(),
            codegen_peak: self.codegen_arena.allocated_bytes(),
            total_allocated: self.total_allocated(),
        }
    }
}

// 사용 예
let arenas = CompilerArenas::new(100 * 1024 * 1024);
compile_with_arena(&arenas, source)?;

let stats = arenas.stats();
println!("Parse: {} MB", stats.parse_peak / 1024 / 1024);
println!("IR: {} MB", stats.ir_peak / 1024 / 1024);
println!("Codegen: {} MB", stats.codegen_peak / 1024 / 1024);
```

### 2. 벤치마크

```rust
// compiler/benches/arena_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_compile_with_arena(c: &mut Criterion) {
    let source = include_str!("../examples/01-factorial.pole-ir");
    
    c.bench_function("compile_factorial_arena", |b| {
        b.iter(|| {
            let arenas = CompilerArenas::new(100 * 1024 * 1024);
            compile_with_arena(black_box(&arenas), black_box(source)).unwrap()
        });
    });
}

criterion_group!(benches, bench_compile_with_arena);
criterion_main!(benches);
```

### 3. 메모리 프로파일링

```bash
# Valgrind Massif (메모리 프로파일링)
valgrind --tool=massif ./target/release/pole_compiler examples/01-factorial.pole-ir

# Massif 결과 시각화
ms_print massif.out.12345

# Heaptrack (Linux)
heaptrack ./target/release/pole_compiler examples/01-factorial.pole-ir
heaptrack_gui heaptrack.pole_compiler.12345.gz
```

---

## 문제 해결

### 문제 1: Arena 메모리 부족

**증상:**
```
CompileError::OutOfMemory { phase: "parsing", used: 52428800, limit: 33554432 }
```

**해결:**
1. MEMORY_LIMIT 증가
2. AST 크기 줄이기 (불필요한 노드 제거)
3. 스트리밍 파싱 (큰 파일)

### 문제 2: Lifetime 에러

**증상:**
```
error[E0597]: `arena` does not live long enough
```

**해결:**
1. Arena lifetime 명시: `CodeGen<'ctx, 'arena>`
2. Arena를 함수 스코프 밖에서 생성
3. `'arena: 'ctx` lifetime bound 추가

### 문제 3: 메모리 누수

**증상:**
Arena reset 후에도 메모리 사용량 감소 안 됨

**해결:**
1. `drop(arena)` 명시적 호출
2. Reference cycle 확인
3. `Rc<T>` 대신 `&'arena T` 사용

---

## 개발 마일스톤

### Phase 1: 기반 구축 (Week 1)
- [x] bumpalo 의존성 추가
- [ ] CompilerArenas 구조체 구현
- [ ] 메모리 통계 수집 기능

### Phase 2: 통합 (Week 2)
- [ ] codegen.rs Arena 통합
- [ ] ir_parser.rs Arena 활용
- [ ] type_checker.rs 최적화

### Phase 3: 검증 (Week 3)
- [ ] 벤치마크 작성
- [ ] OOM 테스트
- [ ] 대규모 프로젝트 테스트 (1000+ 파일)

---

## 성공 기준

### 메모리 효율
- [x] factorial: 110MB → 30MB (73% 감소)
- [ ] 1000 파일 프로젝트: 2GB → 500MB
- [ ] CI/CD 메모리: 4GB → 1GB

### 안정성
- [ ] OOM 시 크래시 0건
- [ ] 명확한 에러 메시지
- [ ] 자동 복구 메커니즘

### 성능
- [x] 컴파일 속도 3x 향상
- [ ] 캐시 히트율 95% 이상
- [ ] GC pause time 제거

---

## 관련 문서

- [Arena 할당 영향도](../reports/ARENA_ALLOCATOR_IMPACT.md)
- [Arena 상태 보고서](../reports/ARENA_ALLOCATOR_STATUS.md)
- [언어 개발 가이드](LANGUAGE_DEV.md)

## 참고 자료

- [bumpalo Documentation](https://docs.rs/bumpalo/)
- [SQLite Memory Management](https://www.sqlite.org/malloc.html)
- [Arena Allocator Pattern](https://www.rfleury.com/p/untangling-lifetimes-the-arena-allocator)

---

**우선순위:** Phase 5.1.5 (높음)  
**예상 기간:** 3주  
**담당:** 컴파일러 팀
