# Motor de Catálogo AVL en Rust 

Este repositorio contiene el código correspondiente al **Segundo Examen Parcial** de la materia **Estructuras de Datos II / Programación II**.

## Descripción del Proyecto
El sistema es un motor de búsqueda de alto rendimiento. Está implementado utilizando un **Árbol AVL** en Rust para garantizar tiempos de búsqueda, inserción y eliminación óptimos ($O(\log n)$) mediante el auto-balanceo.

### Fases Implementadas:
1. **Auditoría y Teoría:** Gestión de memoria en Rust (`Box`, `Option`, `take()`) y rotaciones AVL.
2. **Motor de Consulta:** Búsqueda eficiente utilizando referencias (`&Libro`) para evitar copias innecesarias en memoria.
3. **Mantenimiento (Eliminación):** Lógica para dar de baja nodos (hojas, un hijo, dos hijos mediante sucesor in-orden) y re-balanceo dinámico.
4. **Funcionalidad Extendida:** Búsqueda de libros por rango de ISBN.


### Developed by MR24075
