---
to: src/routes/mod.rs
inject: true
append: true
---
pub mod <%= h.changeCase.snake(name) %>;
