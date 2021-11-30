use std::mem::size_of;

use windows::{
    self,
    Win32::{
        Foundation::{HANDLE, HWND, INVALID_HANDLE_VALUE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32,
                TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
            },
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
        UI::WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId},
    },
};

type DWORD = u32;
type uintptr_t = usize;

fn _get_process_id() -> DWORD {
    let pid: DWORD = 0;
    unsafe {
        let hwnd: Option<HWND> = Some(FindWindowA("", "NieR:Automata"));
        hwnd.expect("Window not found");
        GetWindowThreadProcessId(hwnd, pid as *mut u32);
        let handle: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, false, pid);
        if handle == INVALID_HANDLE_VALUE {
            return 0;
        }
    };

    pid
}

fn _get_module_base_address(procId: DWORD, modName: char) -> uintptr_t {
    let base_address: uintptr_t = 0;
    unsafe {
        let hSnap: HANDLE =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, procId);
        if hSnap != INVALID_HANDLE_VALUE {
            let modEntry: MODULEENTRY32;
            modEntry.dwSize = size_of::<MODULEENTRY32>() as u32;
            if Module32First(hSnap, modEntry as *mut MODULEENTRY32).as_bool() {}
        }
    };
    base_address
}
