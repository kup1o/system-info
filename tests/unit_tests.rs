#[cfg(test)]
mod tests {
    use system_info::*;

    #[test]
    fn test_get_cpu_info() {
        let result = get_cpu_info();
        assert!(result.starts_with("CPU:"));
    }

    #[test]
    fn test_get_gpu_info() {
        let result = get_gpu_info();
        assert!(result.starts_with("GPU:"));
    }

    #[test]
    fn test_get_display_info() {
        let result = get_display_info();
        assert!(result.starts_with("Display:"));
    }

    #[test]
    fn test_get_ram_info() {
        let result = get_ram_info();
        assert!(result.starts_with("RAM:"));
    }
}