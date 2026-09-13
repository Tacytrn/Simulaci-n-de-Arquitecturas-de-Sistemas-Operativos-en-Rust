# Simulación de Arquitecturas de Sistemas Operativos en Rust

Hacemos este repositorio y trabajo con el objetivo de comparar y simular dos arquitecturas de SO en rust.
Las arquitecturas seleccionadas son:

* Sistema por capas
* Microkernel

Las simulaciones no corresponden a sistemas operativos reales ni interactúan directamente con hardware. Su propósito es representar, mediante programas de consola, estructuras y funciones, las características principales de cada arquitectura.

## 1. Sistema por capas

La primera simulación representa un sistema operativo organizado en cinco capas:

1. Hardware
2. Memoria
3. Procesos
4. Sistema de archivos
5. Shell

Cada capa utiliza los servicios proporcionados por la capa inmediatamente inferior.

Por ejemplo, cuando el usuario ejecuta un comando para iniciar un programa, el Shell solicita al sistema de archivos la creación del proceso. El gestor de procesos solicita memoria y, finalmente, el gestor de memoria solicita al hardware simulado un bloque de memoria.

De esta manera, una operación puede recorrer la siguiente estructura:

```text
Usuario
   ↓
Shell
   ↓
Sistema de archivos
   ↓
Gestor de procesos
   ↓
Gestor de memoria
   ↓
Hardware
```

La simulación permite crear procesos identificados mediante un PID, reservar memoria para ellos y asociar archivos a procesos existentes. También se incluye manejo de errores, por ejemplo, al intentar abrir un archivo utilizando un PID inexistente.

Los archivos utilizados en la simulación, como `documento.txt` o `index.html`, representan recursos del sistema de archivos y no corresponden necesariamente a archivos físicos almacenados en el computador.

## 2. Microkernel

La segunda simulación representa una arquitectura Microkernel.

En este modelo el kernel mantiene únicamente funciones básicas, mientras que distintos servicios del sistema se ejecutan de manera independiente.

La simulación implementa tres servidores:

* `ProcessServer`: representa la administración de procesos.
* `MemoryServer`: representa la administración de memoria.
* `FileServer`: representa el manejo de archivos.
La estructura general puede representarse como:

```text
                ProcessServer
                     ↑
                     │
Aplicación ↔ Kernel ↔ MemoryServer
                     │
                     ↓
                  FileServer
```

Cada servidor se ejecuta en un hilo independiente y el kernel se encarga de enrutar los mensajes hacia el servicio correspondiente.

La simulación también incluye un fallo intencional del FileServer. Cuando se solicita un archivo determinado, este servidor termina su ejecución. Posteriormente se demuestra que otros servicios, como MemoryServer, continúan funcionando.

Este comportamiento permite representar una característica importante de los sistemas basados en microkernel: el aislamiento entre servicios puede evitar que el fallo de un componente provoque la caída completa del sistema.

Objetivo de la comparación

Las dos implementaciones permiten observar diferencias estructurales entre ambas arquitecturas.

En el sistema por capas existe una organización jerárquica en la cual cada capa depende de los servicios de la capa inferior.

En el microkernel los servicios se encuentran más aislados y se comunican mediante mensajes utilizando el kernel como intermediario.

El proyecto busca utilizar estas simulaciones para analizar las ventajas y desventajas de ambas alternativas en términos de modularidad, aislamiento de fallos, comunicación y complejidad
