#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS AI-Native Model Process Runtime (IModelRuntime)
// Treats neural network models (LLMs, computer-vision, TTS) as first-class, scheduled OS processes

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Llm,
    Vision,
    Audio,
}

pub struct ModelProcess {
    pub process_id: u32,
    pub model_type: ModelType,
    pub weights_size_bytes: usize,
    pub priority: u8,
}

pub struct IModelRuntime {
    pub scheduled_models: Vec<ModelProcess>,
    pub total_allocated_gpu_vram_mb: u32,
}

impl IModelRuntime {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        IModelRuntime {
            scheduled_models: Vec::new(),
            total_allocated_gpu_vram_mb: 0,
        }
    }

    pub fn schedule_model_process(
        &mut self,
        pid: u32,
        model: ModelType,
        size: usize,
        priority: u8,
    ) -> Result<(), ()> {
        let vram_needed_mb = (size / (1024 * 1024)) as u32;
        if self.total_allocated_gpu_vram_mb + vram_needed_mb > 16384 {
            // 16GB VRAM limit
            return Err(());
        }

        self.scheduled_models.push(ModelProcess {
            process_id: pid,
            model_type: model,
            weights_size_bytes: size,
            priority,
        });
        self.total_allocated_gpu_vram_mb += vram_needed_mb;
        Ok(())
    }

    pub fn kill_model_process(&mut self, pid: u32) -> Result<(), ()> {
        if let Some(pos) = self
            .scheduled_models
            .iter()
            .position(|m| m.process_id == pid)
        {
            let m = self.scheduled_models.remove(pos);
            let vram_mb = (m.weights_size_bytes / (1024 * 1024)) as u32;
            self.total_allocated_gpu_vram_mb -= vram_mb;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_process_scheduler() {
        let mut runtime = IModelRuntime::new();
        // Allocate 2GB model
        assert!(runtime
            .schedule_model_process(101, ModelType::Llm, 2 * 1024 * 1024 * 1024, 10)
            .is_ok());
        assert_eq!(runtime.total_allocated_gpu_vram_mb, 2048);

        // Fail allocation if it exceeds 16GB limit
        assert!(runtime
            .schedule_model_process(102, ModelType::Vision, 15 * 1024 * 1024 * 1024, 5)
            .is_err());

        // Terminate process and free VRAM
        assert!(runtime.kill_model_process(101).is_ok());
        assert_eq!(runtime.total_allocated_gpu_vram_mb, 0);
    }
}
