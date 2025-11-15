# Rust Development Commands

## 🦀 Rust Commands

### สร้างโปรเจกต์ใหม่
```
cargo new project_name
```
```
cargo new --bin my_app
```
```
cargo new --lib my_lib
```

### รันโปรแกรม
```
cargo run
```
```
cargo run --release
```

### Build โปรแกรม  
```
cargo build
```
```
cargo build --release
```

### ตรวจสอบ syntax
```
cargo check
```

### ทดสอบ
```
cargo test
```

### ดูเอกสาร
```
cargo doc --open
```

### ติดตั้ง dependencies
```
cargo add serde
```
```
cargo add tokio --features full
```

### อัพเดท dependencies
```
cargo update
```

### สร้างไฟล์ executable
```
cargo install --path .
```
