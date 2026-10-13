// aula3/src/lib.rs

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // Cria uma nova pilha.
    pub fn new() -> Stack<T> {
        Stack { elements: Vec::new() }
    }

    // Adiciona um elemento ao topo da pilha.
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    // Remove e retorna o elemento do topo da pilha, ou None se a pilha estiver vazia.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Retorna uma referência ao elemento do topo da pilha, ou None se a pilha estiver vazia.
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
    
    // Retorna true se a pilha estiver vazia.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();

        // Testa se a pilha está inicialmente vazia.
        assert!(stack.is_empty());

        // Adiciona elementos a pilha.
        stack.push(10);
        stack.push(20);

        // Verifica o elemento do topo da pilha.
        assert_eq!(stack.peek(), Some(&20));
        
        // Remove elementos da pilha e verifica.
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));

        // A pilha deve estar vazia novamente.
        assert!(stack.is_empty());
    }
}