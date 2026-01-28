/// Implementación de una pila (stack) con semántica LIFO (Last In, First Out).
/// respaldada internamente por 'std::vec::Vec<T>'.
///
/// # Diseño
/// - `push`: inserta al final del `Vec`.
/// - `pop`: extrae desde el final del `Vec`.
/// - `peek`: observa el último elemento sin extraerlo.
///
/// # Examples
/// ```
/// use ds_01_stack::stack::Stack;
///
/// let mut s = Stack::new();
/// s.push(10);
/// s.push(20);
///
/// assert_eq!(s.peek(), Some(&20));
/// assert_eq!(s.pop(), Some(20));
/// assert_eq!(s.pop(), Some(10));
/// assert_eq!(s.pop(), None);
/// ```

#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Crea una nueva pila vacia.
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let s: Stack<i32> = Stack::new();
    /// assert!(s.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Retorna la cantidad de elementos en la pila.
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let mut s = Stack::new();
    /// s.push("a");
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Indica si la pila está vacía.
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let mut s = Stack::new();
    /// assert!(s.is_empty());
    /// s.push(1);
    /// assert!(!s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Inserta un elemento en la cima de la pila.
    ///
    /// Usa "Vec::push" (método nativo).
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let mut s = Stack::new();
    /// s.push(42);
    /// assert_eq!(s.peek(), Some(&42));
    /// ```
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Remueve y retorna el elemento en la cima de la pila.
    ///
    /// Usa "Vec::pop" (método nativo).
    /// - Retorna "none" si la pila está vacía.
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let mut s = Stack::new();
    /// s.push(1);
    /// s.push(2);
    /// assert_eq!(s.pop(), Some(2));
    /// assert_eq!(s.pop(), Some(1));
    /// assert_eq!(s.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Retorna una referencia al elemento en la cima de la pila sin removerlo.
    ///
    /// - Retorna `None` si la pila está vacía.
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let mut s = Stack::new();
    /// assert_eq!(s.peek(), None);
    /// s.push("x");
    /// assert_eq!(s.peek(), Some(&"x"));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Vacía la pila removiendo todos sus elementos.
    ///
    /// # Examples
    /// ```
    /// use ds_01_stack::stack::Stack;
    /// let mut s = Stack::new();
    /// s.push(1);
    /// s.push(2);
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn lifo_order_is_correct() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn peek_does_not_remove_element() {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);

        assert_eq!(s.peek(), Some(&20));
        assert_eq!(s.len(), 2);
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.peek(), Some(&10));
    }

    #[test]
    fn pop_on_empty_returns_none() {
        let mut s: Stack<i32> = Stack::new();
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn peek_on_empty_returns_none() {
        let s: Stack<&str> = Stack::new();
        assert_eq!(s.peek(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn work_with_multiple_types() {
        let mut s = Stack::new();
        s.push(String::from("a"));
        s.push(String::from("b"));

        assert_eq!(s.peek().map(|x| x.as_str()), Some("b"));
        assert_eq!(s.pop().as_deref(), Some("b"));
        assert_eq!(s.pop().as_deref(), Some("a"));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn clear_multiples_stack() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.clear();
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);
    }
}
