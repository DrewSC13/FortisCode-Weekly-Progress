# DS-01 — Stack (Pila) LIFO

## 📌 Descripción
Implementación de una estructura de datos **Stack (Pila)** siguiendo el principio **LIFO (Last In, First Out)** usando `std::vec::Vec` en Rust.

La tarea busca reforzar:
- Uso de estructuras estándar
- Documentación con `///` y doctests
- Pruebas unitarias con `#[test]`

## 🧱 Funcionalidades
- `push`: inserta un elemento
- `pop`: remueve el último elemento
- `peek`: observa el último elemento sin removerlo
- `len` y `is_empty`

## 🧪 Pruebas
- Pruebas unitarias para flujo normal
- Pruebas con pila vacía
- Doctests validados con `cargo test`

## 📚 Documentación
Toda la API está documentada con comentarios `///` e incluye sección `# Examples` ejecutable.

## ▶️ Ejecución
```bash
cargo fmt
cargo clippy
cargo test

🧩 Cómo utilizarlo
use ds_01_stack::Stack;

let mut stack = Stack::new();
stack.push(10);
stack.push(20);

assert_eq!(stack.peek(), Some(&20));
assert_eq!(stack.pop(), Some(20));

⚙️ Cómo funciona
Internamente la pila usa un Vec<T>:

push delega en Vec::push

pop delega en Vec::pop

El último elemento del vector representa la cima de la pila

Esto garantiza complejidad O(1) en las operaciones principales.

Estado: Semana 02 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC