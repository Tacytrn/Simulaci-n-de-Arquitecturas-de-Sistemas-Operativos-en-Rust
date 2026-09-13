# Informe: Simulación de Arquitecturas de Sistemas Operativos

**Integrantes:**
- Barbara Hernández Riquelme
- Alejandro León
- Valentina

---

## 1. Primera arquitectura: Microkernel

El rasgo distintivo de una arquitectura **microkernel** es que el núcleo realiza la menor cantidad posible de funciones. El resto de los servicios, como el sistema de archivos, la memoria y los drivers, se encuentran separados y se comunican principalmente mediante **paso de mensajes**, en lugar de depender de llamadas directas entre todos los componentes.

Para representar esta arquitectura en nuestra simulación implementamos:

- Un **kernel mínimo**, encargado de crear tareas y entregar o enrutar mensajes.
- **Servidores independientes** que se ejecutan en threads separados, simulando procesos aislados en espacio de usuario.
- Comunicación mediante `mpsc::channel`, utilizada para simular un mecanismo de **IPC (Inter-Process Communication)** basado en mensajes.
- Un tipo **Aplicación**, que no se comunica directamente con los servidores, sino que envía solicitudes al kernel para que este las redirija al servicio correspondiente.

De esta manera, buscamos representar la idea principal de un microkernel: mantener un núcleo pequeño y separar los demás servicios del sistema en componentes independientes.

---

## 2. Segunda arquitectura: Sistema de Capas

En un **sistema de capas**, la organización es diferente a la del microkernel. En lugar de utilizar servidores separados, el sistema está estructurado mediante una serie de capas apiladas.

Cada capa puede utilizar únicamente los servicios proporcionados por la capa inmediatamente inferior. Esto significa que una capa no debería saltarse niveles ni realizar llamadas hacia capas superiores.

En nuestra simulación representamos esta organización utilizando **composición y encapsulamiento**, intentando que la propia estructura del programa refleje esta restricción.

La arquitectura utilizada es la siguiente:

```text
Capa 4: Shell / Aplicación
        ↓
Capa 3: Sistema de Archivos
        ↓
Capa 2: Gestor de Procesos
        ↓
Capa 1: Gestor de Memoria
        ↓
Capa 0: Hardware simulado
```

### Res