# 🦀 go_visibility_macro 🦀  
Because Rust's `pub` keyword was just too explicit 🔥  
Finally, a revolutionary crate that brings Go's brilliant visibility conventions to Rust — because who needs explicit keywords when you can just Capitalize Everything?

---

## ✨ Features ✨

### 🚀 Automatic pub-ification 🎩✨

- Struct name starts with uppercase? `pub` it!  
- Field name looks important? `pub` that too!  
- Function name screams "I'm public"? `pub` it like it's hot!  

---

### 😌 Go Developer PTSD Relief

- Miss Go's subtle visibility rules? Now you can write Rust like it's Go!  
- No more confusing `pub` spam — just **Capitalize and pray** 🙏  

---

### 🧙 Zero Runtime Overhead  
*(Because it's a proc macro, and we all know those never slow down compilation...)*

---

### 🤯 100% Guaranteed to Confuse Rust Purists

> "Explicit is better than implicit?"  
> Not anymore!

---

## 📦 Installation

Add this masterpiece to your `Cargo.toml`:

```toml
[dependencies]
go_visibility_macro = "0.1"  # Because semver is for cowards
```

---

## 🛠️ Usage

### 1. Structs? pub-ify them!

```rust
use go_visibility_macro::go_visibility;

#[go_visibility]
struct MyStruct {  // Automagically `pub`!
    PublicField: i32,  // Also `pub`!
    private_field: i32,  // Not `pub` (how sad)
}
```

---

### 2. Functions? pub-ify them harder!

```rust
#[go_visibility]
impl MyStruct {
    fn New() -> Self {  // `pub` because it's uppercase!
        Self { PublicField: 42, private_field: 69 }
    }

    fn get_secret(&self) -> i32 {  // Still private (loser)
        self.private_field
    }
}
```

---

### 3. Watch Rustaceans Scream in Horror 😱

```rust
// "Wait... why is this public?!" — Some Rust dev, probably
let s = MyStruct::New();
println!("{}", s.PublicField);  // Works!
// println!("{}", s.private_field);  // ERROR! (As it should be)
```

---

## 🤔 Why?

- "I miss Go" — You, probably  
- "`pub` is too many letters" — Also you  
- "I like making my IDE highlight random words" — Definitely you  

---

## ⚠️ Warning

🚨 **Not recommended for:**

- Teams that like "readability"  
- People who enjoy Rust's explicitness  

🚨 **Highly recommended for:**

- Proving that yes, you *can* make Rust look like Go  

---

## 🔮 Future Plans

- `#[go_error_handling]` — Replace `Result` with `if err != nil`  
- `#[go_gc]` — Just `malloc()` everywhere and pray  

---

## 🎉 Contributing

PRs welcome!  
(Just make sure your function names are **Capitalized Correctly** or they won't be `pub`!)

🌟 Star this repo if you love Go!  

---

🦀 *Rust is fun, but have you tried not being explicit?* 🦀
