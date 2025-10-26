use super::Module;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::ProcessStatus::{
    EnumProcesses, GetModuleBaseNameW, GetProcessMemoryInfo, K32EnumProcessModules,
    PROCESS_MEMORY_COUNTERS,
};
use windows::Win32::System::Threading::{
    OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE,
    PROCESS_VM_READ,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: u64,
    pub cpu_percent: f32,
    pub is_suspended: bool,
}

pub struct ProcessController {
    enabled: bool,
    cpu_threshold: f32,
    memory_threshold_mb: u64,
    monitored_processes: HashMap<u32, ProcessInfo>,
    whitelist: Vec<String>,
}

impl ProcessController {
    pub fn new() -> Self {
        Self {
            enabled: false,
            cpu_threshold: 80.0,
            memory_threshold_mb: 1024,
            monitored_processes: HashMap::new(),
            whitelist: vec![
                "winshaper.exe".to_string(),
                "explorer.exe".to_string(),
                "dwm.exe".to_string(),
                "csrss.exe".to_string(),
                "winlogon.exe".to_string(),
                "System".to_string(),
            ],
        }
    }

    pub fn get_running_processes(&mut self) -> anyhow::Result<Vec<ProcessInfo>> {
        let mut processes = Vec::new();
        let mut pids: [u32; 1024] = [0; 1024];
        let mut bytes_returned: u32 = 0;

        unsafe {
            if EnumProcesses(
                pids.as_mut_ptr(),
                (pids.len() * mem::size_of::<u32>()) as u32,
                &mut bytes_returned,
            )
            .is_ok()
            {
                let process_count = bytes_returned as usize / mem::size_of::<u32>();

                for &pid in &pids[..process_count] {
                    if pid == 0 {
                        continue;
                    }

                    if let Ok(process_info) = self.get_process_info(pid) {
                        processes.push(process_info);
                    }
                }
            }
        }

        self.monitored_processes.clear();
        for process in &processes {
            self.monitored_processes
                .insert(process.pid, process.clone());
        }

        Ok(processes)
    }

    unsafe fn get_process_info(&self, pid: u32) -> anyhow::Result<ProcessInfo> {
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            pid,
        )?;

        let name = self.get_process_name(process_handle)?;
        let memory_mb = self.get_process_memory(process_handle)?;

        CloseHandle(process_handle)?;

        Ok(ProcessInfo {
            pid,
            name,
            memory_mb,
            cpu_percent: 0.0, // TODO: Implement CPU usage tracking
            is_suspended: false,
        })
    }

    unsafe fn get_process_name(&self, process_handle: HANDLE) -> anyhow::Result<String> {
        let mut module_handle: windows::Win32::Foundation::HMODULE =
            windows::Win32::Foundation::HMODULE::default();
        let mut cb_needed: u32 = 0;

        if K32EnumProcessModules(
            process_handle,
            &mut module_handle,
            mem::size_of::<windows::Win32::Foundation::HMODULE>() as u32,
            &mut cb_needed,
        )
        .is_ok()
        {
            let mut name_buffer: [u16; 260] = [0; 260];
            let len = GetModuleBaseNameW(
                process_handle,
                module_handle,
                &mut name_buffer,
            );

            if len > 0 {
                let name = String::from_utf16_lossy(&name_buffer[..len as usize]);
                return Ok(name);
            }
        }

        Ok("Unknown".to_string())
    }

    unsafe fn get_process_memory(&self, process_handle: HANDLE) -> anyhow::Result<u64> {
        let mut mem_counters: PROCESS_MEMORY_COUNTERS = mem::zeroed();
        mem_counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        if GetProcessMemoryInfo(
            process_handle,
            &mut mem_counters,
            mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
        .is_ok()
        {
            // Convert from bytes to MB
            Ok(mem_counters.WorkingSetSize as u64 / (1024 * 1024))
        } else {
            Ok(0)
        }
    }

    pub fn suspend_process(&mut self, pid: u32) -> anyhow::Result<()> {
        if self.is_whitelisted_process(pid)? {
            return Err(anyhow::anyhow!("Cannot suspend whitelisted process"));
        }

        log::info!("Suspending process with PID: {}", pid);

        // TODO: Implement actual process suspension using NtSuspendProcess
        // This requires more complex Windows API calls
        
        if let Some(process_info) = self.monitored_processes.get_mut(&pid) {
            process_info.is_suspended = true;
        }

        Ok(())
    }

    pub fn resume_process(&mut self, pid: u32) -> anyhow::Result<()> {
        log::info!("Resuming process with PID: {}", pid);

        // TODO: Implement actual process resumption using NtResumeProcess
        
        if let Some(process_info) = self.monitored_processes.get_mut(&pid) {
            process_info.is_suspended = false;
        }

        Ok(())
    }

    pub fn terminate_process(&mut self, pid: u32) -> anyhow::Result<()> {
        if self.is_whitelisted_process(pid)? {
            return Err(anyhow::anyhow!("Cannot terminate whitelisted process"));
        }

        log::warn!("Terminating process with PID: {}", pid);

        unsafe {
            let process_handle = OpenProcess(PROCESS_TERMINATE, false, pid)?;
            TerminateProcess(process_handle, 1)?;
            CloseHandle(process_handle)?;
        }

        self.monitored_processes.remove(&pid);

        Ok(())
    }

    fn is_whitelisted_process(&self, pid: u32) -> anyhow::Result<bool> {
        if let Some(process_info) = self.monitored_processes.get(&pid) {
            Ok(self.whitelist.contains(&process_info.name))
        } else {
            Ok(false)
        }
    }

    pub fn check_thresholds(&mut self) -> anyhow::Result<Vec<ProcessInfo>> {
        let mut violations = Vec::new();

        for process in self.monitored_processes.values() {
            if process.cpu_percent > self.cpu_threshold
                || process.memory_mb > self.memory_threshold_mb
            {
                if !self.whitelist.contains(&process.name) {
                    violations.push(process.clone());
                }
            }
        }

        Ok(violations)
    }
}

impl Module for ProcessController {
    fn name(&self) -> &'static str {
        "process_controller"
    }
    
    fn description(&self) -> &'static str {
        "Monitor, suspend, or kill resource-heavy processes"
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn enable(&mut self) -> anyhow::Result<()> {
        log::info!("Enabling Process Controller module");
        self.enabled = true;
        
        // TODO: Start background monitoring thread
        // TODO: Setup performance counters
        
        Ok(())
    }
    
    fn disable(&mut self) -> anyhow::Result<()> {
        log::info!("Disabling Process Controller module");
        self.enabled = false;
        
        // TODO: Stop background monitoring
        // TODO: Cleanup resources
        
        Ok(())
    }
    
    fn get_settings(&self) -> HashMap<String, serde_json::Value> {
        let mut settings = HashMap::new();
        settings.insert("threshold_cpu".to_string(), serde_json::json!(self.cpu_threshold));
        settings.insert("threshold_ram".to_string(), serde_json::json!(self.memory_threshold_mb));
        settings.insert("whitelist".to_string(), serde_json::json!(self.whitelist));
        settings
    }
    
    fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        if let Some(cpu_threshold) = settings.get("threshold_cpu") {
            if let Some(value) = cpu_threshold.as_f64() {
                self.cpu_threshold = value as f32;
            }
        }
        
        if let Some(memory_threshold) = settings.get("threshold_ram") {
            if let Some(value) = memory_threshold.as_u64() {
                self.memory_threshold_mb = value;
            }
        }
        
        if let Some(whitelist) = settings.get("whitelist") {
            if let Some(list) = whitelist.as_array() {
                self.whitelist = list
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
        
        log::info!("Process Controller settings updated");
        Ok(())
    }
}