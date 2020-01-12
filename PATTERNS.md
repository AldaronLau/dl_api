# Patterns
DL API builds safe Rust APIs to C code using common (and uncommon) C patterns.
Patterns are denoted with attributes:
- [init_struct](Opaque Allocate Invalid / Initialize Later): Initialize pointer
- [init_buffer](Initialize At Least Part of an Uninitialized Buffer): Initialize buffer

## Index of Patterns
- [Opaque Allocate Invalid / Initialize Later](Opaque Allocate Invalid / Initialize Later)
- [Initialize At Least Part of an Uninitialized Buffer](Initialize At Least Part of an Uninitialized Buffer)

## Pattern Descriptions

### Opaque Allocate Invalid / Initialize Later
For the following C code:

```C
typedef struct Opaque {
    /* fields */
} Opaque;

size_t size_of_opaque(void) {
    return size_of(Opaque);
}

void initialize(Opaque* opaque) {
    /* initalize opaque data structure */
}

//// User code. ////
int main(int argc, char* argv[]) {
    Opaque* opaque = malloc(size_of_opaque());
    initialize(opaque);
}
```

Write SafeFFI MuON:

```muon
address: Opaque
  bytes: size_of_opaque
func: size_t size_of_opaque(void)
  mod: Main
func: void initialize(Opaque* opaque)
  mod: Main
  pat:
    init_struct: opaque
```

Then write rust code:

```rust
fn main() {
    let main = Main::new();
    let opaque: Opaque = main.initialize();
}
```

### Initialize At Least Part of an Uninitialized Buffer
For the following C code:

```C
// Returns length of initialized portion of buffer.
size_t fill_buffer(uint8_t* buffer, size_t length) {
    // Fill some or all of the buffer.
}

//// User code. ////
int main(int argc, char* argv[]) {
    size_t length = 64;
    uint8_t* buffer = malloc(length);
    fill_buffer(buffer, length);
    for(size_t i = 0; i < length; i++) {
        printf("%d", buffer[i]);
    }
}
```

Write SafeFFI MuON:

```muon
func: size_t fill_buffer(uint8_t* buffer, size_t length)
  mod: Main
  pat:
    init_buffer: buffer length .return
```

Then write rust code:

```rust
fn main() {
    let main = Main::new();
    let mut buffer = Vec::with_capacity(64);
    main.fill_buffer(&mut buffer);
    for i in buffer {
        println!("{}", i);
    }
}
```
