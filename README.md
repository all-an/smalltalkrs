# SmalltalkRS - Complete Smalltalk Implementation in Rust

[![Coverage](https://img.shields.io/badge/coverage-93.9%25-brightgreen.svg)](coverage/tarpaulin-report.html)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![Tests](https://img.shields.io/badge/tests-20%20passing-brightgreen.svg)](#)

A complete implementation of the Smalltalk programming language with visual programming environment, written in Rust.

## Overview

This project aims to create a fully-featured Smalltalk implementation that includes:
- Pure object-oriented programming language
- Visual development environment with browsers and workspaces
- Live coding and debugging capabilities
- Image-based persistence with .st file format
- Complete class hierarchy and method lookup system
- Smalltalk-to-Rust transpilation capabilities
- Integration with Rust ecosystem libraries

## Running Tests and Coverage

### Running Tests

To run all tests:
```bash
cargo test
```

To run tests with output:
```bash
cargo test -- --nocapture
```

To run specific test modules:
```bash
cargo test core::object
cargo test core::small_integer
```

### Generating Test Coverage

#### Install tarpaulin (coverage tool):
```bash
cargo install cargo-tarpaulin
```

#### Generate coverage report:
```bash
cargo tarpaulin --out Html --output-dir coverage
```

#### Generate coverage with detailed output:
```bash
cargo tarpaulin --out Html --output-dir coverage --verbose
```

#### View coverage report:
Open `coverage/tarpaulin-report.html` in your browser

### Coverage Requirements
- **Target**: 100% test coverage for all modules
- **Minimum**: 95% coverage for any individual module
- **Critical paths**: 100% coverage for core object system and VM

### Testing Guidelines
- Every public function must have unit tests
- All error conditions must be tested
- Edge cases and boundary conditions must be covered
- Integration tests for component interactions
- Performance regression tests for critical paths

## Implementation Roadmap

### Phase 1: Core Language Foundation (Months 1-3)

#### 1.1 Object System Architecture
- [ ] Implement base `Object` class and metaclass system
- [ ] Design object memory management and garbage collection
- [ ] Create object identity and reference system
- [ ] Implement object instantiation and class hierarchy
- [ ] Build method dictionary and lookup mechanism

#### 1.2 Message Passing System
- [ ] Implement three types of messages:
  - [ ] Unary messages (no arguments)
  - [ ] Binary messages (single argument, operators)
  - [ ] Keyword messages (multiple arguments)
- [ ] Create message dispatcher and method resolution
- [ ] Implement method caching for performance
- [ ] Build primitive method support

#### 1.3 Basic Data Types and Collections
- [ ] Implement fundamental objects:
  - [ ] SmallInteger and LargeInteger
  - [ ] Float and Fraction
  - [ ] Character and String
  - [ ] Symbol (interned strings)
  - [ ] Boolean (True/False)
  - [ ] UndefinedObject (nil)
- [ ] Create collection hierarchy:
  - [ ] Array and ByteArray
  - [ ] OrderedCollection
  - [ ] Set and Dictionary
  - [ ] Interval

### Phase 2: Language Constructs (Months 4-5)

#### 2.1 Block Closures
- [ ] Implement block/closure object system
- [ ] Support lexical scoping and variable capture
- [ ] Create block evaluation mechanism
- [ ] Implement non-local returns

#### 2.2 Control Flow
- [ ] Implement control structures as message sends:
  - [ ] Conditional execution (ifTrue:ifFalse:)
  - [ ] Iteration (do:, collect:, select:, etc.)
  - [ ] While loops (whileTrue:, whileFalse:)
  - [ ] Exception handling (on:do:)

#### 2.3 Class and Method Definition
- [ ] Implement class creation and modification
- [ ] Support method compilation and installation
- [ ] Create instance and class variable systems
- [ ] Implement method categories and protocols

### Phase 3: Compiler and Virtual Machine (Months 6-8)

#### 3.1 Parser and Compiler
- [ ] Build Smalltalk syntax parser
- [ ] Implement AST generation
- [ ] Create bytecode compiler
- [ ] Support method decompilation for browsing

#### 3.2 Virtual Machine
- [ ] Design bytecode instruction set
- [ ] Implement bytecode interpreter
- [ ] Create execution contexts and stack frames
- [ ] Implement primitive operations
- [ ] Add JIT compilation support (optional)

#### 3.3 Memory Management
- [ ] Implement generational garbage collection
- [ ] Create object space management
- [ ] Support weak references
- [ ] Implement finalization

### Phase 4: Development Tools (Months 9-11)

#### 4.1 Class Browser
- [ ] Create hierarchical class browser
- [ ] Implement method browsing and editing
- [ ] Support protocol and category views
- [ ] Add search and navigation features

#### 4.2 Workspace and Evaluator
- [ ] Build interactive workspace
- [ ] Implement "do it", "print it", "inspect it"
- [ ] Create expression evaluation
- [ ] Support code snippet execution

#### 4.3 Debugger
- [ ] Implement stack frame inspection
- [ ] Create stepping and breakpoint support
- [ ] Support live method editing during debugging
- [ ] Implement exception handling in debugger

#### 4.4 Inspector and Object Browser
- [ ] Create object inspection tools
- [ ] Implement instance variable editing
- [ ] Support object reference tracking
- [ ] Add object dependency analysis

### Phase 5: Image System (Months 12-13)

#### 5.1 Image Persistence
- [ ] Implement image save/load mechanism
- [ ] Create object serialization system
- [ ] Support incremental image updates
- [ ] Implement image compaction

#### 5.2 Change Management
- [ ] Create change logging system
- [ ] Implement file-out/file-in capabilities
- [ ] Support source code management
- [ ] Add change set management

### Phase 6: Standard Library (Months 14-15)

#### 6.1 Core Classes
- [ ] Implement complete Number hierarchy
- [ ] Create comprehensive Collection classes
- [ ] Build Stream classes for I/O
- [ ] Implement Process and Semaphore for concurrency

#### 6.2 System Classes
- [ ] Create File and Directory handling
- [ ] Implement Date and Time classes
- [ ] Build Exception hierarchy
- [ ] Add Transcript and System classes

#### 6.3 Built-in Test Framework
- [ ] Implement TestCase base class for unit testing
- [ ] Create TestSuite for organizing test collections
- [ ] Build TestRunner with result reporting
- [ ] Add assertion methods (assert:, deny:, should:raise:)
- [ ] Support test discovery and automatic execution
- [ ] Create TestResult with success/failure tracking
- [ ] Implement test fixtures (setUp, tearDown)
- [ ] Add mock object support for testing
- [ ] Build performance testing capabilities
- [ ] Create visual test browser integrated with IDE

### Phase 7: Visual Environment (Months 16-18)

#### 7.1 Graphics Framework
- [ ] Implement basic graphics primitives
- [ ] Create drawing canvas and pen
- [ ] Support color and font management
- [ ] Build image and bitmap handling

#### 7.2 UI Framework (Morphic-style)
- [ ] Design morph-based UI system
- [ ] Implement event handling
- [ ] Create layout managers
- [ ] Support drag-and-drop

#### 7.3 Windows and Menus
- [ ] Build window management system
- [ ] Implement menu framework
- [ ] Create dialog and modal support
- [ ] Add keyboard and mouse handling

### Phase 8: Advanced Features (Months 19-20)

#### 8.1 Reflection and Metaprogramming
- [ ] Enhance reflection capabilities
- [ ] Support dynamic class modification
- [ ] Implement method wrappers
- [ ] Create object proxies

#### 8.2 Concurrency
- [ ] Implement cooperative multitasking
- [ ] Create process scheduler
- [ ] Support semaphores and mutexes
- [ ] Add shared queue implementations

#### 8.3 Network and I/O
- [ ] Implement socket support
- [ ] Create HTTP client/server
- [ ] Support file system operations
- [ ] Add serialization protocols

### Phase 9: Performance and Optimization (Months 21-22)

#### 9.1 Performance Tuning
- [ ] Optimize method lookup caching
- [ ] Improve garbage collection performance
- [ ] Add inline caching for frequent operations
- [ ] Implement primitive optimizations

#### 9.2 Profiling and Analysis
- [ ] Create performance profiling tools
- [ ] Implement memory usage analysis
- [ ] Add execution tracing
- [ ] Build benchmark suite

### Phase 10: File Format and Transpilation (Months 23-25)

#### 10.1 .st File Format Integration
- [ ] Design visual-to-.st conversion specification
- [ ] Implement serialization of visual programs to standard Smalltalk .st files
- [ ] Support roundtrip conversion: Visual ↔ .st Smalltalk code
- [ ] Create .st file validation and error reporting
- [ ] Build enhanced .st file editing tools with visual integration

#### 10.2 Smalltalk-to-Rust Transpilation
- [ ] Design AST-to-Rust translation layer
- [ ] Implement object system mapping to Rust structs/traits
- [ ] Create message passing translation to Rust method calls
- [ ] Support block closures as Rust closures
- [ ] Handle dynamic typing with Rust enums/generics
- [ ] Generate idiomatic Rust code with proper ownership

#### 10.3 Rust Library Integration
- [ ] Create FFI bridge for Rust crate integration
- [ ] Implement automatic Rust type binding generation
- [ ] Support async/await integration with Smalltalk processes
- [ ] Create Cargo.toml dependency management from Smalltalk
- [ ] Build wrapper generation for popular Rust libraries
- [ ] Support zero-cost abstractions where possible

### Phase 11: Testing and Documentation (Months 26-27)

#### 11.1 Comprehensive Testing
- [ ] Achieve 100% test coverage
- [ ] Create unit tests for all core classes
- [ ] Implement integration tests
- [ ] Build performance regression tests
- [ ] Add fuzzing tests for robustness
- [ ] Test .st file format roundtrip conversion
- [ ] Validate Smalltalk-to-Rust transpilation accuracy
- [ ] Add built-in test framework with comprehensive test suites
- [ ] Validate test framework integration with visual environment

#### 11.2 Documentation and Examples
- [ ] Write comprehensive API documentation
- [ ] Create tutorial and examples
- [ ] Build developer guides
- [ ] Add sample applications
- [ ] Document .st file format integration
- [ ] Create Rust integration examples

## Architecture Principles

### Pure Object-Oriented Design
- Everything is an object (numbers, classes, methods, stack frames)
- All computation happens through message sending
- No primitive types or special syntax constructs
- Uniform object model throughout the system

### Live Programming Environment
- Image-based development with persistent object state
- Interactive modification of running code
- Immediate feedback through workspaces
- Debugging with live code modification

### Reflective System
- Complete introspection capabilities
- Runtime modification of classes and methods
- Self-describing system structure
- Metaclass architecture for class behavior

### Performance Considerations
- Efficient method lookup with caching
- Generational garbage collection
- Bytecode interpretation with optional JIT
- Optimized primitive operations

### Interoperability Design
- Seamless .st file format for visual program persistence
- Bidirectional Smalltalk ↔ Rust transpilation
- Zero-overhead Rust library integration
- Preserve Smalltalk semantics in generated Rust code

## Testing Strategy

### Unit Testing
- Test every class and method
- Mock external dependencies
- Property-based testing for mathematical operations
- Boundary condition testing

### Integration Testing
- Test component interactions
- Verify message passing between objects
- Test class hierarchy inheritance
- Validate garbage collection behavior

### System Testing
- End-to-end workflow testing
- Performance benchmarking
- Memory leak detection
- Stress testing with large object graphs

### Visual Environment Testing
- UI component testing
- Event handling verification
- Graphics rendering tests
- User interaction simulation

## Development Guidelines

### Code Organization
```
src/
├── core/           # Core object system and VM
├── compiler/       # Parser and bytecode compiler
├── runtime/        # Runtime support and primitives
├── collections/    # Collection classes
├── tools/          # Development tools (browser, debugger)
├── graphics/       # Graphics and UI framework
├── image/          # Image persistence system
├── st_format/      # .st file format handling and visual integration
├── transpiler/     # Smalltalk-to-Rust transpilation
├── rust_bridge/    # Rust library integration layer
├── test_framework/ # Built-in Smalltalk testing framework
└── tests/          # Comprehensive test suite
```

### Quality Assurance
- 100% test coverage requirement
- Code review for all changes
- Continuous integration with full test suite
- Performance regression detection
- Memory safety verification

## Success Metrics

1. **Completeness**: Full Smalltalk-80 compatibility
2. **Performance**: Competitive with existing implementations
3. **Reliability**: Zero crashes, robust error handling
4. **Usability**: Intuitive visual programming environment
5. **Maintainability**: Clean, well-documented Rust code

## Contributing

This implementation follows the Smalltalk-80 specification as documented in the "Blue Book" (Smalltalk-80: The Language and its Implementation). All contributions should maintain the pure object-oriented nature and live programming capabilities that make Smalltalk unique.

### Key Requirements
- Preserve Smalltalk semantics in .st format and Rust transpilation
- Maintain 100% test coverage including roundtrip conversions
- Support idiomatic Rust code generation
- Enable seamless Rust ecosystem integration

### Development Workflow
1. Write failing tests first (TDD approach)
2. Implement minimal code to make tests pass
3. Refactor while maintaining test coverage
4. Run `cargo tarpaulin` to verify coverage targets
5. All PRs must maintain or improve coverage percentage

## License

[License to be determined]