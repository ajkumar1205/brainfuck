use crate::ir::Instruction;

pub struct FasmGenerator {
    instructions: Vec<String>,
    loop_count: usize,
    loop_stack: Vec<usize>, // new stack to track loop IDs
}

impl FasmGenerator {
    pub fn new() -> Self {
        Self {
            instructions: vec![
                "format ELF executable 3".to_string(),
                "entry start".to_string(),
                "".to_string(),
                "segment readable writeable".to_string(),
                "tape rb 30000".to_string(), // Reserve 30000 bytes for brainfuck tape
                "".to_string(),
                "segment readable executable".to_string(),
                "start:".to_string(),
                "    mov ebp, tape".to_string(), // Base pointer for tape
            ],
            loop_count: 0,
            loop_stack: Vec::new(),
        }
    }

    pub fn generate(&mut self, ir: &[Instruction]) -> String {
        for ins in ir {
            match ins {
                Instruction::Sum(val, ptr) => {
                    // Add/subtract value to/from memory at tape[ptr]
                    self.instructions.push(format!("    add byte [ebp + {}], {}", ptr, val));
                }
                Instruction::Print(ptr) => {
                    self.instructions.push(format!("; Print character at position {}", ptr));
                    self.instructions.push("    mov eax, 4         ; sys_write syscall number".to_string());
                    self.instructions.push("    mov ebx, 1         ; file descriptor (stdout)".to_string());
                    self.instructions.push(format!("    lea ecx, [ebp + {}]  ; pointer to character", ptr));
                    self.instructions.push("    mov edx, 1         ; number of bytes to write".to_string());
                    self.instructions.push("    int 0x80           ; invoke syscall".to_string());
                }
                Instruction::Read(ptr) => {
                    self.instructions.push(format!("; Read character into position {}", ptr));
                    self.instructions.push("    mov eax, 3         ; sys_read syscall number".to_string());
                    self.instructions.push("    mov ebx, 0         ; file descriptor (stdin)".to_string());
                    self.instructions.push(format!("    lea ecx, [ebp + {}]  ; buffer to read into", ptr));
                    self.instructions.push("    mov edx, 1         ; number of bytes to read".to_string());
                    self.instructions.push("    int 0x80           ; invoke syscall".to_string());
                }
                Instruction::LoopStart(ptr) => {
                    let loop_id = self.loop_count;
                    self.loop_count += 1;
                    self.loop_stack.push(loop_id);
                    self.instructions.push(format!("; Start of loop {}", loop_id));
                    self.instructions.push(format!("loop_start_{}:", loop_id));
                    self.instructions.push(format!("    cmp byte [ebp + {}], 0", ptr));
                    self.instructions.push(format!("    je loop_end_{}", loop_id));
                }
                Instruction::LoopEnd(ptr) => {
                    // Pop the matching loop id from the stack
                    if let Some(loop_id) = self.loop_stack.pop() {
                        self.instructions.push(format!("; End of loop {}", loop_id));
                        self.instructions.push(format!("    cmp byte [ebp + {}], 0", ptr));
                        self.instructions.push(format!("    jne loop_start_{}", loop_id));
                        self.instructions.push(format!("loop_end_{}:", loop_id));
                    } else {
                        // Should not happen if the parser checked the syntax correctly.
                        panic!("Unmatched loop end encountered during assembly generation.");
                    }
                }
            }
        }

        // Add exit syscall
        self.instructions.push("\n; Exit program".to_string());
        self.instructions.push("    mov eax, 1      ; sys_exit syscall number".to_string());
        self.instructions.push("    xor ebx, ebx    ; exit status 0".to_string());
        self.instructions.push("    int 0x80        ; invoke syscall".to_string());

        self.instructions.join("\n")
    }
}