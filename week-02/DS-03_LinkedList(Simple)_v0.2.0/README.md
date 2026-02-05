# 🟦 DS-03 — Linked List Simple

```markdown
# DS-03 — Linked List Simple

## 📌 Descripción
Implementación manual de una **Lista Enlazada Simple** para comprender el uso del **Heap** en Rust mediante `Box<T>` y `Option`.

## 🧱 Funcionalidades
- `push_front`
- `pop_front`
- `clear`

## 🧠 Conceptos Clave
- `Box<T>` permite estructuras recursivas
- `Option` marca el final de la lista
- No hay fugas de memoria lógicas

## 🧪 Pruebas
- Eliminación total de nodos
- Validación de lista vacía
- Consistencia de longitud

## ▶️ Ejecución
```bash
cargo fmt
cargo clippy
cargo test

🧩 Cómo utilizarlo
use ds_03_linked_list::LinkedList;

let mut list = LinkedList::new();
list.push_front(1);
list.push_front(2);

assert_eq!(list.pop_front(), Some(2));

⚙️ Cómo funciona

Cada nodo contiene:

Un valor

Un Option<Box<Node>> al siguiente nodo

El uso de Box permite que cada nodo viva en el heap y que Rust conozca el tamaño del tipo.

Estado: Semana 02 (completado)
Rol: Asociado Junior de Software
Autor: Claudio Andrés Sanjinés Cuellar
Empresa: FortisCode Global LLC