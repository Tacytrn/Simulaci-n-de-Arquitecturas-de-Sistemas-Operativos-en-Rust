// =====================================================================
// SIMULACIÓN DE ARQUITECTURA MICROKERNEL
// =====================================================================

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ServerId {
    Process,
    Memory,
    File,
}

impl ServerId {
    fn name(&self) -> &'static str {
        match self {
            ServerId::Process => "ProcessServer",
            ServerId::Memory => "MemoryServer",
            ServerId::File => "FileServer",
        }
    }
}

struct IpcMessage {
    from: String,
    payload: String,
    reply_to: Sender<String>,
}

struct Kernel {
    next_task_id: u32,
    tasks: Vec<(u32, String)>, 
    routes: std::collections::HashMap<&'static str, Sender<IpcMessage>>,
}

impl Kernel {
    fn new() -> Self {
        Kernel {
            next_task_id: 1,
            tasks: Vec::new(),
            routes: std::collections::HashMap::new(),
        }
    }

    fn register_server(&mut self, id: ServerId, sender: Sender<IpcMessage>) {
        self.routes.insert(id.name(), sender);
    }

    fn create_task(&mut self, name: &str) -> u32 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.tasks.push((id, name.to_string()));
        println!("[Kernel] Tarea creada -> id={id}, nombre='{name}'");
        id
    }


    fn send(&self, task_name: &str, target: ServerId, payload: &str) -> Result<String, String> {
        let sender = self
            .routes
            .get(target.name())
            .ok_or_else(|| format!("{} no está disponible", target.name()))?;

        let (reply_tx, reply_rx) = mpsc::channel();
        let msg = IpcMessage {
            from: task_name.to_string(),
            payload: payload.to_string(),
            reply_to: reply_tx,
        };

        println!(
            "[Kernel] Enrutando IPC: '{}' -> {} :: \"{}\"",
            task_name,
            target.name(),
            payload
        );

        sender
            .send(msg)
            .map_err(|_| format!("{} no responde (canal cerrado)", target.name()))?;

        reply_rx
            .recv_timeout(Duration::from_secs(2))
            .map_err(|_| format!("{} no respondió a tiempo", target.name()))
    }
}

trait Server {
    fn run(rx: Receiver<IpcMessage>);
}

struct ProcessServer;
impl Server for ProcessServer {
    fn run(rx: Receiver<IpcMessage>) {
        let mut states: std::collections::HashMap<String, &str> = std::collections::HashMap::new();
        for msg in rx {
            let respuesta = format!("estado('{}') = listo", msg.from);
            states.insert(msg.from.clone(), "listo");
            println!("  [ProcessServer] procesado pedido de '{}': {}", msg.from, msg.payload);
            let _ = msg.reply_to.send(respuesta);
        }
    }
}

// ---------------- MemoryServer ----------------
struct MemoryServer;
impl Server for MemoryServer {
    fn run(rx: Receiver<IpcMessage>) {
        let mut next_addr: u64 = 0x1000;
        for msg in rx {
            let addr = next_addr;
            next_addr += 0x100;
            println!(
                "  [MemoryServer] asignando memoria a '{}' -> pedido: {}",
                msg.from, msg.payload
            );
            let _ = msg.reply_to.send(format!("addr=0x{:X}", addr));
        }
    }
}

struct FileServer;
impl Server for FileServer {
    fn run(rx: Receiver<IpcMessage>) {
        for msg in rx {
            if msg.payload.contains("archivo_prohibido") {
                println!("  [FileServer] *** FALLO SIMULADO al procesar '{}' ***", msg.payload);
                panic!("FileServer: acceso inválido, el servidor termina");
            }
            println!("  [FileServer] abriendo archivo para '{}': {}", msg.from, msg.payload);
            let _ = msg.reply_to.send(format!("fd=OK({})", msg.payload));
        }
    }
}

fn spawn_server<S: Server>(_marker: S) -> Sender<IpcMessage> {
    let (tx, rx) = mpsc::channel::<IpcMessage>();
    thread::spawn(move || {
        S::run(rx);
    });
    tx
}

fn main() {
    println!("=== Simulación de arquitectura MICROKERNEL ===\n");

    let mut kernel = Kernel::new();

    let process_tx = spawn_server(ProcessServer);
    let memory_tx = spawn_server(MemoryServer);
    let file_tx = spawn_server(FileServer);

    kernel.register_server(ServerId::Process, process_tx);
    kernel.register_server(ServerId::Memory, memory_tx);
    kernel.register_server(ServerId::File, file_tx);

    let task_id = kernel.create_task("app_editor_texto");
    let task_name = format!("task-{task_id}");

    println!();

    match kernel.send(&task_name, ServerId::Process, "registrar_inicio") {
        Ok(r) => println!("[App] Respuesta de ProcessServer: {r}\n"),
        Err(e) => println!("[App] Error hablando con ProcessServer: {e}\n"),
    }

    match kernel.send(&task_name, ServerId::Memory, "reservar 4KB") {
        Ok(r) => println!("[App] Respuesta de MemoryServer: {r}\n"),
        Err(e) => println!("[App] Error hablando con MemoryServer: {e}\n"),
    }

    match kernel.send(&task_name, ServerId::File, "abrir documento.txt") {
        Ok(r) => println!("[App] Respuesta de FileServer: {r}\n"),
        Err(e) => println!("[App] Error hablando con FileServer: {e}\n"),
    }

    println!("[App] Ahora se solicita un archivo prohibido para forzar un fallo en FileServer...\n");
    match kernel.send(&task_name, ServerId::File, "abrir archivo_prohibido.sys") {
        Ok(r) => println!("[App] Respuesta de FileServer: {r}\n"),
        Err(e) => println!("[App] FileServer falló como se esperaba: {e}\n"),
    }

    thread::sleep(Duration::from_millis(200));

    println!("[App] A pesar del fallo de FileServer, el resto del sistema sigue operativo:");
    match kernel.send(&task_name, ServerId::Memory, "reservar 8KB adicionales") {
        Ok(r) => println!("[App] Respuesta de MemoryServer: {r}"),
        Err(e) => println!("[App] Error hablando con MemoryServer: {e}"),
    }

    println!("\n=== Fin de la simulación ===");
}
