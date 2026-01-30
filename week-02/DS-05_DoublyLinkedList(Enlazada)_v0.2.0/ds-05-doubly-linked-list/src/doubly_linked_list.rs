use std::cell::RefCell;
use std::rc::{Rc, Weak};

type StrongLink<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: StrongLink<T>,
    prev: WeakLink<T>,
}

/// Doubly Linked List (lista doblemente enlazada) implementada con `Rc<RefCell<Node<T>>>`.
///
/// # ¿Por qué NO se puede usar `Box`?
/// En una lista doble, cada nodo necesita apuntar al siguiente y al anterior.
/// Con `Box<T>`, existe **un único dueño** del nodo. Eso choca con el modelo
/// doble (`next` y `prev`) porque ambos enlaces intentarían “poseer” nodos.
/// Rust no permite múltiples dueños con `Box`.
///
/// # ¿Por qué `Rc<RefCell<_>>`?
/// - `Rc` (Reference Counting) permite **propiedad compartida**: varios punteros
///   pueden referirse al mismo nodo.
/// - `RefCell` permite **Interior Mutability**: actualizar `next/prev` incluso
///   cuando el nodo está detrás de un `Rc` (mutación verificada en runtime).
///
/// # ¿Por qué `Weak` en `prev`?
/// Si `next` y `prev` fueran `Rc`, se formarían ciclos (`Rc` ↔ `Rc`) y el conteo
/// de referencias nunca llega a cero (fuga real). `Weak` rompe el ciclo: no
/// incrementa el conteo fuerte.
///
/// # Examples
/// ```
/// use ds_05_doubly_linked_list::doubly_linked_list::DoublyLinkedList;
///
/// let mut list = DoublyLinkedList::new();
/// list.push_front(2);
/// list.push_front(1);
/// list.push_back(3);
///
/// assert_eq!(list.to_vec_forward(), vec![1, 2, 3]);
/// assert_eq!(list.to_vec_backward(), vec![3, 2, 1]);
/// ```
#[derive(Debug, Default)]
pub struct DoublyLinkedList<T> {
    head: StrongLink<T>,
    tail: StrongLink<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Crea una lista vacía.
    ///
    /// # Examples
    /// ```
    /// use ds_05_doubly_linked_list::doubly_linked_list::DoublyLinkedList;
    /// let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Retorna `true` si la lista está vacía.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Retorna el tamaño de la lista.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Inserta un elemento al inicio (head). O(1)
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            None => {
                // lista vacía
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }

        // si ya había tail, se mantiene; si no había (lista vacía), ya la seteamos
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.tail.as_ref().map(Rc::downgrade),
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
                // head ya existe si había tail (lista no vacía), no se toca
            }
            None => {
                // lista vacía
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }

    /// Recorre de head → tail y retorna un `Vec<T>`.
    ///
    /// Requiere `T: Clone` para no mover valores fuera de los nodos.
    pub fn to_vec_forward(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut out = Vec::with_capacity(self.len);
        let mut cur = self.head.clone();

        while let Some(node) = cur {
            let n = node.borrow();
            out.push(n.value.clone());
            cur = n.next.clone();
        }

        out
    }

    /// Recorre de tail → head y retorna un `Vec<T>`.
    ///
    /// Requiere `T: Clone`.
    pub fn to_vec_backward(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut out = Vec::with_capacity(self.len);
        let mut cur = self.tail.clone();

        while let Some(node) = cur {
            let n = node.borrow();
            out.push(n.value.clone());
            cur = n.prev.as_ref().and_then(|w| w.upgrade());
        }

        out
    }

    /// Limpia la lista. O(1)
    ///
    /// Con `Weak` en `prev`, no hay ciclos fuertes, así que con soltar head/tail
    /// basta para que Rust libere memoria automáticamente.
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[test]
    fn forward_and_backward_traversal() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.to_vec_forward(), vec![1, 2, 3, 4]);
        assert_eq!(list.to_vec_backward(), vec![4, 3, 2, 1]);
    }

    #[test]
    fn can_traverse_from_tail_to_head_without_borrow_errors() {
        let mut list = DoublyLinkedList::new();
        for i in 0..20 {
            if i % 2 == 0 {
                list.push_front(i);
            } else {
                list.push_back(i);
            }
        }

        let backward = list.to_vec_backward();
        assert_eq!(backward.len(), list.len());

        // si compila y corre sin panic, cumple el requisito
        assert!(!backward.is_empty());
    }

    #[test]
    fn clear_empties_list() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.to_vec_forward(), Vec::<i32>::new());
        assert_eq!(list.to_vec_backward(), Vec::<i32>::new());
    }

    #[test]
    fn len_is_consistent_under_load() {
        let mut list = DoublyLinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }
        assert_eq!(list.len(), 100);

        //no tenemos pop en el requisito; igual verificamos recorridos consistentes
        assert_eq!(list.to_vec_forward().len(), 100);
        assert_eq!(list.to_vec_backward().len(), 100);
    }
}
