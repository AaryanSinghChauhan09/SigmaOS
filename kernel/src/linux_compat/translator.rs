use super::SyscallContext;

/// OOP Syscall Translator Trait
pub trait SyscallTranslator {
    fn translate(&self, ctx: &mut SyscallContext) -> Result<i64, SyscallError>;
    fn dispatch(&self, ctx: &mut SyscallContext);
}

#[derive(Debug, Clone, Copy)]
pub enum SyscallError {
    Unimplemented,
    InvalidArgument,
}

pub struct LinuxSyscallTranslator;

impl SyscallTranslator for LinuxSyscallTranslator {
    fn translate(&self, ctx: &mut SyscallContext) -> Result<i64, SyscallError> {
        match ctx.nr {
            0 => self.handle_read(ctx),
            1 => self.handle_write(ctx),
            2 => self.handle_open(ctx),
            3 => self.handle_close(ctx),
            60 => self.handle_exit(ctx),
            _ => Err(SyscallError::Unimplemented),
        }
    }

    fn dispatch(&self, ctx: &mut SyscallContext) {
        match self.translate(ctx) {
            Ok(val) => ctx.ret = val,
            Err(SyscallError::Unimplemented) => {
                // -38 is ENOSYS in Linux
                ctx.ret = -38;
            }
            Err(SyscallError::InvalidArgument) => {
                // -22 is EINVAL in Linux
                ctx.ret = -22;
            }
        }
    }
}

impl LinuxSyscallTranslator {
    fn handle_read(&self, ctx: &SyscallContext) -> Result<i64, SyscallError> {
        let fd = ctx.args[0] as i32;
        let _buf = ctx.args[1] as *mut u8;
        let _len = ctx.args[2] as usize;
        
        if fd < 0 {
            return Err(SyscallError::InvalidArgument);
        }

        // Normally maps to native sigma_fs::read
        crate::log::info("linux_compat", "Intercepted Linux read syscall");
        Ok(0)
    }

    fn handle_write(&self, ctx: &SyscallContext) -> Result<i64, SyscallError> {
        let fd = ctx.args[0] as i32;
        let _buf = ctx.args[1] as *const u8;
        let len = ctx.args[2] as usize;

        if fd < 0 {
            return Err(SyscallError::InvalidArgument);
        }

        // Normally maps to native sigma_fs::write
        crate::log::info("linux_compat", "Intercepted Linux write syscall");
        Ok(len as i64)
    }

    fn handle_open(&self, _ctx: &SyscallContext) -> Result<i64, SyscallError> {
        crate::log::info("linux_compat", "Intercepted Linux open syscall");
        Ok(3) // Mock FD
    }

    fn handle_close(&self, _ctx: &SyscallContext) -> Result<i64, SyscallError> {
        crate::log::info("linux_compat", "Intercepted Linux close syscall");
        Ok(0)
    }

    fn handle_exit(&self, ctx: &SyscallContext) -> Result<i64, SyscallError> {
        let code = ctx.args[0] as i32;
        crate::log::info("linux_compat", "Intercepted Linux exit syscall");
        // Exit process normally
        Ok(code as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_translation_success() {
        let translator = LinuxSyscallTranslator;
        let mut ctx = SyscallContext::new(1, [1, 0, 10, 0, 0, 0]); // write(1, NULL, 10)
        translator.dispatch(&mut ctx);
        assert_eq!(ctx.ret, 10);
    }

    #[test]
    fn test_linux_translation_unimplemented() {
        let translator = LinuxSyscallTranslator;
        let mut ctx = SyscallContext::new(999, [0; 6]); // invalid syscall
        translator.dispatch(&mut ctx);
        assert_eq!(ctx.ret, -38); // ENOSYS
    }
}
