/*
    PRUEBA DE ESCRITORIO
    Inserción: [10, 20, 30, 5, 2, 25]

    1. Insertar 10, 20, 30 -> Rotación Izquierda en 10. Raíz: 20.
    2. Insertar 5, 2       -> Rotación Derecha en 10.
    3. Insertar 25         -> Balanceado.

    ESTADO FINAL DEL ÁRBOL:
          20
        /    \
       5      30
      / \    /
     2  10  25
*/

/*
    ANÁLISIS DE RUST (Por qué usar take())
    En Rust, no se puede dejar un campo de una estructura vacío (null) durante una
    reasignación. La función take() permite extraer el valor de un Option (dejando None
    en su lugar) para mover la propiedad del nodo a una variable temporal. Esto es
    indispensable en las rotaciones AVL para reorganizar los punteros sin violar
    las reglas de propiedad (ownership) del compilador.
*/


use std::io;
#[derive(Debug, Clone)]
struct Libro {
    isbn: u32,
    titulo: String,
}

struct Nodo {
    libro: Libro,
    // Option permite que el hijo sea nulo (None).
    // Box coloca el Nodo en el heap, permitiendo una estructura recursiva de tamaño conocido.
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(libro: Libro) -> Self {
        Nodo {
            libro,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    //Uso de as_ref(): Convierte la referencia `&Option<Box<Nodo>>` en un `Option<&Box<Nodo>>`.
    // Permite mirar  al valor dentro del Option
    // sin consumir el Option original ni tomar propiedad (ownership) del nodo.
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    // .take() extrae el valor de la Option dejando un None en su lugar.
    // Esto permite mover la propiedad (ownership) del nodo sin violar las reglas de Rust[cite: 12, 14].
    let mut x = y.izquierdo.take().expect("Hijo izquierdo ausente");
    // Reasignamos el hijo derecho de 'x' al hijo izquierdo de 'y'
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    // 'y' ahora pasa a ser el hijo derecho del nuevo nodo raíz 'x'
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Hijo derecho ausente");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

fn insertar(nodo_opt: Option<Box<Nodo>>, libro: Libro) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(libro)),
        Some(n) => n,
    };

    let isbn_nuevo = libro.isbn;

    if isbn_nuevo < nodo.libro.isbn {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), libro));
    } else if isbn_nuevo > nodo.libro.isbn {
        nodo.derecho = Some(insertar(nodo.derecho.take(), libro));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && isbn_nuevo < nodo.izquierdo.as_ref().unwrap().libro.isbn {
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo > nodo.derecho.as_ref().unwrap().libro.isbn {
        return rotar_izquierda(nodo);
    }
    if balance > 1 && isbn_nuevo > nodo.izquierdo.as_ref().unwrap().libro.isbn {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo < nodo.derecho.as_ref().unwrap().libro.isbn {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    nodo
}

fn imprimir(nodo: &Option<Box<Nodo>>, nivel: usize) {
    if let Some(n) = nodo {
        imprimir(&n.derecho, nivel + 1);
        println!(
            "{:indent$}[ISBN: {}] {}",
            "",
            n.libro.isbn,
            n.libro.titulo,
            indent = nivel * 4
        );
        imprimir(&n.izquierdo, nivel + 1);
    }
}

// Busca un libro por su ISBN y retorna una referencia al mismo si existe.
//haciendo un match directamente sobre la referencia del Option
fn buscar(nodo: &Option<Box<Nodo>>, isbn: u32) -> Option<&Libro> {
    match nodo {
        None => None, 
        Some(n) => {
            if isbn == n.libro.isbn {
                Some(&n.libro)
            } else if isbn < n.libro.isbn {
                buscar(&n.izquierdo, isbn)
            } else {
                buscar(&n.derecho, isbn)
            }
        }
    }
}


/// Encuentra el libro con el ISBN mínimo en un subárbol.
fn encontrar_minimo(nodo: &Box<Nodo>) -> &Libro {
    match nodo.izquierdo.as_ref() {
        None => &nodo.libro, 
        Some(izq) => encontrar_minimo(izq),
    }
}

// Elimina un libro por su ISBN y retorna la nueva raíz del subárbol modificado.
fn eliminar(nodo_opt: Option<Box<Nodo>>, isbn: u32) -> Option<Box<Nodo>> {
    let mut nodo = match nodo_opt {
        None => return None,
        Some(n) => n,
    };

    if isbn < nodo.libro.isbn {
        nodo.izquierdo = eliminar(nodo.izquierdo.take(), isbn);
    } else if isbn > nodo.libro.isbn {
        nodo.derecho = eliminar(nodo.derecho.take(), isbn);
    } else {        
        if nodo.izquierdo.is_none() {
            return nodo.derecho.take(); 
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo.take();
        }
        let isbn_sucesor = {
            let sucesor = encontrar_minimo(nodo.derecho.as_ref().unwrap());
            nodo.libro = sucesor.clone();
            sucesor.isbn
        };
        nodo.derecho = eliminar(nodo.derecho.take(), isbn_sucesor);
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 {
        if obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
            return Some(rotar_derecha(nodo));
        } else {
            let hijo_izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
            return Some(rotar_derecha(nodo)); 
        }
    }
    if balance < -1 {
        if obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
            return Some(rotar_izquierda(nodo));
        } else {
            let hijo_der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(rotar_derecha(hijo_der));
            return Some(rotar_izquierda(nodo)); 
        }
    }

    Some(nodo)
}


// Busca y retorna una lista de referencias a libros cuyo ISBN esté dentro del rango [min, max].
// La función auxiliar `buscar_rango_aux` realiza una búsqueda recursiva en el árbol, agregando libros al vector de resultados si su ISBN está dentro del rango especificado.
fn buscar_rango<'a>(nodo: &'a Option<Box<Nodo>>, min: u32, max: u32) -> Vec<&'a Libro> {
    let mut resultados = Vec::new();
    buscar_rango_aux(nodo, min, max, &mut resultados);
    resultados
}

fn buscar_rango_aux<'a>(
    nodo: &'a Option<Box<Nodo>>,
    min: u32,
    max: u32,
    resultados: &mut Vec<&'a Libro>,
) {
    if let Some(n) = nodo {
        if n.libro.isbn > min {
            buscar_rango_aux(&n.izquierdo, min, max, resultados);
        }
        if n.libro.isbn >= min && n.libro.isbn <= max {
            resultados.push(&n.libro);
        }
        if n.libro.isbn < max {
            buscar_rango_aux(&n.derecho, min, max, resultados);
        }
    }
}


fn main() {
    let mut raiz: Option<Box<Nodo>> = None;
    let datos = vec![
        (10, "El Quijote"),
        (20, "1984"),
        (30, "Hamlet"),
        (5, "Fahrenheit 451"),
        (2, "La Odisea"),
        (25, "El Principito"),
    ];

    println!("--- Sistema de Inventario de Librería (AVL) ---");
    for (isbn, titulo) in datos {
        let libro = Libro {
            isbn,
            titulo: titulo.to_string(),
        };
        raiz = Some(insertar(raiz.take(), libro));
    }

    imprimir(&raiz, 0);

    // --- ESPACIO PARA TUS PRUEBAS ---


    // -------------------------------------------------------------
    // Pruebas de búsqueda (FASE 2)
    // -------------------------------------------------------------
    /*println!("\n---BÚSQUEDA---");
    println!("(Ingrese '0' en cualquier momento para finalizar la búsqueda)");
    loop {
        println!("\nPor favor, ingrese el ISBN del libro que desea buscar:");
        
        let mut entrada = String::new();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada del usuario");
        match entrada.trim().parse::<u32>() {
            Ok(0) => {
                println!("Saliendo del motor de búsqueda...");
                break; 
            }            Ok(isbn_ingresado) => {
                match buscar(&raiz, isbn_ingresado) {
                    Some(libro) => println!("Éxito: Se encontró el ISBN {}: '{}'", isbn_ingresado, libro.titulo),
                    None => println!("El ISBN {} no existe en el catálogo.", isbn_ingresado),
                }
            }
            Err(_) => {
                println!("Entrada no válida. Por favor, ingrese '0' para salir.");
            }
        }
    }*/


    // -------------------------------------------------------------
    // FASE 3: Mantenimiento 
    // -------------------------------------------------------------
    /*println!("\n--- ELIMINACIÓN ---");
    println!("(Ingrese '0' para finalizar el mantenimiento)");

    loop {
        println!("\nÁrbol actual:");
        imprimir(&raiz, 0);
        println!("\nIngrese el ISBN del libro que desea eliminar:");

        let mut entrada_del = String::new();
        io::stdin().read_line(&mut entrada_del).expect("Error al leer");

        match entrada_del.trim().parse::<u32>() {
            Ok(0) => break,
            Ok(isbn_a_borrar) => {
                if buscar(&raiz, isbn_a_borrar).is_some() {
                    raiz = eliminar(raiz.take(), isbn_a_borrar);
                    println!("Libro con ISBN {} eliminado exitosamente.", isbn_a_borrar);
                } else {
                    println!("El ISBN {} no se encuentra en el catálogo.", isbn_a_borrar);
                }
            }
            Err(_) => println!("Por favor, ingrese un número válido."),
        }
    }
    println!("--- MANTENIMIENTO FINALIZADO ---");

*/

    // -------------------------------------------------------------
    // FASE 4: Funcionalidades Extendidas Búsqueda por Rango
    // -------------------------------------------------------------
    println!("\n--- BÚSQUEDA POR RANGO  ---");
    println!("(Ingrese '0' en el valor mínimo para finalizar)");

    loop {
        println!("\nIngrese el ISBN mínimo del rango (o '0' para salir):");
        let mut entrada_min = String::new();
        std::io::stdin().read_line(&mut entrada_min).expect("Error al leer");
        
        let min = match entrada_min.trim().parse::<u32>() {
            Ok(0) => {
                println!("Saliendo de la búsqueda por rango...");
                break;
            }
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingrese un número válido.");
                continue;
            }
        };

        println!("Ingrese el ISBN máximo del rango:");
        let mut entrada_max = String::new();
        std::io::stdin().read_line(&mut entrada_max).expect("Error al leer");
        
        let max = match entrada_max.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingrese un número válido para el máximo.");
                continue;
            }
        };

        if min > max {
            println!("Error: El valor mínimo no puede ser mayor que el máximo.");
            continue;
        }

        let libros_en_rango = buscar_rango(&raiz, min, max);

        if libros_en_rango.is_empty() {
            println!("No se encontraron libros en el rango [{} - {}].", min, max);
        } else {
            println!("Libros encontrados en el rango [{} - {}]:", min, max);
            for libro in libros_en_rango {
                println!("   - ISBN {}: {}", libro.isbn, libro.titulo);
            }
        }
    }
    
    println!("\n¡Programa finalizado con éxito! El sistema de la biblioteca Santa Ana está listo.");

}
