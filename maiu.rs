// =====================================================================
// SIMULACIÓN DE ARQUITECTURA DE SISTEMA DE CAPAS (LAYERED SYSTEM)
// =====================================================================

mod hardware {
    pub struct HardwareLayer {
        next_address: u64,
    }

    impl HardwareLayer {
        pub fn new() -> Self {
            println!("[Capa 0: Hardware] Inicializando memoria física simulada");
            HardwareLayer { next_address: 0x1000 }
        }

        pub fn alloc_raw_block(&mut self, size_kb: u32) -> u64 {
            let addr = self.next_address;
            self.next_address += (size_kb as u64) * 0x10;
            println!(
                "  [Capa 0: Hardware] Bloque físico entregado: 0x{:X} ({} KB)",
                addr, size_kb
            );
            addr
        }
    }
}


mod memory {
    use crate::hardware::HardwareLayer;

    pub struct MemoryLayer {
        hardware: HardwareLayer,         
        regions: Vec<(String, u64, u32)>, 
    }

    impl MemoryLayer {
        pub fn new() -> Self {
            println!("[Capa 1: Memoria] Inicializando gestor de memoria");
            MemoryLayer {
                hardware: HardwareLayer::new(),
                regions: Vec::new(),
            }
        }

        pub fn allocate(&mut self, owner: &str, size_kb: u32) -> u64 {
            println!(
                "  [Capa 1: Memoria] '{}' solicita {} KB -> pide bloque a Capa 0",
                owner, size_kb
            );
            let addr = self.hardware.alloc_raw_block(size_kb);
            self.regions.push((owner.to_string(), addr, size_kb));
            addr
        }

        pub fn total_reservado_kb(&self) -> u32 {
            self.regions.iter().map(|(_, _, size)| *size).sum()
        }
    }
}

mod process {
    use crate::memory::MemoryLayer;

    pub struct ProcessLayer {
        memory: MemoryLayer, 
        next_pid: u32,
        table: Vec<(u32, String, u64)>, 
    }

    impl ProcessLayer {
        pub fn new() -> Self {
            println!("[Capa 2: Procesos] Inicializando gestor de procesos");
            ProcessLayer {
                memory: MemoryLayer::new(),
                next_pid: 1,
                table: Vec::new(),
            }
        }

        pub fn create_process(&mut self, name: &str) -> u32 {
            let pid = self.next_pid;
            self.next_pid += 1;

            println!(
                "  [Capa 2: Procesos] Creando proceso '{}' (pid={}) -> pide memoria a Capa 1",
                name, pid
            );
            let pcb_addr = self.memory.allocate(name, 4);

            self.table.push((pid, name.to_string(), pcb_addr));
            println!(
                "  [Capa 2: Procesos] Proceso '{}' creado -> pid={}, pcb en 0x{:X}",
                name, pid, pcb_addr
            );
            pid
        }

        pub fn listar(&self) -> &Vec<(u32, String, u64)> {
            &self.table
        }
    }
}

mod filesystem {
    use crate::process::ProcessLayer;

    pub struct FileSystemLayer {
        process_mgr: ProcessLayer, 
        files: Vec<(String, String)>, 
    }

    impl FileSystemLayer {
        pub fn new() -> Self {
            println!("[Capa 3: Archivos] Inicializando sistema de archivos");
            FileSystemLayer {
                process_mgr: ProcessLayer::new(),
                files: Vec::new(),
            }
        }

        pub fn crear_proceso(&mut self, name: &str) -> u32 {
            self.process_mgr.create_process(name)
        }

    
        pub fn listar_procesos(&self) -> &Vec<(u32, String, u64)> {
            self.process_mgr.listar()
        }

        pub fn abrir_archivo(&mut self, pid_dueno: u32, filename: &str) -> Result<(), String> {
            let existe_proceso = self
                .process_mgr
                .listar()
                .iter()
                .any(|(pid, _, _)| *pid == pid_dueno);

            if !existe_proceso {
                return Err(format!(
                    "no existe el proceso con pid={} (Capa 3 no puede abrir un archivo para un proceso inexistente)",
                    pid_dueno
                ));
            }

            println!(
                "  [Capa 3: Archivos] Abriendo '{}' para pid={}",
                filename, pid_dueno
            );
            self.files.push((filename.to_string(), pid_dueno.to_string()));
            Ok(())
        }
    }
}

mod shell {
    use crate::filesystem::FileSystemLayer;

    pub struct Shell {
        fs: FileSystemLayer, 
    }

    impl Shell {
        pub fn new() -> Self {
            println!("[Capa 4: Shell] Inicializando intérprete de comandos\n");
            Shell {
                fs: FileSystemLayer::new(),
            }
        }

        pub fn ejecutar_comando(&mut self, comando: &str) {
            println!("\n[Capa 4: Shell] Usuario ejecuta: \"{}\"", comando);
            match comando {
                cmd if cmd.starts_with("run ") => {
                    let nombre = &cmd[4..];
                    let pid = self.fs.crear_proceso(nombre);
                    println!("[Capa 4: Shell] Proceso '{}' lanzado con pid={}", nombre, pid);
                }
                cmd if cmd.starts_with("open ") => {
                    let partes: Vec<&str> = cmd[5..].splitn(2, ' ').collect();
                    if partes.len() != 2 {
                        println!("[Capa 4: Shell] Uso: open <pid> <archivo>");
                        return;
                    }
                    let pid: u32 = match partes[0].parse() {
                        Ok(p) => p,
                        Err(_) => {
                            println!("[Capa 4: Shell] pid inválido: {}", partes[0]);
                            return;
                        }
                    };
                    match self.fs.abrir_archivo(pid, partes[1]) {
                        Ok(()) => println!(
                            "[Capa 4: Shell] Archivo '{}' abierto correctamente",
                            partes[1]
                        ),
                        Err(e) => println!("[Capa 4: Shell] Error: {}", e),
                    }
                }
                _ => println!("[Capa 4: Shell] Comando no reconocido: {}", comando),
            }
        }

        
        pub fn listar_procesos(&self) -> &Vec<(u32, String, u64)> {
            self.fs.listar_procesos()
        }
    }
}

use shell::Shell;

fn main() {
    println!("=== Simulación de arquitectura de SISTEMA DE CAPAS ===\n");

   
    let mut shell = Shell::new();

    
    shell.ejecutar_comando("run editor_texto");


    shell.ejecutar_comando("open 1 documento.txt");


    shell.ejecutar_comando("run navegador");
    shell.ejecutar_comando("open 2 index.html");

    shell.ejecutar_comando("open 99 fantasma.txt");

    println!("\n[Capa 4: Shell] Procesos activos (vía Capa 3 -> Capa 2):");
    for (pid, nombre, pcb) in shell.listar_procesos() {
        println!("  pid={} nombre='{}' pcb=0x{:X}", pid, nombre, pcb);
    }

    println!("\n=== Fin de la simulación ===");
}
