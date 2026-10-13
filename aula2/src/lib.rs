// aula2/src/lib.rs

pub struct MyVec {
    data: Vec<i32>,
}

impl MyVec {
    pub fn new() -> MyVec {
        MyVec { data: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_get() {
        let mut vec = MyVec::new();

        // Adiciona dois elementos.
        vec.push(10);
        vec.push(20);

        // verifica se os elementos foram inseridos corretamente.
        assert_eq!(vec.get(0), Some (&10));
        assert_eq!(vec.get(1), Some (&20));
        // Um índice que não existe deve retornar None.
        assert_eq!(vec.get(2), None);
    }
}