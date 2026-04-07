//! ternlang-fs: Triadic File System Abstraction.
//! 
//! Standard OS file systems treat a file as either 'Written' or 'Not Found'.
//! `ternlang-fs` implements a "Transactional Pend" state (State 0) natively.
//! A file can exist in an ambiguous state while multiple agents (MoE) 
//! deliberate on the validity of its content.

pub mod fs {
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy)]
    pub enum FileState {
        Committed = 1,
        Pending = 0,
        Purged = -1,
    }

    pub struct TernaryFile {
        pub path: String,
        pub data: Vec<u8>,
        pub state: FileState,
    }

    pub struct TernaryFS {
        storage: HashMap<String, TernaryFile>,
    }

    impl TernaryFS {
        pub fn new() -> Self {
            TernaryFS { storage: HashMap::new() }
        }

        /// Writes a file into the "Pending" state (State 0).
        /// It remains in logical equilibrium until `commit()` or `veto()` is called.
        pub fn write_pending(&mut self, path: &str, data: Vec<u8>) {
            println!("tern-fs: Writing '{}' to Pending (State 0) storage.", path);
            self.storage.insert(path.to_string(), TernaryFile {
                path: path.to_string(),
                data,
                state: FileState::Pending,
            });
        }

        pub fn commit(&mut self, path: &str) -> Result<(), &'static str> {
            if let Some(file) = self.storage.get_mut(path) {
                file.state = FileState::Committed;
                println!("tern-fs: File '{}' promoted to Committed (State 1).", path);
                Ok(())
            } else {
                Err("File not found.")
            }
        }

        pub fn read(&self, path: &str) -> Result<&Vec<u8>, &'static str> {
            if let Some(file) = self.storage.get(path) {
                match file.state {
                    FileState::Committed => Ok(&file.data),
                    FileState::Pending => Err("REJECTED: Attempted to read from State 0 (Pending) file. Hardware lock engaged."),
                    FileState::Purged => Err("VETO: File has been purged by security audit."),
                }
            } else {
                Err("File not found.")
            }
        }
    }
}
