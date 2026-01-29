/// Lista enlazada simple (Singly Linked List) implementada manualmente para aprender sobre asignacion en el Heap en Rust.
///
/// # Por que 'Box'?
/// En Rust, un 'None' contiene un puntero al siguiente nodo. Crea una estructura recursiva:
/// 'Node -> next -> Node -> next -> ...'
///
/// Para que Rust pueda conocer el tamaño del tipo en tiempo de compilacion, necesitamos que el "siguiente" nodo sea un puntero.
///
/// 'Box<T>' coloca 'T' en el Heap y en la estructura solo que el puntero de tamaño fijo.
///
/// # Por qué `Option`?
/// Para representar el final de la lista:
/// - `None` significa "no hay siguiente nodo".
/// - `Some(Box<Node<T>>)` significa "existe siguiente nodo".
///
/// # Examples
/// ```
/// use ds_03_linked_list::linked_list::LinkedList;
///
/// let mut list = LinkedList::new();
/// assert!(list.is_empty());
///
/// list.push_front(1);
/// list.push_front(2);
/// list.push_front(3);
///
/// assert_eq!(list.pop_front(), Some(3));
/// assert_eq!(list.pop_front(), Some(2));
/// assert_eq!(list.pop_front(), Some(1));
/// assert_eq!(list.pop_front(), None);
/// assert!(list.is_empty());
/// ```

#[derive(Debug, Default)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Crea una lista vacía.
    ///
    /// # Examples
    /// ```
    /// use ds_03_linked_list::linked_list::LinkedList;
    /// let list: LinkedList<i32> = LinkedList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Retorna la cantidad de elementos.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Retorna `true` si la lista está vacía.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Inserta un elemento al inicio de la lista.
    ///
    /// Complejidad: O(1)
    ///
    /// # Examples
    /// ```
    /// use ds_03_linked_list::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(10);
    /// list.push_front(20);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
    }

    /// Extrae el primer elemento de la lista y lo retorna.
    ///
    /// Complejidad: O(1)
    ///
    /// # Examples
    /// ```
    /// use ds_03_linked_list::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let Node { value, next } = *old_head;
            self.head = next;
            self.len -= 1;
            value
        })
    }

    /// Retorna `true` si la cabeza de la lista es `None`.
    ///
    /// Esto es útil como verificación explícita de que la lista quedó vacía.
    pub fn head_is_none(&self) -> bool {
        self.head.is_none()
    }

    /// Vacía completamente la lista removiendo todos los nodos.
    ///
    /// Nota: esto no necesita recorrer manualmente liberando memoria.
    /// Rust libera automáticamente al salir de scope (Drop) cuando ya no existen
    /// referencias a los nodos.
    pub fn clear(&mut self) {
        self.head = None;
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_and_pop_front_lifo_like_front_behavior() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn after_removing_all_nodes_head_is_none() {
        let mut list = LinkedList::new();

        list.push_front(10);
        list.push_front(20);

        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
        assert!(list.head_is_none());

        // Requisito: al eliminar todo, head debe ser None
        assert!(list.head_is_none());
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_on_empty_returns_none() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
        assert!(list.head_is_none());
    }

    #[test]
    fn clear_removes_all_nodes() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        list.clear();

        assert!(list.is_empty());
        assert!(list.head_is_none());
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn no_logical_memory_leaks_len_is_consistent() {
        // "Fuga Logica" aqui significa: len incosistente o nodos inaccesibles.
        // Si pop_front no re-enlaza head correctamente, len podria quedar mal.
        let mut list = LinkedList::new();

        for i in 0..100 {
            list.push_front(i);
        }
        assert_eq!(list.len(), 100);

        for _ in 0..100 {
            assert!(list.pop_front().is_some());
        }

        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
        assert!(list.head_is_none());
        assert_eq!(list.len(), 0);
    }
}