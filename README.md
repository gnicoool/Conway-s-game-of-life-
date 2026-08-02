# Conway's Game of Life

Proyecto en Rust que implementa una simulacion visual del **Juego de la Vida de Conway** renderizar la evolucion de las celulas en tiempo real.

**Jackelyn Giron**

## Vista previa

![Demostracion del proyecto](conways-game-of-life.gif)

## Descripcion

Este proyecto genera una cuadrícula de celulas que evolucionan automaticamente siguiendo las reglas clasicas de Conway:

- Una celula viva sobrevive si tiene 2 o 3 vecinas vivas.
- Una celula muerta nace si tiene exactamente 3 vecinas vivas.
- En cualquier otro caso, la celula muere o permanece muerta.

La simulacion incluye varios patrones iniciales distribuidos en el tablero, como `blinker`, `glider`, `toad`, `loaf` y una figura personalizada llamada `flor`.

- Implementado en Rust.
- Visualizacion en ventana con `minifb`.
- Actualizacion cada 100 ms.
- Bordes conectados entre si mediante aritmetica toroidal.

## Requisitos

- Tener instalado [Rust](https://www.rust-lang.org/tools/install).
- Tener `cargo` disponible en la terminal.

## Como ejecutarlo

Clona el repositorio:

```bash
git clone https://github.com/gnicoool/Conway-s-game-of-life-.git
cd Conway-s-game-of-life-
```

Ejecuta el proyecto:

```bash
cargo run
```

## Controles

- `Esc`: cerrar la simulacion.

