# 🟦 DS-05 — Doubly Linked List

# DS-05 — Doubly Linked List

## 📌 Descripción
Lista doblemente enlazada implementada con `Rc<RefCell<Node>>` y `Weak`.

## 🧱 Funcionalidades
- `push_front`
- `push_back`
- Recorrido bidireccional

## 🔐 Seguridad de Memoria
- `Rc` → propiedad compartida
- `RefCell` → interior mutability
- `Weak` → evita ciclos de referencia

## ▶️ Ejecución
```bash
cargo fmt
cargo clippy
cargo test

## 🧩 Cómo utilizarlo
use ds_05_doubly_linked_list::DoublyLinkedList;

let mut list = DoublyLinkedList::new();
list.push_front(1);
list.push_back(2);

assert_eq!(list.to_vec_forward(), vec![1,2]);

## ⚙️ Cómo funciona
Cada nodo mantiene:

next: Rc<RefCell<Node>>

prev: Weak<RefCell<Node>>

Esto permite recorrer la lista en ambos sentidos sin fugas de memoria.

Estado: Semana 02 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC