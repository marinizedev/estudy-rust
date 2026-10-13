# 🦀 estudy-rust

Repositório de estudos de **Rust** com foco em **Estruturas de Dados**, criado durante a disciplina **Implementação de Estruturas de Dados** do curso de **Análise e Desenvolvimento de Sistemas (ADS)** na UniFECAF.

Cada pasta `aulaX` corresponde a uma aula e traz um código-fonte comentado em português, com a explicação do conceito direto nos comentários.

---

## 🎯 Objetivo

- Aprender Rust na prática, implementando do zero as principais estruturas de dados e algoritmos.
- Entender os conceitos que tornam Rust diferente: **ownership**, **borrowing**, **`Option`**, **`Box`**, **traits**, **genéricos** e **concorrência segura**.
- Manter um registro organizado do aprendizado, aula por aula, como material de consulta e portfólio.

Todo o desenvolvimento foi feito no **GitHub Codespaces**, conforme recomendação do professor.

---

## 📂 Estrutura do repositório

```
estudy-rust/
├── Cargo.toml        # Workspace: reúne todas as aulas
├── Cargo.lock        # Versões exatas das dependências
├── .gitignore        # Ignora a pasta target/
├── README.md
├── aula2/            # Vetor próprio (MyVec) + testes
├── aula3/            # Pilha (Stack) genérica
├── aula4/            # Lista encadeada com Box
├── aula5/            # Árvore binária de busca e AVL
├── aula6/            # Grafos: DFS e BFS
├── aula7/            # Tabela hash com encadeamento
├── aula8/            # Concorrência com Arc e Mutex
├── aula9/            # Ordenação
│   ├── (raiz)        #   Bubble Sort
│   ├── aula9_1/      #   Quick Sort
│   └── aula9_2/      #   Merge Sort
├── aula10/           # Busca
│   ├── (raiz)        #   Busca binária
│   └── aula10_1/     #   Busca por interpolação
├── aula11/           # Iteradores e programação funcional
├── aula12/           # Gerenciamento de memória com Drop
└── aula13/           # Coleções da biblioteca padrão
```

Cada aula é um projeto Rust independente, com seu próprio `Cargo.toml` e sua pasta `src/`. O `Cargo.toml` da raiz transforma o repositório em um **workspace** do Cargo: todas as aulas compartilham o mesmo `Cargo.lock` e a mesma pasta `target/` (gerada na compilação e ignorada pelo Git).

---

## 📚 Conteúdo por aula

| Aula | Tema | Arquivo | Conceitos principais |
|------|------|---------|----------------------|
| 2 | Vetor próprio (`MyVec`) e testes | `aula2/src/lib.rs` | Projeto `--lib`, `pub struct`, `impl`, `#[cfg(test)]`, `#[test]`, `assert_eq!`, `Option` |
| 3 | Pilha (Stack) | `aula3/src/lib.rs` | Genéricos `<T>`, `push`, `pop`, `peek`, LIFO |
| 4 | Lista encadeada | `aula4/src/main.rs` | `Box`, `Option<Box<Node>>`, ownership |
| 5 | Árvore BST / AVL | `aula5/src/main.rs` | Recursão, rotações, fator de balanceamento |
| 6 | Grafos | `aula6/src/main.rs` | Lista de adjacências, DFS, BFS, `VecDeque` |
| 7 | Tabela hash | `aula7/src/main.rs` | Função de dispersão, baldes, tratamento de colisão |
| 8 | Concorrência | `aula8/src/main.rs` | `Arc`, `Mutex`, `thread::spawn` |
| 9 | Ordenação | `aula9/`, `aula9_1/`, `aula9_2/` | Bubble, Quick e Merge Sort, slices |
| 10 | Busca | `aula10/`, `aula10_1/` | Busca binária e por interpolação |
| 11 | Iteradores | `aula11/src/main.rs` | `filter`, `map`, `sum`, `find`, closures |
| 12 | Gerenciamento de memória | `aula12/src/main.rs` | Trait `Drop`, escopo, RAII |
| 13 | Coleções da std | `aula13/src/main.rs` | `HashMap`, `HashSet`, `BinaryHeap`, `VecDeque` |

---

## 🔍 Detalhando cada aula

### Aula 2 — Vetor próprio (`MyVec`) e testes
Primeiro contato com a diferença entre projeto **binário** e projeto **biblioteca**. Como esta aula usa **módulo de testes**, o projeto é criado como biblioteca:

```bash
cargo new --lib aula2
```

Com `--lib`, o código fica em `src/lib.rs` (e não em `src/main.rs`, que é o que o `cargo new aula2` gera).

O `MyVec` é uma struct pública que guarda os dados em um `Vec<i32>` interno e oferece três operações:
- `new`: cria um vetor vazio;
- `push`: adiciona um elemento ao final;
- `get`: devolve uma referência ao elemento do índice, ou `None` se o índice não existir.

O módulo `tests` só é compilado quando se roda `cargo test`. O teste verifica a inserção com `push` e a leitura com `get`, incluindo o caso de índice inexistente, que retorna `None`.

### Aula 3 — Pilha (`Stack<T>`)
Implementação de uma pilha **genérica** (funciona com qualquer tipo) sobre um `Vec<T>`. Operações: `new`, `push`, `pop`, `peek` e `is_empty`. Segue o princípio **LIFO** (último a entrar, primeiro a sair). Inclui teste automatizado.

### Aula 4 — Lista encadeada
Cada `Node` guarda um valor e um `Option<Box<Node>>` apontando para o próximo nó. Mostra como o `Box` aloca dados no heap e como o ownership é transferido ao encadear nós.

### Aula 5 — Árvore binária de busca (BST) e AVL
Árvore em que valores menores ficam à esquerda e maiores à direita. A inserção chama `balance()`, que usa **altura**, **fator de balanceamento** e **rotações** (esquerda e direita, simples e duplas) para manter a árvore AVL balanceada. Também há busca e impressão da árvore com `{:#?}`.

### Aula 6 — Grafos (DFS e BFS)
Grafo **não direcionado** representado por **lista de adjacências**. Duas travessias:
- **DFS** (busca em profundidade), recursiva;
- **BFS** (busca em largura), com fila (`VecDeque`).

### Aula 7 — Tabela hash
Tabela hash feita à mão: a função de dispersão soma os bytes da chave e aplica módulo pelo tamanho da tabela. Colisões são resolvidas com **encadeamento** (cada balde é um vetor de pares chave-valor). Operações: `insert` (atualiza se a chave já existe) e `get`.

### Aula 8 — Concorrência com `Arc` e `Mutex`
Várias threads modificam o mesmo vetor com segurança. O `Arc` permite compartilhar a posse entre threads e o `Mutex` garante acesso exclusivo por vez. Exemplo central da **concorrência sem data races** em Rust.

### Aula 9 — Algoritmos de ordenação
| Algoritmo | Pasta | Ideia | Complexidade (médio caso) |
|-----------|-------|-------|---------------------------|
| Bubble Sort | `aula9` | Troca vizinhos fora de ordem repetidamente | O(n²) |
| Quick Sort | `aula9/aula9_1` | Escolhe um pivô e particiona o array | O(n log n) |
| Merge Sort | `aula9/aula9_2` | Divide ao meio, ordena e intercala | O(n log n) |

### Aula 10 — Algoritmos de busca
- **Busca binária** (`aula10`): em array ordenado, divide o intervalo ao meio a cada passo. O(log n).
- **Busca por interpolação** (`aula10/aula10_1`): estima a posição do alvo pelo valor, funcionando melhor em dados ordenados e uniformemente distribuídos.

### Aula 11 — Iteradores
Estilo funcional em Rust: soma dos quadrados dos números pares com `iter().filter().map().sum()` e busca do primeiro valor maior que 5 com `find`.

### Aula 12 — Gerenciamento de memória com `Drop`
Implementa o trait `Drop` para executar código automaticamente quando um valor sai de escopo, liberando recursos sem *garbage collector*.

### Aula 13 — Coleções da biblioteca padrão
Tour pelas coleções prontas da std:
- `HashMap`: dicionário chave-valor;
- `HashSet`: conjunto de valores únicos;
- `BinaryHeap`: fila de prioridade (max-heap);
- `VecDeque`: fila com inserção e remoção nas duas pontas.

---

## ⚙️ Preparação do ambiente

Antes de rodar qualquer aula, é preciso ter o Rust instalado. Mesmo no GitHub Codespaces isso **não vem pronto**: o ambiente precisa ser preparado.

1. Acesse a documentação oficial: [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Lá fica o comando de instalação atualizado.
2. Execute o comando de instalação do `rustup` no terminal (no Linux/Codespaces):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Carregue o Cargo no terminal atual (ou feche e abra um novo terminal):

```bash
source "$HOME/.cargo/env"
```

4. Confirme se deu certo:

```bash
rustc --version
cargo --version
```

> O `rustup` instala o compilador (`rustc`) e o gerenciador de pacotes e projetos (`cargo`).

---

## ▶️ Como executar

Com o ambiente preparado, entre na pasta da aula e rode:

```bash
# Rodar uma aula (exemplo: grafos)
cd aula6
cargo run

# Rodar uma aula aninhada (exemplo: Quick Sort)
cd aula9/aula9_1
cargo run

# As aulas 2 e 3 são bibliotecas (lib.rs): não têm main, então use cargo test
cd aula2
cargo test

cd ../aula3
cargo test
```

Por ser um workspace, também é possível rodar qualquer aula a partir da raiz, indicando o nome do projeto com `-p`:

```bash
cargo run -p aula6
cargo test -p aula3
cargo build --workspace   # compila todas as aulas de uma vez
```

---

## 🛠️ Tecnologias

- Rust
- Cargo
- GitHub Codespaces

---

## 👩‍💻 Autora

**Marinize Santana**
Estudante de Análise e Desenvolvimento de Sistemas (UniFECAF)

- GitHub: [github.com/marinizedev](https://github.com/marinizedev)
- LinkedIn: [linkedin.com/in/marinize-santana-47bb2b372](https://linkedin.com/in/marinize-santana-47bb2b372)
